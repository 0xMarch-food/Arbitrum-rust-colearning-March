use alloy::{
    primitives::{utils::format_units, U256},
    providers::{Provider, ProviderBuilder},
};
use eyre::Result;

//Gas费 = Gas 价格 × Gas 限额
fn calculate_transfer_gas_fee(price: u128, limit: u128) -> U256 {
    U256::from(price * limit)
}

#[tokio::main]
async fn main() -> Result<()> {
    // 连接到 Arbitrum Sepolia 测试网的公共 RPC 端点
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;
 
    // 创建一个提供者实例
    let provider = ProviderBuilder::new().connect_http(rpc_url); 

    // 获取当前Gas价格
    let gas_price = provider.get_gas_price().await?;
    println!("1.The gas price of Arbitrum Sepolia is {} ETH", format_units(gas_price, "ether")?);

    // 3. 定义基础转账参数
    // 普通ETH转账的Gas Limit行业通用值为21,000
    let base_gas_limit: u128 = 21_000;

    // 4. 编写预估函数
    let estimated_fee = calculate_transfer_gas_fee(gas_price, base_gas_limit);


    println!("2.行业通用值gas limit是 {}", base_gas_limit);
    println!("3.预估转账gas费为 {} ETH", format_units(estimated_fee, "ether")?);

    Ok(())
}

