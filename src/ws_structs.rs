use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WsRequest {
    pub op : String, 
    pub data : WsRequestData, 
    pub id : Option<u64>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ping {
    pub op : String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum WsRequestData {
    AuthData {key : String, secret : String}, 
    Ping (String),
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
pub enum WsResponse {
    SubscribeResponse {
        channel : String,
        write_ts : Option<String>, 
        data : WsResponseData
    }, 
    PublishResponse {
        id : Option<String>, 
        data : WsResponseData
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum WsResponseData {
    CancelAllOrdersData {
        success : bool, 
        order_ids : Vec<String>,
    }, 
    CancelOrderData {
        success : bool, 
        order_id : String
    }, 
    CreateEditOrderData {
        order_id : String, 
        account : String, 
        instrument_id : String, 
        instrument_name : String, 
        instrument_type : String, 
        expiry : Option<String>, 
        strike : Option<String>, 
        option_type : Option<String>, 
        order_type : String, 
        order_status : String, 
        side : String, 
        amount : String, 
        price : String, 
        filled : String, 
        initial_margin : String, 
        avg_price : Option<String>, 
        created_timestamp : String, 
        timestamp : String, 
        system_type : String
    }, 
    StatusData {
        account : String, 
        subscriptions : Vec<String>
    }, 
    PingData {
        success : bool, 
        timestamp : String
    }, 
    OrderBookData {
        r#type : String, 
        instrument_id : String, 
        instrument_name : String, 
        instrument_type : String, 
        bids : Vec<Vec<String>>, 
        asks : Vec<Vec<String>>,
        last_updated : String, 
        checksum : String
    },
    IndexData {
        price : String, 
        timestamp : String
    }, 
    OrdersData {
        timestamp : String, 
        orders : Vec<Order>
    }, 
    FillsData {
        timestamp : String, 
        fill : Fill
    }, 
    PositionsData {
        timestamp : String, 
        positions : Vec<Position>
    }, 
    TradesData {
        trade_id : String, 
        instrument_id : String, 
        instrument_name : String, 
        instrument_type : String, 
        side : String, 
        price : String, 
        amount : Option<String>, 
        created_timestamp : String
    }, 
    TickerData {
        timestamp : String, 
        tickers : Vec<Ticker>
    }, 
    BookTickerData {
        timestamp : String, 
        tickers : Vec<BookTicker>
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    pub instrument_id : String, 
    pub instrument_name : String, 
    pub instrument_type : String, 
    pub amount : String, 
    pub mark_price : String, 
    pub option : Option<OptionData>, 
    pub asset : String, 
    pub side : String, 
    pub avg_entry_price : String, 
    pub unrealized_pnl : String, 
    pub maintenance_margin : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OptionData {
    pub strike : String, 
    pub option_type : String, 
    pub expiry : String, 
    pub iv : String, 
    pub delta : String, 
    pub theta : String, 
    pub rho : String, 
    pub vega : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fill {
    pub trade_id : String, 
    pub order_id : String, 
    pub instrument_id : String, 
    pub instrument_name : String, 
    pub instrument_type : String, 
    pub price : String, 
    pub side : String, 
    pub fees : String, 
    pub filled : String, 
    pub order_status : String, 
    pub liquidity : String, 
    pub created_timestamp : String, 
    pub system_type : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub order_id : String, 
    pub account : String, 
    pub instrument_id : String, 
    pub instrument_name : String, 
    pub instrument_type : String, 
    pub order_type : String, 
    pub side : String, 
    pub price : String, 
    pub amount : String, 
    pub filled : String, 
    pub order_status : String, 
    pub created_timestamp : String, 
    pub system_type : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookTicker {
    pub instrument_id : String, 
    pub instrument_name : String, 
    pub instrument_type : String, 
    pub bid : PriceLevel, 
    pub ask : PriceLevel
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ticker {
    pub instrument_id : String, 
    pub instrument_name : String, 
    pub instrument_type : String, 
    pub funding_rate : String, 
    pub next_funding_rate : String, 
    pub mark : PriceLevel, 
    pub  bid : PriceLevel, 
    pub ask : PriceLevel
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceLevel {
    pub price : String, 
    pub delta : Option<String>, 
    pub theta : Option<String>, 
    pub gamma : Option<String>, 
    pub rho : Option<String>, 
    pub vega : Option<String>, 
    pub iv : Option<String>, 
    pub amount : Option<String>
}
