pub mod aevo; 
pub mod secret;
pub mod env; 
pub mod signature; 
pub mod rest; 

#[cfg(test)]
mod tests {
    use rest::RestResponse;
    use test_log::test;
    use tokio::{join, sync::mpsc};
    use tokio_tungstenite::tungstenite::Message;
    use super::*;

    #[test(tokio::test)]
    async fn test_open_connection() {
        let mut client = aevo::AevoClient {
            signing_key: None, 
            wallet_address : Some(secret::WALLET_ADDRESS.to_string()), 
            api_key : Some(secret::API_KEY.to_string()),
            api_secret : Some(secret::API_SECRET.to_string()), 
            wallet_private_key : None, 
            connection: None, 
            client: reqwest::Client::new(), 
            env: env::ENV::MAINNET,
        };

        client.open_connection().await.unwrap(); 
    }

    #[test(tokio::test)]
    async fn test_get_index() {
        let client = aevo::AevoClient {
            signing_key: None, 
            wallet_address : None, 
            api_key : None,
            api_secret : None, 
            wallet_private_key : None, 
            connection: None, 
            client: reqwest::Client::new(), 
            env: env::ENV::MAINNET,
        };

        let index = client.get_index("ETH".to_string()).await.unwrap();

        println!("Response: {:?}", index); 

        match index {
            RestResponse::GetIndex { .. } => {}, 
            _ => {
                panic!("Not GetIndex type: {:?}", index)
            }
        }
    }

    #[test(tokio::test)]
    async fn test_get_markets() {
        let client = aevo::AevoClient {
            signing_key: None, 
            wallet_address : None, 
            api_key : None,
            api_secret : None, 
            wallet_private_key : None, 
            connection: None, 
            client: reqwest::Client::new(), 
            env: env::ENV::MAINNET,
        };

        let markets = client.get_markets("ETH".to_string()).await.unwrap();

        println!("Response: {:?}", markets); 

        match markets {
            RestResponse::GetMarkets { .. } => {}, 
            _ => {
                panic!("Not GetMarkets type: {:?}", markets)
            }
        }
    }

    #[test(tokio::test)]
    async fn test_get_account() {
        let client = aevo::AevoClient {
            signing_key: None, 
            wallet_address : Some(secret::WALLET_ADDRESS.to_string()), 
            api_key : Some(secret::API_KEY.to_string()),
            api_secret : Some(secret::API_SECRET.to_string()), 
            wallet_private_key : None, 
            connection: None, 
            client: reqwest::Client::new(), 
            env: env::ENV::MAINNET,
        };

        let account = client.rest_get_account().await.unwrap();

        println!("Response: {:?}", account); 

        match account {
            RestResponse::GetAccount { .. } => {}, 
            _ => {
                panic!("Not GetAccount type: {:?}", account)
            }
        }
    }

    #[test(tokio::test)]
    async fn test_get_portfolio() {
        let client = aevo::AevoClient {
            signing_key: None, 
            wallet_address : Some(secret::WALLET_ADDRESS.to_string()), 
            api_key : Some(secret::API_KEY.to_string()),
            api_secret : Some(secret::API_SECRET.to_string()), 
            wallet_private_key : None, 
            connection: None, 
            client: reqwest::Client::new(), 
            env: env::ENV::MAINNET,
        };

        let portfolio = client.rest_get_portfolio().await.unwrap();

        println!("Response: {:?}", portfolio); 

        match portfolio {
            RestResponse::GetPortfolio { .. } => {}, 
            _ => {
                panic!("Not GetPortfolio type: {:?}", portfolio)
            }
        }
    }

    #[test(tokio::test)]
    async fn test_get_open_orders() {
        let client = aevo::AevoClient {
            signing_key: None, 
            wallet_address : Some(secret::WALLET_ADDRESS.to_string()), 
            api_key : Some(secret::API_KEY.to_string()),
            api_secret : Some(secret::API_SECRET.to_string()), 
            wallet_private_key : None, 
            connection: None, 
            client: reqwest::Client::new(), 
            env: env::ENV::MAINNET,
        };

        let open_orders = client.rest_get_open_orders().await.unwrap();

        println!("Response: {:?}", open_orders); 

        match open_orders {
            RestResponse::GetOrders { .. } => {}, 
            _ => {
                panic!("Not GetOrders type: {:?}", open_orders)
            }
        }
    }

