use alloy::primitives::U256;
#[derive(Debug)]
pub enum ENV {
    MAINNET, 
    TESTNET
}
#[derive(Debug)]
pub struct Addresses {
    pub l1_bridge : String, 
    pub l1_usdc : String, 
    pub l2_withdraw_proxy : String, 
    pub l2_usdc : String
}
#[derive(Debug)]
pub struct Config {
    pub rest_url : String, 
    pub ws_url : String, 
    pub signing_domain : SigningDomain
}
#[derive(Debug)]
pub struct SigningDomain {
    pub name: String, 
    pub version : String, 
    pub chain_id : U256
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