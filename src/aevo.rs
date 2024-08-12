use log::{info, debug, error};
use tokio_tungstenite::{connect_async, WebSocketStream, MaybeTlsStream, tungstenite::protocol::Message};
use tokio::{net::TcpStream, sync::mpsc::UnboundedSender};  
use serde_derive::{Deserialize, Serialize};
use futures::{ SinkExt, StreamExt };
use eyre::{eyre, Result}; 
use tokio_tungstenite::tungstenite;
use reqwest;
use chrono::prelude::*;
use crate::env::ENV;

#[derive(Serialize, Deserialize, Debug)]
pub struct WsRequest {
    op : String, 
    data : WsRequestData, 
    id : Option<u64>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum WsRequestData {
    AuthData {key : String, secret : String}, 
    ChannelData (Vec<String>),
    OrderData {
        maker : String, 
        is_buy : bool, 
        instrument: String, 
        limit_price: String, 
        amount : String, 
        salt : String, 
        signature : String, 
        post_only : bool, 
        mmp : bool,
        timestamp : String,
    }, 
    EditOrderData {
        order_id : String, 
        maker : String, 
        is_buy : bool, 
        instrument: String, 
        limit_price: String, 
        amount : String, 
        salt : String, 
        signature : String, 
        post_only : bool, 
        mmp : bool,
        timestamp : String,
    }, 
    CancelOrderData {
        order_id : String
    }, 
    CancelAllOrdersData {}
} 

pub struct AevoClient {
    pub signing_key : Option<String>, 
    pub wallet_address : Option<String>, 
    pub wallet_private_key : Option<String>, 
    pub api_key : Option<String>, 
    pub api_secret : Option<String>, 
    pub connection : Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    pub client : reqwest::Client, 
    pub env : ENV,
}

pub const PRICE_DECIMALS: u32 = 6; 
pub const AMOUNT_DECIMALS: u32 = 6;

impl AevoClient {
    pub async fn open_connection(&mut self)  -> Result<()>{
        info!("Opening Aevo websocket connection..."); 

        let ws_url = self.env.get_config().ws_url; 

        let connection_response = connect_async(&ws_url).await?; 
        debug!("Connection response: {:?}", connection_response.1); 
        self.connection = Some(connection_response.0); 

        match (&self.api_key, &self.api_secret, &self.wallet_address) {
            (Some(key), Some(secret), Some(_)) => {
                info!("Connecting to {}", ws_url); 

                let auth_request = WsRequest {
                    op : "auth".to_string(),
                    data : WsRequestData::AuthData { key: key.to_string(), secret: secret.to_string() },
                    id : Some(1)
                }; 

                let auth_msg = Message::from(serde_json::to_string(&auth_request)?); 

                debug!("The auth message: {:?}", auth_msg); 

                if let Some(connection) = &mut self.connection {
                    connection.send(auth_msg).await?;
                }
            }, 
            (_, _, _) => info!("Api key and/or wallet address not defined: No authentication is set in initial connection")
        }

        Ok(())
    }

    pub async fn close_connection(&mut self) -> Result<()> {
        info!("Closing connection"); 
        if let Some(mut connection) = self.connection.take() {
            connection.close(None).await?; 
        }
        info!("Connection closed");

        Ok(())
    }

    pub async fn reconnect(&mut self) -> Result<()> {
        info!("Trying to reconnect Aevo websocket..."); 

        self.close_connection().await?;
        self.open_connection().await 
    }

    pub async fn read_messages(&mut self, tx : UnboundedSender<Message>) -> Result<()> {
        loop {
            match &mut self.connection {
                Some(ws_stream) => {
                    tokio::select! {
                        message = ws_stream.next() => {
                            match message {
                                Some(Ok(msg)) => {
                                    match tx.send(msg) {
                                        Err(e) => error!("Problem sending data through unbounded channel: {}", e), 
                                        _ => {}
                                    }; 
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
                }, 
                None => {
                    return Err(eyre!("No connection is set"))
                }
            }
        }
    }

    pub async fn send (&mut self, data: &Message) -> Result<()>{
        let mut attempts = 0; 
        const MAX_ATTEMPTS: u8 = 2; 
        while attempts < MAX_ATTEMPTS {
            match &mut self.connection {
                Some(connection) => {
                    match connection.send(data.clone()).await {
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
                }, 
                None => {
                    return Err(eyre!("Connection not established"))
                }
            }
        }

        Err(eyre!("Failed to send message after maximum attempts"))
    }

    pub async fn subscribe_tickers(&mut self, asset: String) -> Result<()> {
        let request = WsRequest {
            op : "subscribe".to_string(),
            data : WsRequestData::ChannelData(vec![format!("ticker:{}:OPTION", asset)]), 
            id: None
        };

        let msg = Message::from(serde_json::to_string(&request)?); 
        self.send(&msg).await
    }

    pub async fn subscribe_ticker(&mut self, channel: String) -> Result<()> {
        let request = WsRequest {
            op : "subscribe".to_string(),
            data : WsRequestData::ChannelData(vec![channel]), 
            id: None
        };

        let msg = Message::from(serde_json::to_string(&request)?); 
        self.send(&msg).await
    }

    pub async fn subscribe_markprice(&mut self, asset: String) -> Result<()> {
        let request = WsRequest {
            op : "subscribe".to_string(),
            data : WsRequestData::ChannelData(vec![format!("markprice:{}:OPTION", asset)]), 
            id: None
        };

        let msg = Message::from(serde_json::to_string(&request)?); 
        self.send(&msg).await
    }

    pub async fn subscribe_orderbook(&mut self, instrument_name: String) -> Result<()> {
        let request = WsRequest {
            op : "subscribe".to_string(),
            data : WsRequestData::ChannelData(vec![format!("orderbook:{}", instrument_name)]), 
            id: None
        };

        let msg = Message::from(serde_json::to_string(&request)?); 
        self.send(&msg).await
    }

    pub async fn subscribe_trades(&mut self, instrument_name: String) -> Result<()> {
        let request = WsRequest {
            op : "subscribe".to_string(),
            data : WsRequestData::ChannelData(vec![format!("orderbook:{}", instrument_name)]), 
            id: None
        };

        let msg = Message::from(serde_json::to_string(&request)?); 
        self.send(&msg).await
    }

    pub async fn subscribe_index(&mut self, asset: String) -> Result<()> {
        let request = WsRequest {
            op : "subscribe".to_string(),
            data : WsRequestData::ChannelData(vec![format!("index:{}", asset)]), 
            id: None
        };

        let msg = Message::from(serde_json::to_string(&request)?); 
        self.send(&msg).await
    }

    pub async fn subscribe_orders(&mut self) -> Result<()> {
        let request = WsRequest {
            op : "subscribe".to_string(),
            data : WsRequestData::ChannelData(vec!["orders".to_string()]), 
            id: None
        };

        let msg = Message::from(serde_json::to_string(&request)?); 
        self.send(&msg).await
    }

    pub async fn subscribe_fills(&mut self) -> Result<()> {
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

        let wallet_address= match &self.wallet_address {
            Some(address) => address.clone(), 
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
        &mut self, 
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
        &mut self,
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

        let wallet_address= match &self.wallet_address {
            Some(address) => address.clone(), 
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

    pub async fn cancel_order(&mut self, order_id : String) -> Result<()>{
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

    pub async fn cancel_all_orders(&mut self) -> Result<()> {
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