    #[test(tokio::test)]
    async fn test_create_order() {
        let client = aevo::AevoClient {
            signing_key: Some(secret::SIGNING_KEY.to_string()), 
            wallet_address : Some(secret::WALLET_ADDRESS.to_string()), 
            api_key : Some(secret::API_KEY.to_string()),
            api_secret : Some(secret::API_SECRET.to_string()), 
            wallet_private_key : None, 
            connection: None, 
            client: reqwest::Client::new(), 
            env: env::ENV::MAINNET,
        };

        let (order, order_id) = client.create_order_rest(
            1, 
            true, 
            Some(2400.0), 
            0.01, 
            None, 
            None, 
            None, 
            None, 
            None
        ).await.unwrap();

        println!("Order created: {:?} with id : {}", order, order_id); 
    }

    #[test(tokio::test)]
    async fn test_open_order() {
        let mut client = aevo::AevoClient {
            signing_key: Some(secret::SIGNING_KEY.to_string()), 
            wallet_address : Some(secret::WALLET_ADDRESS.to_string()), 
            api_key : Some(secret::API_KEY.to_string()),
            api_secret : Some(secret::API_SECRET.to_string()), 
            wallet_private_key : None, 
            connection: None, 
            client: reqwest::Client::new(), 
            env: env::ENV::MAINNET,
        };

        let response = client.rest_create_order(
            1, 
            true, 
            2400.0, 
            0.01, 
            None
        ).await.unwrap(); 

        println!("Response: {:?}", response); 

        match response {
            RestResponse::CreateOrder { .. } => {}, 
            _ => {
                panic!("Not CreateOrder type: {:?}", response)
            }
        }
    }

    #[test(tokio::test)]
    async fn test_open_market_order() {
        let mut client = aevo::AevoClient {
            signing_key: Some(secret::SIGNING_KEY.to_string()), 
            wallet_address : Some(secret::WALLET_ADDRESS.to_string()), 
            api_key : Some(secret::API_KEY.to_string()),
            api_secret : Some(secret::API_SECRET.to_string()), 
            wallet_private_key : None, 
            connection: None, 
            client: reqwest::Client::new(), 
            env: env::ENV::MAINNET,
        };

        let response = client.rest_create_market_order(
            1, 
            true,
            0.01,
        ).await.unwrap(); 

        println!("Response: {:?}", response); 

        match response {
            RestResponse::CreateOrder { .. } => {}, 
            _ => {
                panic!("Not CreateOrder type: {:?}", response)
            }
        }
    }

    #[test(tokio::test)]
    async fn test_cancel_all_orders() {
        let mut client = aevo::AevoClient {
            signing_key: Some(secret::SIGNING_KEY.to_string()), 
            wallet_address : Some(secret::WALLET_ADDRESS.to_string()), 
            api_key : Some(secret::API_KEY.to_string()),
            api_secret : Some(secret::API_SECRET.to_string()), 
            wallet_private_key : None, 
            connection: None, 
            client: reqwest::Client::new(), 
            env: env::ENV::MAINNET,
        };

        let response = client.rest_cancel_all_orders(
            None, 
            None
        ).await.unwrap(); 

        println!("Response: {:?}", response); 

        match response {
            RestResponse::DeleteOrdersAll { .. } => {}, 
            _ => {
                panic!("Not CreateOrder type: {:?}", response)
            }
        }
    }

    #[test(tokio::test)]
    async fn test_subscribe_index() {
        let mut client = aevo::AevoClient {
            signing_key: Some(secret::SIGNING_KEY.to_string()), 
            wallet_address : Some(secret::WALLET_ADDRESS.to_string()), 
            api_key : Some(secret::API_KEY.to_string()),
            api_secret : Some(secret::API_SECRET.to_string()), 
            wallet_private_key : None, 
            connection: None, 
            client: reqwest::Client::new(), 
            env: env::ENV::MAINNET,
        };

        client.open_connection().await.unwrap();

        let (tx, mut rx) = mpsc::unbounded_channel::<Message>(); 

        client.subscribe_index("ETH".to_string()).await.unwrap();

        let task1 = tokio::spawn(async move {
            client.read_messages(tx).await.unwrap()
        });

        let task2 = tokio::spawn(async move {
            loop {
                let msg = rx.recv().await; 
                match msg {
                    Some(data) => println!("The data: {:?}", data), 
                    None => {}
                }
            }
        });  

        join!(task1, task2); 
    }

