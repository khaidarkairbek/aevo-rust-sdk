use crate::aevo::{AevoClient, PRICE_DECIMALS, AMOUNT_DECIMALS};
use alloy::{hex::ToHexExt, primitives::{address, bytes, keccak256, Address, Sign, Signature, I256, U256}, signers::{local::{LocalSigner, PrivateKeySigner}, Signer}, sol}; 
use alloy::sol_types::Eip712Domain;
use eyre::{eyre, Result}; 
use alloy::sol_types::SolStruct;
use std::{collections::HashMap, str::FromStr};

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

impl AevoClient {
    pub async fn sign_order(
        &self, 
        instrument_id: u64, 
        is_buy: bool, 
        limit_price: Option<f64>, 
        quantity: f64, 
        timestamp: i64
    ) -> Result<(U256, String, String)> {
        let salt = U256::from(rand::random::<u64>());

        let price = match limit_price {
            Some(p) => U256::from((p * 10_i32.pow(PRICE_DECIMALS) as f64).floor()), 
            None => {
                if is_buy {
                    U256::MAX 
                } else {
                    U256::ZERO
                }
            }
        };

        let wallet_address: Address = self.credentials
            .as_ref()
            .ok_or_else(|| eyre!("Order sign error: Wallet address not set"))?
            .wallet_address
            .parse()?;  
        

        let signing_key = &self.credentials
            .as_ref()
            .ok_or_else(|| eyre!("Order sign error: Signing key not set"))?
            .signing_key; 

        let order = Order {
            maker: wallet_address, 
            isBuy: is_buy, 
            limitPrice: price, 
            amount: U256::from((quantity * 10_i32.pow(AMOUNT_DECIMALS) as f64).floor()), 
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
        let signer: PrivateKeySigner = signing_key.parse().expect("should parse private key"); 
        let signature: Signature = signer.sign_hash(&signable_bytes).await?;

        Ok((salt, format!("0x{}",signature.as_bytes().encode_hex()), format!("0x{}", signable_bytes.encode_hex())))
    }

    pub async fn sign_withdraw(
        &self, 
        collateral: String, 
        to: String, 
        amount: f64, 
        data: U256
    ) -> Result<(U256, String, String)>{
        let salt = U256::from(rand::random::<u64>());

        let signing_key = &self.credentials
            .as_ref()
            .ok_or_else(|| eyre!("Order sign error: Signing key not set"))?
            .signing_key; 

        let withdraw = Withdraw {
            collateral: collateral.parse()?, 
            to: to.parse()?, 
            amount: U256::from((amount * 10_i32.pow(AMOUNT_DECIMALS) as f64).floor()), 
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

        Ok((salt, format!("0x{}",signature.as_bytes().encode_hex()), format!("0x{}", signable_bytes.encode_hex())))
    }
}