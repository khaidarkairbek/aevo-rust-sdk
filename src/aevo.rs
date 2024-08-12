use core::time;
use std::{collections::HashMap, str::FromStr};
use alloy::{consensus::Account, hex::{self, ToHexExt}, primitives::{address, bytes, keccak256, Address, Sign, Signature, I256, U256}, signers::{local::{LocalSigner, PrivateKeySigner}, Signer}, sol}; 
use log::{info, debug, error};
use tokio_tungstenite::{connect_async, WebSocketStream, MaybeTlsStream, tungstenite::protocol::Message};
use tokio::{net::TcpStream, sync::mpsc::UnboundedSender};  
use serde_derive::{Deserialize, Serialize};
use futures::{ sink::With, SinkExt, StreamExt };
use eyre::{eyre, Error, Result}; 
use tokio_tungstenite::tungstenite; 
use tokio::sync::mpsc::unbounded_channel; 
use reqwest::{self, header::HeaderMap}; 
use rand;
use alloy::sol_types::Eip712Domain;
use alloy::sol_types::SolStruct;
use chrono::prelude::*;


pub enum ENV {
    MAINNET, 
    TESTNET
}

impl ENV {
    pub fn get_config(&self) -> Config {
        match self {
            ENV::MAINNET => {
                Config {
                    rest_url : "https://api.aevo.xyz".to_string(), 
                    ws_url : "wss://ws.aevo.xyz".to_string(), 
                    signing_domain : SigningDomain {
                        name : "Aevo Mainnet".to_string(), 
                        version : "1".to_string(), 
                        chain_id : U256::from(1)
                    }
                }
            }, 
            ENV::TESTNET => {
                Config {
                    rest_url : "https://api-testnet.aevo.xyz".to_string(), 
                    ws_url : "wss://ws-testnet.aevo.xyz".to_string(), 
                    signing_domain : SigningDomain {
                        name : "Aevo Testnet".to_string(), 
                        version : "1".to_string(), 
                        chain_id : U256::from(11155111)
                    }
                }
            }
        }
    }

    pub fn get_addresses(&self) -> Addresses {
        match self {
            ENV::MAINNET => {
                Addresses {
                    l1_bridge : "0x4082C9647c098a6493fb499EaE63b5ce3259c574".to_string(), 
                    l1_usdc : "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string(), 
                    l2_withdraw_proxy : "0x4d44B9AbB13C80d2E376b7C5c982aa972239d845".to_string(),
                    l2_usdc : "0x643aaB1618c600229785A5E06E4b2d13946F7a1A".to_string()
                }
            }, 
            ENV::TESTNET => {
                Addresses {
                    l1_bridge : "0xb459023ECAf4ee7E55BEC136e592d2B7afF482E2".to_string(), 
                    l1_usdc : "0xcC3e3DBb31a7410e1dc5156593CdBFA0616BB309".to_string(), 
                    l2_withdraw_proxy : "0x870b65A0816B9e9A0dFCE08Fd18EFE20f245011f".to_string(),
                    l2_usdc : "0x52623B37Ff81c53567D6D16fd94638734cCDCf27".to_string()
                }
            }
        }
    }
}

