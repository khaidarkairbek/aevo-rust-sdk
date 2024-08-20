use std::sync::Arc;
use log::{info, debug, error};
use tokio_tungstenite::{connect_async, WebSocketStream, MaybeTlsStream, tungstenite::protocol::Message};
use tokio::{net::TcpStream, sync::{mpsc::UnboundedSender, Mutex}};  
use serde_derive::{Deserialize, Serialize};
use futures::{ stream::{SplitSink, SplitStream}, SinkExt, StreamExt };
use eyre::{eyre, Result}; 
use tokio_tungstenite::tungstenite;
use reqwest;
use chrono::prelude::*;
use crate::{env::ENV, ws_structs::*};

pub struct AevoClient {
    pub credentials : Option<ClientCredentials>, 
    pub writer: Arc<Mutex<Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>>,
    pub reader: Arc<Mutex<Option<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>>,
    pub client : reqwest::Client, 
    pub env : ENV,
}

pub struct ClientCredentials {
    pub signing_key : String, 
    pub wallet_address : String, 
    pub wallet_private_key : Option<String>, 
    pub api_key : String, 
    pub api_secret : String, 
}

pub const PRICE_DECIMALS: u32 = 6; 
pub const AMOUNT_DECIMALS: u32 = 6;

impl AevoClient {
    pub async fn new(
        credentials: Option<ClientCredentials>, 
        env : ENV
    ) -> Result<AevoClient> {

        let mut client = AevoClient {
            credentials : credentials, 
            writer : Arc::new(Mutex::new(None)), 
            reader : Arc::new(Mutex::new(None)),
            client : reqwest::Client::new(), 
            env: env
        }; 

        let ws_stream = client.open_connection().await?; 

        let (writer, reader) = ws_stream.split(); 

        client.writer = Arc::new(Mutex::new(Some(writer)));
        client.reader = Arc::new(Mutex::new(Some(reader)));

        Ok(client)
    }

    pub async fn open_connection(&self) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>>{
        info!("Opening Aevo websocket connection..."); 

        let ws_url = self.env.get_config().ws_url; 

        let (mut ws_stream, _) = connect_async(&ws_url).await?;
        

        match &self.credentials {
            Some(credentials) => {
                info!("Connecting to {}", ws_url); 

                let auth_request = WsRequest {
                    op : "auth".to_string(),
                    data : WsRequestData::AuthData { key: credentials.api_key.to_string(), secret: credentials.api_secret.to_string() },
                    id : Some(1)
                }; 

                let auth_msg = Message::from(serde_json::to_string(&auth_request)?); 

                debug!("The auth message: {:?}", auth_msg); 

                ws_stream.send(auth_msg).await?;
            }, 
            None => info!("Api key and/or wallet address not defined: No authentication is set in initial connection")
        }

        Ok(ws_stream)
    }

    pub async fn close_connection(&self) -> Result<()> {
        info!("Closing connection");

        let mut reader = self.reader.lock().await; 
        let mut writer = self.writer.lock().await;

        if let (Some(rx), Some(tx)) = (reader.take(), writer.take()) { 
            let mut ws_stream = tx.reunite(rx)?; 
            ws_stream.close(None).await?;
        }

        info!("Connection closed");

        Ok(())
    }

    pub async fn reconnect(&self) -> Result<()> {
        info!("Trying to reconnect Aevo websocket..."); 

        self.close_connection().await?;

        let ws_stream = self.open_connection().await?;

        let (writer, reader) = ws_stream.split(); 

        // Update the writer
        {
            let mut writer_guard = self.writer.lock().await;
            *writer_guard = Some(writer);
        }

        // Update the reader
        {
            let mut reader_guard = self.reader.lock().await;
            *reader_guard = Some(reader);
        }

        Ok(())
    }

    pub async fn read_messages(&self, tx : UnboundedSender<WsResponse>) -> Result<()> {
        loop {
            let msg = {
                let mut reader_guard = self.reader.lock().await; 
                match reader_guard.as_mut() {
                    Some(ws_stream) => {
                        ws_stream.next().await
                    }, 
                    None => {
                        return Err(eyre!("No connection is set"))
                    }
                }
            }; 

            match msg {
                Some(Ok(msg)) => {
                    match AevoClient::parse_response(msg) {
                        Ok(response) => {
                            match tx.send(response) {
                                Err(e) => error!("Problem sending data through unbounded channel: {}", e), 
                                _ => {}
                            };
                        }, 
                        Err(e) => error!("Problem parsing the response: {}", e)
                    }
 
                }, 
                Some(Err(e)) => {
                    match e {
                        tungstenite::Error::ConnectionClosed | tungstenite::Error::AlreadyClosed => {
                            info!("Aevo websocket connection close with error : {}", e);
                            
                            self.reconnect().await?; 
                        },
                        _ => {
                            error!("Message reading error : {}", e);
                        }
                    }
                    error!("Error reading message: {}", e);
                },
                None => {}
            }
        }
    }

