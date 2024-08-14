pub mod aevo; 
pub mod env; 
pub mod signature; 
pub mod rest; 
pub mod ws_structs;

#[cfg(test)]
mod tests {
    use aevo::{AevoClient, ClientCredentials};
    use log::info;
    use rest::RestResponse;
    use test_log::test;
    use tokio::{join, sync::mpsc};
    use tokio_tungstenite::tungstenite::Message;
    use ws_structs::WsResponse;
    use super::*;

    #[test(tokio::test)]
    async fn test_open_connection() { 
        let credentials = ClientCredentials {
            signing_key : std::env::var("SIGNING_KEY").unwrap(), 
            wallet_address : std::env::var("WALLET_ADDRESS").unwrap(), 
            api_secret : std::env::var("API_SECRET").unwrap(), 
            api_key : std::env::var("API_KEY").unwrap(), 
            wallet_private_key : None
        };
        
        let mut client = AevoClient::new(Some(credentials), env::ENV::MAINNET).await.unwrap(); 

    }

    #[test(tokio::test)]
    async fn test_get_index() {
        let credentials = ClientCredentials {
            signing_key : std::env::var("SIGNING_KEY").unwrap(), 
            wallet_address : std::env::var("WALLET_ADDRESS").unwrap(), 
            api_secret : std::env::var("API_SECRET").unwrap(), 
            api_key : std::env::var("API_KEY").unwrap(), 
            wallet_private_key : None
        };
        
        let mut client = AevoClient::new(Some(credentials), env::ENV::MAINNET).await.unwrap(); 

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
        let credentials = ClientCredentials {
            signing_key : std::env::var("SIGNING_KEY").unwrap(), 
            wallet_address : std::env::var("WALLET_ADDRESS").unwrap(), 
            api_secret : std::env::var("API_SECRET").unwrap(), 
            api_key : std::env::var("API_KEY").unwrap(), 
            wallet_private_key : None
        };
        
        let mut client = AevoClient::new(Some(credentials), env::ENV::MAINNET).await.unwrap(); 

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
        let credentials = ClientCredentials {
            signing_key : std::env::var("SIGNING_KEY").unwrap(), 
            wallet_address : std::env::var("WALLET_ADDRESS").unwrap(), 
            api_secret : std::env::var("API_SECRET").unwrap(), 
            api_key : std::env::var("API_KEY").unwrap(), 
            wallet_private_key : None
        };
        
        let mut client = AevoClient::new(Some(credentials), env::ENV::MAINNET).await.unwrap(); 

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
        let credentials = ClientCredentials {
            signing_key : std::env::var("SIGNING_KEY").unwrap(), 
            wallet_address : std::env::var("WALLET_ADDRESS").unwrap(), 
            api_secret : std::env::var("API_SECRET").unwrap(), 
            api_key : std::env::var("API_KEY").unwrap(), 
            wallet_private_key : None
        };
        
        let mut client = AevoClient::new(Some(credentials), env::ENV::MAINNET).await.unwrap(); 

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
        let credentials = ClientCredentials {
            signing_key : std::env::var("SIGNING_KEY").unwrap(), 
            wallet_address : std::env::var("WALLET_ADDRESS").unwrap(), 
            api_secret : std::env::var("API_SECRET").unwrap(), 
            api_key : std::env::var("API_KEY").unwrap(), 
            wallet_private_key : None
        };
        
        let mut client = AevoClient::new(Some(credentials), env::ENV::MAINNET).await.unwrap(); 

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
        let credentials = ClientCredentials {
            signing_key : std::env::var("SIGNING_KEY").unwrap(), 
            wallet_address : std::env::var("WALLET_ADDRESS").unwrap(), 
            api_secret : std::env::var("API_SECRET").unwrap(), 
            api_key : std::env::var("API_KEY").unwrap(), 
            wallet_private_key : None
        };
        
        let mut client = AevoClient::new(Some(credentials), env::ENV::MAINNET).await.unwrap(); 

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
        let credentials = ClientCredentials {
            signing_key : std::env::var("SIGNING_KEY").unwrap(), 
            wallet_address : std::env::var("WALLET_ADDRESS").unwrap(), 
            api_secret : std::env::var("API_SECRET").unwrap(), 
            api_key : std::env::var("API_KEY").unwrap(), 
            wallet_private_key : None
        };
        
        let mut client = AevoClient::new(Some(credentials), env::ENV::MAINNET).await.unwrap(); 

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
        let credentials = ClientCredentials {
            signing_key : std::env::var("SIGNING_KEY").unwrap(), 
            wallet_address : std::env::var("WALLET_ADDRESS").unwrap(), 
            api_secret : std::env::var("API_SECRET").unwrap(), 
            api_key : std::env::var("API_KEY").unwrap(), 
            wallet_private_key : None
        };
        
        let mut client = AevoClient::new(Some(credentials), env::ENV::MAINNET).await.unwrap(); 

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
        let credentials = ClientCredentials {
            signing_key : std::env::var("SIGNING_KEY").unwrap(), 
            wallet_address : std::env::var("WALLET_ADDRESS").unwrap(), 
            api_secret : std::env::var("API_SECRET").unwrap(), 
            api_key : std::env::var("API_KEY").unwrap(), 
            wallet_private_key : None
        };
        
        let mut client = AevoClient::new(Some(credentials), env::ENV::MAINNET).await.unwrap(); 

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
        let credentials = ClientCredentials {
            signing_key : std::env::var("SIGNING_KEY").unwrap(), 
            wallet_address : std::env::var("WALLET_ADDRESS").unwrap(), 
            api_secret : std::env::var("API_SECRET").unwrap(), 
            api_key : std::env::var("API_KEY").unwrap(), 
            wallet_private_key : None
        };
        
        let mut client = AevoClient::new(Some(credentials), env::ENV::MAINNET).await.unwrap();

        let (tx, mut rx) = mpsc::unbounded_channel::<WsResponse>(); 

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
        let credentials = ClientCredentials {
            signing_key : std::env::var("SIGNING_KEY").unwrap(), 
            wallet_address : std::env::var("WALLET_ADDRESS").unwrap(), 
            api_secret : std::env::var("API_SECRET").unwrap(), 
            api_key : std::env::var("API_KEY").unwrap(), 
            wallet_private_key : None
        };
        
        let mut client = AevoClient::new(Some(credentials), env::ENV::MAINNET).await.unwrap(); 

        let (tx, mut rx) = mpsc::unbounded_channel::<WsResponse>(); 

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
    async fn test_subscribe_orderbook() {
        let credentials = ClientCredentials {
            signing_key : std::env::var("SIGNING_KEY").unwrap(), 
            wallet_address : std::env::var("WALLET_ADDRESS").unwrap(), 
            api_secret : std::env::var("API_SECRET").unwrap(), 
            api_key : std::env::var("API_KEY").unwrap(), 
            wallet_private_key : None
        };
        
        let mut client = AevoClient::new(Some(credentials), env::ENV::MAINNET).await.unwrap(); 

        let (tx, mut rx) = mpsc::unbounded_channel::<WsResponse>(); 

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
        let credentials = ClientCredentials {
            signing_key : std::env::var("SIGNING_KEY").unwrap(), 
            wallet_address : std::env::var("WALLET_ADDRESS").unwrap(), 
            api_secret : std::env::var("API_SECRET").unwrap(), 
            api_key : std::env::var("API_KEY").unwrap(), 
            wallet_private_key : None
        };
        
        let mut client = AevoClient::new(Some(credentials), env::ENV::MAINNET).await.unwrap(); 

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

        let (tx, mut rx) = mpsc::unbounded_channel::<WsResponse>();

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
    async fn test_ws_cancel_order() {
        let credentials = ClientCredentials {
            signing_key : std::env::var("SIGNING_KEY").unwrap(), 
            wallet_address : std::env::var("WALLET_ADDRESS").unwrap(), 
            api_secret : std::env::var("API_SECRET").unwrap(), 
            api_key : std::env::var("API_KEY").unwrap(), 
            wallet_private_key : None
        };
        
        let mut client = AevoClient::new(Some(credentials), env::ENV::MAINNET).await.unwrap(); 

        client.cancel_order(
            "0x3dbf007fc71ca02327fee4591e5a1f1fce63dc3f97d916ecfd887c46745a2820".to_string()
        ).await.unwrap(); 
    }
}
