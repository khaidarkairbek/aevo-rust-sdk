mod aevo; 
mod secret;

#[cfg(test)]
mod tests {
    use aevo::RestHeaders;
    use test_log::test;

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
            env: aevo::ENV::MAINNET,
            rest_headers : RestHeaders::new(secret::API_KEY.to_string(), secret::API_SECRET.to_string()).unwrap(), 
            extra_headers : None
        };

        println!("{:?}", client.rest_get_account().await.unwrap()); 

        //client.open_connection().await.unwrap(); 
    }
}