    pub fn parse_response(msg : Message) -> Result<WsResponse> {
        let msg_txt = msg.into_text()?; 
        serde_json::from_str::<WsResponse>(&msg_txt).map_err(|e| eyre!("Error : {}; Message : {}", e, msg_txt))
    }

    pub async fn send (&self, data: &Message) -> Result<()>{
        let mut attempts = 0; 
        const MAX_ATTEMPTS: u8 = 2; 
        while attempts < MAX_ATTEMPTS {
            let result = {
                let mut writer_guard = self.writer.lock().await; 
                match writer_guard.as_mut() {
                    Some(ws_sink) => {
                        ws_sink.send(data.clone()).await
                    }, 
                    None => {
                        return Err(eyre!("Connection not established"))
                    }
                }
            }; 

            match result {
                Ok(_) => return Ok(()),
                Err(e) => {
                    match e  {
                        tungstenite::Error::ConnectionClosed | tungstenite::Error::AlreadyClosed => {
                            if attempts == 0 {
                                info!("Aevo websocket connection close with error : {}", e);
                                self.reconnect().await?;
                                attempts += 1; 
                                continue; 
                            } else {
                                return Err(eyre!("Failed to send message after reconnection attempt: {}", e))
                            }
                        }, 
                        _ => {
                            return Err(eyre!("Problem sending message: {}", e))
                        }
                    }
                }
            }
        }

        Err(eyre!("Failed to send message after maximum attempts"))
    }

    pub async fn subscribe_tickers(&self, asset: String) -> Result<()> {
        let request = WsRequest {
            op : "subscribe".to_string(),
            data : WsRequestData::ChannelData(vec![format!("ticker:{}:OPTION", asset)]), 
            id: None
        };

        let msg = Message::from(serde_json::to_string(&request)?); 
        self.send(&msg).await
    }

    pub async fn subscribe_book_ticker(&self, asset: String, instrument_type: String) -> Result<()> {
        let request = WsRequest {
            op : "subscribe".to_string(),
            data : WsRequestData::ChannelData(vec![format!("book-ticker:{}:{}", asset, instrument_type)]), 
            id: None
        };

        let msg = Message::from(serde_json::to_string(&request)?); 
        self.send(&msg).await
    }

    pub async fn subscribe_ticker(&self, channel: String) -> Result<()> {
        let request = WsRequest {
            op : "subscribe".to_string(),
            data : WsRequestData::ChannelData(vec![channel]), 
            id: None
        };

        let msg = Message::from(serde_json::to_string(&request)?); 
        self.send(&msg).await
    }

    pub async fn subscribe_orderbook(&self, instrument_name: String) -> Result<()> {
        let request = WsRequest {
            op : "subscribe".to_string(),
            data : WsRequestData::ChannelData(vec![format!("orderbook:{}", instrument_name)]), 
            id: None
        };

        let msg = Message::from(serde_json::to_string(&request)?); 
        self.send(&msg).await
    }

    pub async fn subscribe_trades(&self, instrument_name: String) -> Result<()> {
        let request = WsRequest {
            op : "subscribe".to_string(),
            data : WsRequestData::ChannelData(vec![format!("orderbook:{}", instrument_name)]), 
            id: None
        };

        let msg = Message::from(serde_json::to_string(&request)?); 
        self.send(&msg).await
    }

    pub async fn subscribe_index(&self, asset: String) -> Result<()> {
        let request = WsRequest {
            op : "subscribe".to_string(),
            data : WsRequestData::ChannelData(vec![format!("index:{}", asset)]), 
            id: None
        };

        let msg = Message::from(serde_json::to_string(&request)?); 
        self.send(&msg).await
    }

    pub async fn subscribe_orders(&self) -> Result<()> {
        let request = WsRequest {
            op : "subscribe".to_string(),
            data : WsRequestData::ChannelData(vec!["orders".to_string()]), 
            id: None
        };

        let msg = Message::from(serde_json::to_string(&request)?); 
        self.send(&msg).await
    }

    pub async fn subscribe_fills(&self) -> Result<()> {
        let request = WsRequest {
            op : "subscribe".to_string(),
            data : WsRequestData::ChannelData(vec!["fills".to_string()]), 
            id: None
        };

        let msg = Message::from(serde_json::to_string(&request)?); 
        self.send(&msg).await
    } 

