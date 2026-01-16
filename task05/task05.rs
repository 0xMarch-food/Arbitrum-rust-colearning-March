use alloy::{
    primitives::address,
    providers::{Provider, ProviderBuilder},
    sol,
};
use eyre::Result;

// 使用 sol! 宏定义合约接口 
// #[sol(rpc)] 生成支持异步 RPC 调用的方法
sol! {
    #[sol(rpc)]
    interface IERC20 {
        function name() external view returns (string);
        function symbol() external view returns (string);
        function totalSupply() external view returns (uint256);
        function balanceOf(address account) external view returns (uint256);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // 连接到 Arbitrum Sepolia 测试网的公共 RPC 端点
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;
 
    // 创建一个提供者实例
    let provider = ProviderBuilder::new().connect_http(rpc_url); 

    // 实例化合约对象 (地址为 Arbitrum Sepolia WETH)
    let contract_address= address!("0x980B62Da83eFf3D4576C647993b0c1D7faf17c73");
    let contract = IERC20::new(contract_address, provider);

    // 调用只读方法 (使用 .call() 发起请求)
    let name = contract.name().call().await?;
    let symbol = contract.symbol().call().await?;
    let total_supply = contract.totalSupply().call().await?;

    println!("--- 合约信息 ---");
    println!("名称: {}", name);
    println!("符号: {}", symbol);
    println!("总供应量: {}", total_supply);

    // 读取特定状态 (查询一个随机地址的余额)
    let some_account = address!("0000000000000000000000000000000000000000");
    let balance = contract.balanceOf(some_account).call().await?;
    println!("零地址余额: {}", balance);

    Ok(())
}