pub struct Addresses {
    l1_bridge : String, 
    l1_usdc : String, 
    l2_withdraw_proxy : String, 
    l2_usdc : String
}

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

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum RestResponse {
    GetIndex { timestamp : String, price : String }, 
    GetMarkets (Vec<MarketInfo>), 
    DeleteOrder {order_id : String}, 
    GetAccount {
        account : String, 
        username : String, 
        account_type : String, 
        portfolio : bool, 
        equity : String, 
        balance : String, 
        credit : String, 
        credited : bool, 
        collaterals : Vec<CollateralAccountInfo>, 
        available_balance : String, 
        initial_margin : String, 
        maintenance_margin : String, 
        email_address : String, 
        in_liquidation : bool, 
        referral_bonus : f64, 
        has_been_referred : bool, 
        referrer : Option<String>, 
        intercom_hash : String, 
        permissions : Option<Vec<String>>,
        positions : Vec<String>, //should be position type
        signing_keys : Vec<SigningKeyInfo>, 
        api_keys : Vec<ApiKeyInfo>, 
        fee_structures : Vec<FeeStructureInfo>, 
        leverages : Vec<LeverageInfo>, 
        manual_mode : bool, 
        manual_withdrawals : Vec<ManualWithdrawalInfo>
    }, 
    GetPortfolio {
        balance : String, 
        pnl : String, 
        realized_pnl : String, 
        profit_factor : String, 
        win_rate : String, 
        sharpe_ratio : String, 
        greeks : Vec<PortfolioGreeks>, 
        user_margin : UsedMarginInfo
    },
    GetOrders (Vec<OrderInfo>), 
    DeleteOrdersAll {
        success : bool, 
        order_ids : Vec<String>
    },
    CreateOrder {
        order_id: String, 
        account: String, 
        instrument_id: String, 
        instrument_name: String, 
        instrument_type: String, 
        order_type: String, 
        side: String, 
        amount: String, 
        price: String, 
        avg_price: String, 
        filled: String, 
        order_status: String, 
        post_only: Option<bool>, 
        reduce_only: Option<bool>, 
        initial_margin: Option<String>, 
        option_type: Option<String>, 
        iv: Option<String>, 
        expiry: Option<String>,
        strike: Option<String>, 
        created_timestamp: Option<String>,
        timestamp: String, 
        system_type: String, 
        time_in_force: Option<String>, 
        stop: Option<String>, 
        trigger: Option<String>, 
        close_position: Option<bool>, 
        partial_position: Option<bool>, 
        isolated_margin: Option<String>, 
        parent_order_id: Option<String>, 
        self_trade_prevention: Option<String>
    }, 
    Withdraw{success : bool}

} 