    pub async fn create_order_ws (
        &self, 
        instrument_id: u64, 
        is_buy: bool, 
        limit_price: f64, 
        quantity: f64, 
        post_only: Option<bool>,
        mmp: Option<bool>,
    ) -> Result<(WsRequestData, String)>{
        let timestamp = Utc::now().timestamp(); 
        let (salt, signature, order_id) = self.sign_order(
            instrument_id, 
            is_buy, 
            Some(limit_price), 
            quantity, 
            timestamp
        ).await?; 

        let wallet_address= match &self.credentials {
            Some(ClientCredentials{wallet_address, ..}) => wallet_address.clone(), 
            None => return Err(eyre!("Order sign error: Wallet address not set"))
        };
        
        let payload: WsRequestData = WsRequestData::OrderData {
            maker : wallet_address, 
            is_buy: is_buy, 
            instrument: instrument_id.to_string(), 
            limit_price : (limit_price * 10_i32.pow(PRICE_DECIMALS) as f64).floor().to_string(), 
            amount : (quantity * 10_i32.pow(AMOUNT_DECIMALS) as f64).floor().to_string(), 
            salt : salt.to_string(), 
            signature : signature, 
            post_only : match post_only {
                Some(p) => p, 
                None => true
            },
            mmp : match mmp {
                Some(m) => m, 
                None => true, 
            },
            timestamp : timestamp.to_string(),
        }; 

        Ok((payload, order_id))
    }

    pub async fn create_order(
        &self, 
        instrument_id: u64,
        is_buy: bool, 
        limit_price: f64, 
        quantity: f64, 
        post_only: Option<bool>, 
        id: Option<u64>, 
        mmp: Option<bool>
    ) -> Result<String>{

        let (data, order_id) = self.create_order_ws(instrument_id, is_buy, limit_price, quantity, post_only, mmp).await?;


        let request = WsRequest {
            op : "create_order".to_string(), 
            data : data,
            id : id
        };

        info!("Order created: {:?}", request.data); 

        let msg = Message::from(serde_json::to_string(&request)?); 
        self.send(&msg).await?;


        Ok(order_id)
    }

    pub async fn edit_order (
        &self,
        order_id: String,
        instrument_id: u64,
        is_buy: bool,
        limit_price: f64,
        quantity: f64,
        id: Option<u64>,
        post_only: Option<bool>,
        mmp: Option<bool>,
    ) -> Result<String>{
        let timestamp = Utc::now().timestamp();
        let (salt, signature, new_order_id) = self.sign_order(
            instrument_id, 
            is_buy, 
            Some(limit_price), 
            quantity, 
            timestamp
        ).await?; 

        let wallet_address= match &self.credentials {
            Some(ClientCredentials{wallet_address, ..}) => wallet_address.clone(), 
            None => return Err(eyre!("Order sign error: Wallet address not set"))
        };

        let request = WsRequest {
            op : "edit_order".to_string(), 
            data : WsRequestData::EditOrderData { 
                order_id: order_id, 
                maker: wallet_address, 
                is_buy: is_buy, 
                instrument: instrument_id.to_string(), 
                limit_price: (limit_price * 10_i32.pow(6 as u32) as f64).floor().to_string(), 
                amount: (quantity * 10_i32.pow(6 as u32) as f64).floor().to_string(), 
                salt: salt.to_string(), 
                signature: signature, 
                post_only: match post_only {
                    Some(p) => p, 
                    None => true
                }, 
                mmp : match mmp {
                    Some(m) => m, 
                    None => true, 
                }, 
                timestamp: timestamp.to_string()
            }, 
            id: id
        }; 

        info!("Order edited: {:?}", request.data); 
        
        let msg = Message::from(serde_json::to_string(&request)?); 
        self.send(&msg).await?;

        Ok(new_order_id)
    }

    pub async fn cancel_order(&self, order_id : String) -> Result<()>{
        let request = WsRequest{
            op: "cancel_order".to_string(), 
            data: WsRequestData::CancelOrderData { order_id: order_id },
            id: None
        }; 

        info!("Order cancelled: {:?}", request.data); 

        let msg = Message::from(serde_json::to_string(&request)?); 
        self.send(&msg).await?;
        Ok(())
    }

    pub async fn cancel_all_orders(&self) -> Result<()> {
        let request = WsRequest{
            op: "cancel_all_orders".to_string(), 
            data: WsRequestData::CancelAllOrdersData { },
            id: None
        }; 

        info!("Cancelling all orders"); 
        let msg = Message::from(serde_json::to_string(&request)?); 
        self.send(&msg).await?;

        Ok(())
    }
}
