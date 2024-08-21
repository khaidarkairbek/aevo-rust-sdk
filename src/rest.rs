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
    GetIndex(GetIndexData),
    GetOrders (Vec<OrderData>), 
    GetMarkets (Vec<MarketInfo>), 
    DeleteOrder (DeleteOrderData), 
    GetAccount (GetAccountData), 
    GetPortfolio (GetPortfolioData), 
    DeleteOrdersAll (DeleteOrdersAllData),
    CreateOrder (OrderData),
    EditOrder (OrderData), 
    Withdraw (WithdrawData), 
    Error(ErrorData)
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ErrorData {
    pub error : String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct WithdrawData {
    pub timestamp : String, 
    pub price : String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GetIndexData {
    pub timestamp : String, 
    pub price : String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GetAccountData {
    pub account : String, 
    pub username : String, 
    pub account_type : String, 
    pub portfolio : bool, 
    pub equity : String, 
    pub balance : String, 
    pub credit : String, 
    pub credited : bool, 
    pub collaterals : Vec<CollateralInfo>, 
    pub available_balance : String, 
    pub initial_margin : String, 
    pub maintenance_margin : String, 
    pub email_address : String, 
    pub in_liquidation : bool, 
    pub referral_bonus : f64, 
    pub has_been_referred : bool, 
    pub referrer : Option<String>, 
    pub intercom_hash : String, 
    pub permissions : Option<Vec<String>>,
    pub positions : Vec<String>, //should be position type
    pub signing_keys : Vec<SigningKeyInfo>, 
    pub api_keys : Vec<ApiKeyInfo>, 
    pub fee_structures : Vec<FeeStructureInfo>, 
    pub leverages : Vec<LeverageInfo>, 
    pub manual_mode : bool, 
    pub manual_withdrawals : Vec<ManualWithdrawalInfo>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GetPortfolioData {
    pub balance : String, 
    pub pnl : String, 
    pub realized_pnl : String, 
    pub profit_factor : String, 
    pub win_rate : String, 
    pub sharpe_ratio : String, 
    pub greeks : Vec<PortfolioGreeks>, 
    pub user_margin : UsedMarginInfo
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DeleteOrdersAllData {
    pub success : bool, 
    pub order_ids : Vec<String>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DeleteOrderData {
    pub order_id : String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RestWithdraw {
    pub account : String, 
    pub collateral : String, 
    pub to : String, 
    pub amount : String, 
    pub salt : String, 
    pub signature : String, 
    pub data : Option<String>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RestOrder {
    pub maker : String, 
    pub is_buy : bool, 
    pub instrument: String, 
    pub limit_price: String, 
    pub amount : String, 
    pub salt : String, 
    pub signature : String, 
    pub post_only : bool, 
    pub reduce_only : bool, 
    pub close_position : bool, 
    pub timestamp : String, 
    pub trigger : Option<String>, 
    pub stop : Option<String>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct OrderData {
    pub order_id : String, 
    pub account : String, 
    pub instrument_id : String, 
    pub instrument_name : String, 
    pub instrument_type : String, 
    pub order_type : String, 
    pub side : String, // buy  or sell
    pub amount : String, 
    pub price : String, 
    pub avg_price : Option<String>, 
    pub filled : String, 
    pub order_status : String,
    pub post_only : Option<bool>, 
    pub reduce_only : Option<bool>, 
    pub initial_margin : Option<String>, 
    pub option_type : Option<String>, 
    pub iv : Option<String>, 
    pub expiry : Option<String>, 
    pub strike : Option<String>, 
    pub created_timestamp : Option<String>, 
    pub timestamp : String, 
    pub system_type : String, 
    pub time_in_force : Option<String>, 
    pub stop : Option<String>, 
    pub trigger : Option<String>, 
    pub close_position : Option<bool>, 
    pub partial_position : Option<bool>,
    pub isolated_margin : Option<String>, 
    pub parent_order_id : Option<String>,
    pub self_trade_prevention : Option<String>
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
    pub delta : String, 
    pub theta : String, 
    pub  gamma : String, 
    pub rho : String, 
    pub vega : String, 
    pub iv : String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PortfolioGreeks {
    pub asset : String, 
    pub delta : String, 
    pub theta : String, 
    pub gamma : String, 
    pub rho : String, 
    pub vega : String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SigningKeyInfo {
    pub signing_key : String, 
    pub expiry : String, 
    pub created_timestamp : String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ApiKeyInfo {
    pub api_key : String, 
    pub read_only : bool, 
    pub created_timestamp : String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FeeStructureInfo {
    pub asset : String, 
    pub instrument_type : String, 
    pub taker_fee : String, 
    pub maker_fee : String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LeverageInfo {
    pub instrument_id : String, 
    pub leverage : String, 
    pub margin_type : String  // Cross or Margin 
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct UsedMarginInfo {
    pub used : String, 
    pub balance : String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ManualWithdrawalInfo {
    pub account : String, 
    pub amount : String, 
    pub chain_id : String, 
    pub collateral : String, 
    pub withdrawal_id : String, 
    pub to : String, 
    pub label : String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CollateralInfo {
    pub collateral_asset : String, 
    pub balance: String, 
    pub available_balance: String, 
    pub withdrawable_balance : String, 
    pub margin_value : String, 
    pub collateral_value : String, 
    pub collateral_yield_bearing : bool
}

impl AevoClient {
    pub async fn get_index(&self, asset: String) -> Result<RestResponse> {
        let response = self.client
            .get(format!("{}/index?asset={}", self.env.get_config().rest_url, asset))
            .send().await?; 
        let data = response.json::<GetIndexData>().await?;
        Ok(RestResponse::GetIndex(data))
    }   

    pub async fn get_markets(&self, asset: String) -> Result<RestResponse> {
        let response = self.client.get(format!("{}/markets?asset={}", self.env.get_config().rest_url, asset)).send().await?; 
        let data = response.json::<Vec<MarketInfo>>().await?;
        Ok(RestResponse::GetMarkets(data))
    }

    pub async fn rest_cancel_order(&self, order_id : String) -> Result<RestResponse> {
        info!("Cancelling order {}", order_id); 
        if let Some(ClientCredentials{api_key, api_secret, ..}) = &self.credentials {
            let response = self.client
                .delete(format!("{}/orders/{}", self.env.get_config().rest_url, order_id))
                .header("AEVO-KEY", api_key)
                .header("AEVO-SECRET", api_secret)
                .send().await?; 
            let data = response.json::<DeleteOrderData>().await?;
            Ok(RestResponse::DeleteOrder(data))
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
            let data = response.json::<GetAccountData>().await?;
            Ok(RestResponse::GetAccount(data))
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

            let data = response.json::<GetPortfolioData>().await?;
            Ok(RestResponse::GetPortfolio(data))
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
            let data = response.json::<Vec<OrderData>>().await?;
            Ok(RestResponse::GetOrders(data))
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
            let data = response.json::<DeleteOrdersAllData>().await?;
            Ok(RestResponse::DeleteOrdersAll(data))
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
            let data = response.json::<OrderData>().await?;
            Ok(RestResponse::CreateOrder(data))
        } else {
            Err(eyre!("Api key and/or secret are not established"))
        }
    }

    pub async fn rest_edit_order (
        &self, 
        order_id : &String, 
        instrument_id: u64, 
        is_buy: bool, 
        limit_price: f64, 
        quantity: f64, 
        post_only: Option<bool>
    ) -> Result<RestResponse> {
        if let Some(ClientCredentials{api_key, api_secret, ..}) = &self.credentials {
            let (data, new_order_id) = self.create_order_rest(
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
    
            info!("Editing rest order: {:?}", data); 
    
            let response = self.client
                .post(format!("{}/orders/{}", self.env.get_config().rest_url, order_id))
                .json(&data)
                .header("AEVO-KEY", api_key)
                .header("AEVO-SECRET", api_secret)
                .send().await?; 
            let data = response.json::<OrderData>().await?;
            Ok(RestResponse::EditOrder(data))
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
            
            let data = response.json::<OrderData>().await?;
            Ok(RestResponse::CreateOrder(data))
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
    
            let response = self.client
                .post(format!("{}/withdraw", self.env.get_config().rest_url))
                .json(&data)
                .header("AEVO-KEY", api_key)
                .header("AEVO-SECRET", api_secret)
                .send().await?; 
            
            let data = response.json::<WithdrawData>().await?;
    
            Ok(RestResponse::Withdraw(data))
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