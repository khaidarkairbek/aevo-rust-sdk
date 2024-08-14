use crate::aevo::{AevoClient, ClientCredentials, AMOUNT_DECIMALS, PRICE_DECIMALS};
use std::collections::HashMap;
use alloy::primitives::U256; 
use log::{info, debug, error};
use serde_derive::{Deserialize, Serialize};
use eyre::{eyre, Result};
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum RestResponse {
    GetIndex { timestamp : String, price : String },
    GetOrders (Vec<OrderInfo>), 
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
        collaterals : Vec<CollateralInfo>, 
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
    Withdraw{success : bool}, 
    Error{ error : String }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RestWithdraw {
    account : String, 
    collateral : String, 
    to : String, 
    amount : String, 
    salt : String, 
    signature : String, 
    data : Option<String>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
    avg_price : Option<String>, 
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Greeks {
    delta : String, 
    theta : String, 
    gamma : String, 
    rho : String, 
    vega : String, 
    iv : String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PortfolioGreeks {
    asset : String, 
    delta : String, 
    theta : String, 
    gamma : String, 
    rho : String, 
    vega : String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SigningKeyInfo {
    signing_key : String, 
    expiry : String, 
    created_timestamp : String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ApiKeyInfo {
    api_key : String, 
    read_only : bool, 
    created_timestamp : String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FeeStructureInfo {
    asset : String, 
    instrument_type : String, 
    taker_fee : String, 
    maker_fee : String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LeverageInfo {
    instrument_id : String, 
    leverage : String, 
    margin_type : String  // Cross or Margin 
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct UsedMarginInfo {
    used : String, 
    balance : String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ManualWithdrawalInfo {
    account : String, 
    amount : String, 
    chain_id : String, 
    collateral : String, 
    withdrawal_id : String, 
    to : String, 
    label : String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CollateralInfo {
    collateral_asset : String, 
    balance: String, 
    available_balance: String, 
    withdrawable_balance : String, 
    margin_value : String, 
    collateral_value : String, 
    collateral_yield_bearing : bool
}

impl AevoClient {
    pub async fn get_index(&self, asset: String) -> Result<RestResponse> {
        let response = self.client
            .get(format!("{}/index?asset={}", self.env.get_config().rest_url, asset))
            .send().await?; 
        let data = response.json::<RestResponse>().await?;
        Ok(data)
    }   

    pub async fn get_markets(&self, asset: String) -> Result<RestResponse> {
        let response = self.client.get(format!("{}/markets?asset={}", self.env.get_config().rest_url, asset)).send().await?; 
        let data = response.json::<RestResponse>().await?;
        Ok(data)
    }

    pub async fn rest_cancel_order(&self, order_id : String) -> Result<RestResponse> {
        info!("Cancelling order {}", order_id); 
        if let Some(ClientCredentials{api_key, api_secret, ..}) = &self.credentials {
            let response = self.client
                .delete(format!("{}/orders/{}", self.env.get_config().rest_url, order_id))
                .header("AEVO-KEY", api_key)
                .header("AEVO-SECRET", api_secret)
                .send().await?; 
            let data = response.json::<RestResponse>().await?;
            Ok(data)
        } else {
            Err(eyre!("Api key and/or secret are not established"))
        }
    }  

    pub async fn rest_get_account(&self) -> Result<RestResponse> {
        info!("Getting account info"); 
        if let Some(ClientCredentials{api_key, api_secret, ..}) = &self.credentials {
            let response = self.client
                .get(format!("{}/account", self.env.get_config().rest_url))
                .header("AEVO-KEY", api_key)
                .header("AEVO-SECRET", api_secret)
                .send().await?; 
            let data = response.json::<RestResponse>().await?;
            Ok(data)
        } else {
            Err(eyre!("Api key and/or secret are not established"))
        }
    }

    pub async fn rest_get_portfolio(&self) -> Result<RestResponse> {
        info!("Getting portfolio info");
        if let Some(ClientCredentials{api_key, api_secret, ..}) = &self.credentials {
            let response = self.client
                .get(format!("{}/portfolio", self.env.get_config().rest_url))
                .header("AEVO-KEY", api_key)
                .header("AEVO-SECRET", api_secret)
                .send().await?;  

            let data = response.json::<RestResponse>().await?;
            Ok(data)
        } else {
            Err(eyre!("Api key and/or secret are not established"))
        }
    }

    pub async fn rest_get_open_orders(&self) -> Result<RestResponse> {
        info!("Getting open orders");
        if let Some(ClientCredentials{api_key, api_secret, ..}) = &self.credentials {
            let response = self.client
                .get(format!("{}/orders", self.env.get_config().rest_url))
                .header("AEVO-KEY", api_key)
                .header("AEVO-SECRET", api_secret)
                .send().await?; 
            info!("Response: {:?}", response); 
            let data = response.json::<RestResponse>().await?;
            Ok(data)
        } else {
            Err(eyre!("Api key and/or secret are not established"))
        }
    }

    pub async fn rest_cancel_all_orders(&self, instrument_type: Option<String>, asset: Option<String> ) -> Result<RestResponse> {
        info!("Cancelling all orders"); 

        if let Some(ClientCredentials{api_key, api_secret, ..}) = &self.credentials {
            let mut body = HashMap::<String, String>::new(); 
            if let Some(i_t) = instrument_type {
                body.insert("instrument_type".to_string(), i_t); 
            };

            if let Some(a) = asset {
                body.insert("asset".to_string(), a); 
            };

            let response = self.client
                .delete(format!("{}/orders-all", self.env.get_config().rest_url))
                .json(&body)
                .header("AEVO-KEY", api_key)
                .header("AEVO-SECRET", api_secret)
                .send().await?; 
            let data = response.json::<RestResponse>().await?;
            Ok(data)
        } else {
            Err(eyre!("Api key and/or secret are not established"))
        }

    }

    pub async fn create_order_rest (
        &self, 
        instrument_id: u64, 
        is_buy: bool, 
        limit_price: Option<f64>, 
        quantity: f64, 
        post_only: Option<bool>, 
        reduce_only: Option<bool>, 
        close_position: Option<bool>,
        trigger: Option<String>, 
        stop: Option<String>
    ) -> Result<(RestOrder, String)>{
        let timestamp = Utc::now().timestamp();
        let (salt, signature, order_id) = self.sign_order(
            instrument_id, 
            is_buy, 
            limit_price, 
            quantity, 
            timestamp
        ).await?; 

        let wallet_address= match &self.credentials {
            Some(ClientCredentials {wallet_address, ..}) => wallet_address.clone(), 
            None => return Err(eyre!("Order sign error: Wallet address not set"))
        };
        
        let payload = RestOrder {
            maker : wallet_address, 
            is_buy: is_buy, 
            instrument: instrument_id.to_string(), 
            limit_price : match limit_price {
                Some(p) => (p * 10_i32.pow(PRICE_DECIMALS) as f64).floor().to_string(), 
                None => {
                    if is_buy {
                        U256::MAX.to_string()
                    } else {
                        U256::ZERO.to_string()
                    }
                }
            }, 
            amount : (quantity * 10_i32.pow(AMOUNT_DECIMALS) as f64).floor().to_string(), 
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

    pub async fn rest_create_order (
        &self, 
        instrument_id: u64, 
        is_buy: bool, 
        limit_price: f64, 
        quantity: f64, 
        post_only: Option<bool>
    ) -> Result<RestResponse>{
        if let Some(ClientCredentials{api_key, api_secret, ..}) = &self.credentials {
            let (data, order_id) = self.create_order_rest(
                instrument_id, 
                is_buy, 
                Some(limit_price), 
                quantity, 
                post_only, 
                None, 
                None, 
                None, 
                None,
            ).await?; 
    
            info!("Creating rest order: {:?}", data); 
    
            let response = self.client
                .post(format!("{}/orders", self.env.get_config().rest_url))
                .json(&data)
                .header("AEVO-KEY", api_key)
                .header("AEVO-SECRET", api_secret)
                .send().await?; 
            let data = response.json::<RestResponse>().await?;
            Ok(data)
        } else {
            Err(eyre!("Api key and/or secret are not established"))
        }
    }

    pub async fn rest_create_market_order (
        &self, 
        instrument_id: u64, 
        is_buy: bool, 
        quantity: f64
    ) -> Result<RestResponse> {
        if let Some(ClientCredentials{api_key, api_secret, ..}) = &self.credentials {
            let (data, order_id) = self.create_order_rest(
                instrument_id, 
                is_buy, 
                None, 
                quantity, 
                Some(false), 
                None, 
                None, 
                None, 
                None,
            ).await?; 
    
            info!("Creating rest market order: {:?}", data); 
    
            let response = self.client
                .post(format!("{}/orders", self.env.get_config().rest_url))
                .json(&data)
                .header("AEVO-KEY", api_key)
                .header("AEVO-SECRET", api_secret)
                .send().await?; 

            info!("Response: {:?}", response); 
            
            let data = response.json::<RestResponse>().await?;
            Ok(data)
        } else {
            Err(eyre!("Api key and/or secret are not established"))
        }
    }

    pub async fn withdraw(
        &self, 
        amount: f64, 
        collateral: Option<String>, 
        to: Option<String>, 
        data: Option<U256>,
    ) -> Result<RestResponse> {
        if let Some(ClientCredentials{api_key, api_secret, ..}) = &self.credentials {
            let collateral = match collateral {
                Some(val) => val, 
                None => self.env.get_addresses().l1_usdc,
            }; 
    
            let to = match to {
                Some(val) => val, 
                None => self.env.get_addresses().l2_withdraw_proxy
            }; 
    
            let (data, withdraw_id) = self.create_withdraw(collateral, to, amount, data).await?;
    
            info!("Withdrawing {}", withdraw_id); 
            info!("Withdraw data : {:?}", data); 
    
            let response = self.client
                .post(format!("{}/withdraw", self.env.get_config().rest_url))
                .json(&data)
                .header("AEVO-KEY", api_key)
                .header("AEVO-SECRET", api_secret)
                .send().await?; 
            
            let data = response.json::<RestResponse>().await?;
    
            Ok(data)
        } else {
            Err(eyre!("Api key and/or secret are not established"))
        }
    }

    pub async fn create_withdraw(
        &self, 
        collateral: String, 
        to: String, 
        amount: f64, 
        data: Option<U256>,
    ) -> Result<(RestWithdraw, String)>{

        let _data = match data {
            Some(val) => val, 
            None => U256::ZERO
        };

        let wallet_address= match &self.credentials {
            Some(ClientCredentials{wallet_address, ..}) => wallet_address.clone(), 
            None => return Err(eyre!("Order sign error: Wallet address not set"))
        };

        let (salt, signature, withdraw_id) = self.sign_withdraw(collateral.clone(), to.clone(), amount, _data).await?;

        let payload = RestWithdraw {
            account : wallet_address, 
            collateral: collateral, 
            to: to, 
            amount : (amount * 10_i32.pow(AMOUNT_DECIMALS) as f64).floor().to_string(), 
            salt: salt.to_string(), 
            signature: signature, 
            data: match data {
                Some(val) => Some(val.to_string()), 
                None => None
            }
        }; 

        Ok((payload, withdraw_id))
    }
}