#[derive(Serialize, Deserialize, Debug)]
pub struct RestWithdraw {
    account : String, 
    collateral : String, 
    to : String, 
    amount : String, 
    salt : String, 
    signature : String, 
    data : Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RestOrder {
    maker : String, 
    is_buy : bool, 
    instrument: String, 
    limit_price: String, 
    amount : String, 
    salt : String, 
    signature : String, 
    post_only : bool, 
    reduce_only : bool, 
    close_position : bool, 
    timestamp : String, 
    trigger : Option<String>, 
    stop : Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderInfo {
    order_id : String, 
    account : String, 
    instrument_id : String, 
    instrument_name : String, 
    instrument_type : String, 
    order_type : String, 
    side : String, // buy  or sell
    amount : String, 
    price : String, 
    avg_price : String, 
    filled : String, 
    order_status : String,
    post_only : Option<bool>, 
    reduce_only : Option<bool>, 
    initial_margin : Option<String>, 
    option_type : Option<String>, 
    iv : Option<String>, 
    expiry : Option<String>, 
    strike : Option<String>, 
    created_timestamp : Option<String>, 
    timestamp : String, 
    system_type : String, 
    time_in_force : Option<String>, 
    stop : Option<String>, 
    trigger : Option<String>, 
    close_position : Option<bool>, 
    partial_position : Option<bool>,
    isolated_margin : Option<String>, 
    parent_order_id : Option<String>,
    self_trade_prevention : Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsedMarginInfo {
    used : String, 
    balance : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ManualWithdrawalInfo {
    account : String, 
    amount : String, 
    chain_id : String, 
    collateral : String, 
    withdrawal_id : String, 
    to : String, 
    label : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CollateralAccountInfo {
    collateral_asset : String, 
    balance: String, 
    available_balance: String, 
    withdrawable_balance : String, 
    margin_value : String, 
    collateral_value : String, 
    collateral_yield_bearing : bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SigningKeyInfo {
    signing_key : String, 
    expiry : String, 
    created_timestamp : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiKeyInfo {
    api_key : String, 
    read_only : bool, 
    created_timestamp : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeStructureInfo {
    asset : String, 
    instrument_type : String, 
    taker_fee : String, 
    maker_fee : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LeverageInfo {
    instrument_id : String, 
    leverage : String, 
    margin_type : String  // Cross or Margin 
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum MarketInfo {
    Perp {
        instrument_id : String, 
        instrument_name: String, 
        instrument_type : String, 
        underlying_asset : String, 
        quote_asset : String, 
        price_step : String, 
        amount_step : String, 
        min_order_value : String, 
        max_order_value : String, 
        max_notional_value : String, 
        mark_price : String, 
        index_price : String, 
        is_active : bool, 
        max_leverage : String
    }, 
    Option {
        instrument_id : String, 
        instrument_name : String, 
        instrument_type : String, 
        underlying_asset : String, 
        quote_asset : String, 
        price_step : String, 
        amount_step : String, 
        min_order_value : String, 
        max_order_value : String, 
        max_notional_value : String, 
        mark_price : String, 
        forward_price : String, 
        index_price : String, 
        is_active : bool, 
        option_type : String, 
        expiry : String, 
        strike : String, 
        greeks : Greeks
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Greeks {
    delta : String, 
    theta : String, 
    gamma : String, 
    rho : String, 
    vega : String, 
    iv : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PortfolioGreeks {
    asset : String, 
    delta : String, 
    theta : String, 
    gamma : String, 
    rho : String, 
    vega : String, 
    iv : String
}

pub struct Config {
    rest_url : String, 
    ws_url : String, 
    signing_domain : SigningDomain
}

struct SigningDomain {
    name: String, 
    version : String, 
    chain_id : U256
}

pub struct RestHeaders {
    pub aevo_key : String, 
    pub aevo_secret : String, 
    pub header_map : HeaderMap
}

impl RestHeaders {
    pub fn new(key: String, secret: String) -> Result<Self> {
        let mut map = HeaderMap::new(); 
        map.insert("AEVO-KEY", key.parse()?);
        map.insert("AEVO-SECRET", secret.parse()?); 

        Ok(RestHeaders{
            aevo_key : key, 
            aevo_secret : secret, 
            header_map : map
        })
    }
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
    pub rest_headers : RestHeaders, 
    pub extra_headers : Option<String>, 
}

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

    

    pub async fn get_index(&self, asset: String) -> Result<RestResponse> {
        let response = self.client.get(format!("{}/index?asset={}", self.env.get_config().rest_url, asset)).send().await?; 
        let data = response.json::<RestResponse>().await?;
        Ok(data)
    }   

    pub async fn get_markets(&self, asset: String) -> Result<RestResponse> {
        let response = self.client.get(format!("{}/markets?asset={}", self.env.get_config().rest_url, asset)).send().await?; 
        let data = response.json::<RestResponse>().await?;
        Ok(data)
    }

    pub async fn rest_cancel_order(&self, order_id : String) -> Result<RestResponse> {
        let response = self.client.delete(format!("{}/orders/{}", self.env.get_config().rest_url, order_id)).headers(self.rest_headers.header_map.clone()).send().await?; 
        let data = response.json::<RestResponse>().await?;
        Ok(data)
    }  

    pub async fn rest_get_account(&self) -> Result<RestResponse> {
        let response = self.client.get(format!("{}/account", self.env.get_config().rest_url)).headers(self.rest_headers.header_map.clone()).send().await?; 
        let data = response.json::<RestResponse>().await?;
        Ok(data)
    }

    pub async fn rest_get_portfolio(&self) -> Result<RestResponse> {
        let response = self.client.get(format!("{}/portfolio", self.env.get_config().rest_url)).headers(self.rest_headers.header_map.clone()).send().await?; 
        let data = response.json::<RestResponse>().await?;
        Ok(data)
    }

    pub async fn rest_get_open_rders(&self) -> Result<RestResponse> {
        let response = self.client.get(format!("{}/orders", self.env.get_config().rest_url)).headers(self.rest_headers.header_map.clone()).send().await?; 
        let data = response.json::<RestResponse>().await?;
        Ok(data)
    }

    pub async fn rest_cancel_all_orders(&self, instrument_type: Option<String>, asset: Option<String> ) -> Result<RestResponse> {
        let mut body = HashMap::<String, String>::new(); 
        if let Some(i_t) = instrument_type {
            body.insert("instrument_type".to_string(), i_t); 
        };

        if let Some(a) = asset {
            body.insert("asset".to_string(), a); 
        };

        let response = self.client.delete(format!("{}/orders-all", self.env.get_config().rest_url)).json(&body).headers(self.rest_headers.header_map.clone()).send().await?; 
        let response_txt = response.text().await?; 
        let data = serde_json::from_str::<RestResponse>(&response_txt)?; 
        Ok(data)
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

    pub async fn sign_order(
        &self, 
        instrument_id: u64, 
        is_buy: bool, 
        limit_price: f64, 
        quantity: f64, 
        timestamp: i64, 
        price_decimals: Option<u8>, 
        amount_decimals: Option<u8>
    ) -> Result<(U256, String, String)> {
        let salt = U256::from(rand::random::<u64>());
        let price_decimals = match price_decimals {
            Some(d) => d, 
            None => 6
        }; 

        let amount_decimals = match amount_decimals {
            Some(d) => d, 
            None => 6
        };

        let wallet_address: Address = match &self.wallet_address {
            Some(address) => address.parse()?, 
            None => return Err(eyre!("Order sign error: Wallet address not set"))
        };

        let signing_key = match &self.signing_key {
            Some(key) => key, 
            None => return Err(eyre!("Order sign error: Signing key not set"))
        };

        let order = Order {
            maker: wallet_address, 
            isBuy: is_buy, 
            limitPrice: U256::from((limit_price * 10_i32.pow(price_decimals as u32) as f64).floor()), 
            amount: U256::from((quantity * 10_i32.pow(amount_decimals as u32) as f64).floor()), 
            salt: salt, 
            instrument: U256::from(instrument_id), 
            timestamp : U256::from(timestamp)
        }; 

        let signing_domain = self.env.get_config().signing_domain; 

        let domain = Eip712Domain {
            name: Some(signing_domain.name.into()), 
            version: Some(signing_domain.version.into()), 
            chain_id: Some(signing_domain.chain_id), 
            verifying_contract: None, 
            salt: None
        };
        
        let signable_bytes = order.eip712_signing_hash(&domain); 
        let signer = PrivateKeySigner::from_str(signing_key)?; 
        let signature: Signature = signer.sign_hash(&signable_bytes).await?;

        Ok((salt, signature.as_bytes().encode_hex(), format!("0x{}", signable_bytes.encode_hex())))
    } 

    pub async fn create_order_ws (
        &self, 
        instrument_id: u64, 
        is_buy: bool, 
        limit_price: f64, 
        quantity: f64, 
        post_only: Option<bool>,
        mmp: Option<bool>,
        price_decimals: Option<u8>, 
        amount_decimals: Option<u8>,
    ) -> Result<(WsRequestData, String)>{
        let timestamp = Utc::now().timestamp();
        let (salt, signature, order_id) = self.sign_order(
            instrument_id, 
            is_buy, 
            limit_price, 
            quantity, 
            timestamp, 
            price_decimals, 
            amount_decimals
        ).await?; 

        let wallet_address= match &self.wallet_address {
            Some(address) => address.clone(), 
            None => return Err(eyre!("Order sign error: Wallet address not set"))
        };

        let price_decimals = match price_decimals {
            Some(d) => d, 
            None => 6
        }; 

        let amount_decimals = match amount_decimals {
            Some(d) => d, 
            None => 6
        };
        
        let payload = WsRequestData::OrderData {
            maker : wallet_address, 
            is_buy: is_buy, 
            instrument: instrument_id.to_string(), 
            limit_price : (limit_price * 10_i32.pow(price_decimals as u32) as f64).floor().to_string(), 
            amount : (quantity * 10_i32.pow(amount_decimals as u32) as f64).floor().to_string(), 
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

    pub async fn create_order_rest (
        &self, 
        instrument_id: u64, 
        is_buy: bool, 
        limit_price: f64, 
        quantity: f64, 
        post_only: Option<bool>, 
        reduce_only: Option<bool>, 
        close_position: Option<bool>, 
        price_decimals: Option<u8>, 
        amount_decimals: Option<u8>, 
        trigger: Option<String>, 
        stop: Option<String>
    ) -> Result<(RestOrder, String)>{
        let timestamp = Utc::now().timestamp();
        let (salt, signature, order_id) = self.sign_order(
            instrument_id, 
            is_buy, 
            limit_price, 
            quantity, 
            timestamp, 
            price_decimals, 
            amount_decimals
        ).await?; 

        let wallet_address= match &self.wallet_address {
            Some(address) => address.clone(), 
            None => return Err(eyre!("Order sign error: Wallet address not set"))
        };

        let price_decimals = match price_decimals {
            Some(d) => d, 
            None => 6
        }; 

        let amount_decimals = match amount_decimals {
            Some(d) => d, 
            None => 6
        };
        
        let payload = RestOrder {
            maker : wallet_address, 
            is_buy: is_buy, 
            instrument: instrument_id.to_string(), 
            limit_price : (limit_price * 10_i32.pow(price_decimals as u32) as f64).floor().to_string(), 
            amount : (quantity * 10_i32.pow(amount_decimals as u32) as f64).floor().to_string(), 
            salt : salt.to_string(), 
            signature : signature, 
            post_only : match post_only {
                Some(p) => p, 
                None => true
            }, 
            reduce_only : match reduce_only {
                Some(r) => r, 
                None => false
            },
            close_position : match close_position {
                Some(c) => c, 
                None => false
            },
            timestamp : timestamp.to_string(), 
            trigger : trigger, 
            stop : stop
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

        let (data, order_id) = self.create_order_ws(instrument_id, is_buy, limit_price, quantity, post_only, mmp, None, None).await?;


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
            limit_price, 
            quantity, 
            timestamp, 
            None, 
            None
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

    pub async fn rest_create_order (
        &mut self, 
        instrument_id: u64, 
        is_buy: bool, 
        limit_price: f64, 
        quantity: f64, 
        post_only: Option<bool>
    ) -> Result<RestResponse>{
        let (data, order_id) = self.create_order_rest(
            instrument_id, 
            is_buy, 
            limit_price, 
            quantity, 
            post_only, 
            None, 
            None, 
            None, 
            None, 
            None, 
            None
        ).await?; 

        info!("Creating rest order: {:?}", data); 

        let response = self.client
            .post(format!("{}/orders", self.env.get_config().rest_url))
            .json(&data)
            .headers(self.rest_headers.header_map.clone())
            .send().await?; 
        
        let data = response.json::<RestResponse>().await?;
        Ok(data)
    }

    pub async fn rest_create_market_order (
        &mut self, 
        instrument_id: u64, 
        is_buy: bool, 
        quantity: f64
    ) -> Result<RestResponse> {
        let limit_price = match is_buy {
            true => f64::MAX, 
            false => 0.0
        }; 

        let (data, order_id) = self.create_order_rest(
            instrument_id, 
            is_buy, 
            limit_price, 
            quantity, 
            None, 
            None, 
            None, 
            None, 
            None, 
            None, 
            None
        ).await?; 

        info!("Creating rest market order: {:?}", data); 

        let response = self.client
            .post(format!("{}/orders", self.env.get_config().rest_url))
            .json(&data)
            .headers(self.rest_headers.header_map.clone())
            .send().await?; 
        
        let data = response.json::<RestResponse>().await?;
        Ok(data)
    }

    pub async fn sign_withdraw(
        &self, 
        collateral: String, 
        to: String, 
        amount: f64, 
        data: U256, 
        amount_decimals: Option<u8>
    ) -> Result<(U256, String, String)>{
        let salt = U256::from(rand::random::<u64>());

        let amount_decimals = match amount_decimals {
            Some(d) => d, 
            None => 6
        };

        let signing_key = match &self.signing_key {
            Some(key) => key, 
            None => return Err(eyre!("Order sign error: Signing key not set"))
        };

        let withdraw = Withdraw {
            collateral: collateral.parse()?, 
            to: to.parse()?, 
            amount: U256::from((amount * 10_i32.pow(amount_decimals as u32) as f64).floor()), 
            salt: salt, 
            data: data
        }; 
        
        let signing_domain = self.env.get_config().signing_domain; 

        let domain = Eip712Domain {
            name: Some(signing_domain.name.into()), 
            version: Some(signing_domain.version.into()), 
            chain_id: Some(signing_domain.chain_id), 
            verifying_contract: None, 
            salt: None
        };
        
        let signable_bytes = withdraw.eip712_signing_hash(&domain); 
        let signer = PrivateKeySigner::from_str(signing_key)?; 
        let signature: Signature = signer.sign_hash(&signable_bytes).await?;

        Ok((salt, signature.as_bytes().encode_hex(), format!("0x{}", signable_bytes.encode_hex())))
    }

    pub async fn create_withdraw(
        &self, 
        collateral: String, 
        to: String, 
        amount: f64, 
        data: Option<U256>, 
        amount_decimals: Option<u8>
    ) -> Result<(RestWithdraw, String)>{

        let _data = match data {
            Some(val) => val, 
            None => U256::ZERO
        };

        let wallet_address= match &self.wallet_address {
            Some(address) => address.clone(), 
            None => return Err(eyre!("Order sign error: Wallet address not set"))
        };

        let (salt, signature, withdraw_id) = self.sign_withdraw(collateral.clone(), to.clone(), amount, _data, amount_decimals).await?;

        let amount_decimals = match amount_decimals {
            Some(d) => d, 
            None => 6
        }; 

        let payload = RestWithdraw {
            account : wallet_address, 
            collateral: collateral, 
            to: to, 
            amount : (amount * 10_i32.pow(amount_decimals as u32) as f64).floor().to_string(), 
            salt: salt.to_string(), 
            signature: signature, 
            data: match data {
                Some(val) => Some(val.to_string()), 
                None => None
            }
        }; 

        Ok((payload, withdraw_id))
    }

    pub async fn withdraw(
        &self, 
        amount: f64, 
        collateral: Option<String>, 
        to: Option<String>, 
        data: Option<U256>, 
        amount_decimals: Option<u8>
    ) -> Result<RestResponse> {

        let collateral = match collateral {
            Some(val) => val, 
            None => self.env.get_addresses().l1_usdc,
        }; 

        let to = match to {
            Some(val) => val, 
            None => self.env.get_addresses().l2_withdraw_proxy
        }; 

        let (data, withdraw_id) = self.create_withdraw(collateral, to, amount, data, amount_decimals).await?;

        info!("Withdrawing {}", withdraw_id); 
        info!("Withdraw data : {:?}", data); 

        let response = self.client
            .post(format!("{}/withdraw", self.env.get_config().rest_url))
            .json(&data)
            .headers(self.rest_headers.header_map.clone())
            .send().await?; 
        
        let data = response.json::<RestResponse>().await?;

        Ok(data)
    }
}


sol!{
    struct Order {
        address maker; 
        bool isBuy; 
        uint256 limitPrice; 
        uint256 amount; 
        uint256 salt; 
        uint256 instrument; 
        uint256 timestamp;
    }

    struct Withdraw {
        address collateral; 
        address to; 
        uint256 amount; 
        uint256 salt; 
        uint256 data; 
    }
}