    #[test(tokio::test)]
    async fn test_subscribe_fills() {
        let mut client = aevo::AevoClient {
            signing_key: Some(secret::SIGNING_KEY.to_string()), 
            wallet_address : Some(secret::WALLET_ADDRESS.to_string()), 
            api_key : Some(secret::API_KEY.to_string()),
            api_secret : Some(secret::API_SECRET.to_string()), 
            wallet_private_key : None, 
            connection: None, 
            client: reqwest::Client::new(), 
            env: env::ENV::MAINNET,
        };

        client.open_connection().await.unwrap();

        let (tx, mut rx) = mpsc::unbounded_channel::<Message>(); 

        client.subscribe_fills().await.unwrap();

        let task1 = tokio::spawn(async move {
            client.read_messages(tx).await.unwrap()
        });

        let task2 = tokio::spawn(async move {
            loop {
                let msg = rx.recv().await; 
                match msg {
                    Some(data) => println!("The data: {:?}", data), 
                    None => {}
                }
            }
        });  

        join!(task1, task2); 
    }

    #[test(tokio::test)]
    async fn test_subscribe_markprice() {
        let mut client = aevo::AevoClient {
            signing_key: Some(secret::SIGNING_KEY.to_string()), 
            wallet_address : Some(secret::WALLET_ADDRESS.to_string()), 
            api_key : Some(secret::API_KEY.to_string()),
            api_secret : Some(secret::API_SECRET.to_string()), 
            wallet_private_key : None, 
            connection: None, 
            client: reqwest::Client::new(), 
            env: env::ENV::MAINNET,
        };

        client.open_connection().await.unwrap();

        let (tx, mut rx) = mpsc::unbounded_channel::<Message>(); 

        client.subscribe_markprice("ETH".to_string()).await.unwrap();

        let task1 = tokio::spawn(async move {
            client.read_messages(tx).await.unwrap()
        });

        let task2 = tokio::spawn(async move {
            loop {
                let msg = rx.recv().await; 
                match msg {
                    Some(data) => println!("The data: {:?}", data), 
                    None => {}
                }
            }
        });  

        join!(task1, task2); 
    }

    #[test(tokio::test)]
    async fn test_subscribe_orderbook() {
        let mut client = aevo::AevoClient {
            signing_key: Some(secret::SIGNING_KEY.to_string()), 
            wallet_address : Some(secret::WALLET_ADDRESS.to_string()), 
            api_key : Some(secret::API_KEY.to_string()),
            api_secret : Some(secret::API_SECRET.to_string()), 
            wallet_private_key : None, 
            connection: None, 
            client: reqwest::Client::new(), 
            env: env::ENV::MAINNET,
        };

        client.open_connection().await.unwrap();

        let (tx, mut rx) = mpsc::unbounded_channel::<Message>(); 

        client.subscribe_orderbook("ETH".to_string()).await.unwrap();

        let task1 = tokio::spawn(async move {
            client.read_messages(tx).await.unwrap()
        });

        let task2 = tokio::spawn(async move {
            loop {
                let msg = rx.recv().await; 
                match msg {
                    Some(data) => println!("The data: {:?}", data), 
                    None => {}
                }
            }
        });  

        join!(task1, task2); 
    }

    #[test(tokio::test)]
    async fn test_ws_open_order() {
        let mut client = aevo::AevoClient {
            signing_key: Some(secret::SIGNING_KEY.to_string()), 
            wallet_address : Some(secret::WALLET_ADDRESS.to_string()), 
            api_key : Some(secret::API_KEY.to_string()),
            api_secret : Some(secret::API_SECRET.to_string()), 
            wallet_private_key : None, 
            connection: None, 
            client: reqwest::Client::new(), 
            env: env::ENV::MAINNET,
        };

        client.open_connection().await.unwrap();

        let order_id = client.create_order(
            1, 
            true, 
            2400.0, 
            0.01, 
            None, 
            None, 
            None
        ).await.unwrap(); 

        println!("Order id: {}", order_id); 
    }

    #[test(tokio::test)]
    async fn test_ws_cancel_order() {
        let mut client = aevo::AevoClient {
            signing_key: Some(secret::SIGNING_KEY.to_string()), 
            wallet_address : Some(secret::WALLET_ADDRESS.to_string()), 
            api_key : Some(secret::API_KEY.to_string()),
            api_secret : Some(secret::API_SECRET.to_string()), 
            wallet_private_key : None, 
            connection: None, 
            client: reqwest::Client::new(), 
            env: env::ENV::MAINNET,
        };

        client.open_connection().await.unwrap();

        client.cancel_order(
            "0xb6a96d4214697b20a904d2c38973a2a4e301a62a7b6981275c679595549acc43".to_string()
        ).await.unwrap(); 
    }
}
