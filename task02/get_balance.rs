use alloy::providers::{Provider, ProviderBuilder}; 
use std::error::Error;
use alloy::primitives::Address;
use alloy::primitives::{utils::format_units, U256};
 
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // 连接到 Arbitrum Sepolia 测试网的公共 RPC 端点
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;
 
    // 创建一个提供者实例
    let provider = ProviderBuilder::new().connect_http(rpc_url); 

    // 查询账户余额
    let account_address: Address = "0x2AB1F21F0E4dC396c228a1b7b674440126710237".parse()?;
    let balance_wei = provider.get_balance(account_address).await?;
    println!("Balance of {:?} in wei is {}", account_address, balance_wei);

    // 从 wei 转换为 ETH
    let balance_ether = format_units(balance_wei, "ether")?;
    println!("Balance of {:?} in ETH is {}", account_address, balance_ether);
    
    Ok(())
}






