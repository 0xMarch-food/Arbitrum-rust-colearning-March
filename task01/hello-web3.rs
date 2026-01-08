use alloy::providers::{Provider, ProviderBuilder}; 
use std::error::Error;
use alloy::primitives::Address;
use alloy::sol;

sol! { 
   #[sol(rpc)] 
   contract HelloWeb3 { 
        function hello_web3() pure public returns(string memory); 
   } 
} 
 
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, web3!");

    // Set up the HTTP transport which is consumed by the RPC client.
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;
 
    // Create a provider with the HTTP transport using the `reqwest` crate.
    let provider = ProviderBuilder::new().connect_http(rpc_url); 

    // Fetch and print the latest block number.
    let latest_block = provider.get_block_number().await?;
    println!("Latest block: {}", latest_block);

    // Interact with the smart contract.
    let contract_address: Address = "0x3f1f78ED98Cd180794f1346F5bD379D5Ec47DE90".parse()?;
    let contract = HelloWeb3::new(contract_address, provider);
    let result = contract.hello_web3().call().await?;
    println!("合约返回: {}", result);
    
    Ok(())
}