use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WsRequest {
    pub op : String, 
    pub data : WsRequestData, 
    pub id : Option<u64>
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
pub enum WsResponse {
    SubscribeResponse {
        channel : String, 
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
    instrument_id : String, 
    instrument_name : String, 
    instrument_type : String, 
    amount : String, 
    mark_price : String, 
    option : Option<OptionData>, 
    asset : String, 
    side : String, 
    avg_entry_price : String, 
    unrealized_pnl : String, 
    maintenance_margin : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OptionData {
    strike : String, 
    option_type : String, 
    expiry : String, 
    iv : String, 
    delta : String, 
    theta : String, 
    rho : String, 
    vega : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fill {
    trade_id : String, 
    order_id : String, 
    instrument_id : String, 
    instrument_name : String, 
    instrument_type : String, 
    price : String, 
    side : String, 
    fees : String, 
    filled : String, 
    order_status : String, 
    liquidity : String, 
    created_timestamp : String, 
    system_type : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    order_id : String, 
    account : String, 
    instrument_id : String, 
    instrument_name : String, 
    instrument_type : String, 
    order_type : String, 
    side : String, 
    price : String, 
    amount : String, 
    filled : String, 
    order_status : String, 
    created_timestamp : String, 
    system_type : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookTicker {
    instrument_id : String, 
    instrument_name : String, 
    instrument_type : String, 
    bid : PriceLevel, 
    ask : PriceLevel
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ticker {
    instrument_id : String, 
    instrument_name : String, 
    instrument_type : String, 
    funding_rate : String, 
    next_funding_rate : String, 
    mark : PriceLevel, 
    bid : PriceLevel, 
    ask : PriceLevel
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceLevel {
    price : String, 
    delta : String, 
    theta : String, 
    gamma : String, 
    rho : String, 
    vega : String, 
    iv : String, 
    amount : Option<String>
}
