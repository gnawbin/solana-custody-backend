//! 基本使用示例
//! 
//! 展示如何使用 sol-spl-token 模块实现需求：
//! 1. 用户注册时自动创建钱包
//! 2. 购买稳定币并转换为目标代币
//! 3. 将代币转账到外部钱包

use sol_spl_token::{
    config::SolanaConfig,
    wallet::{WalletManager, UserWallet},
    token::TokenManager,
    swap::SwapManager,
    error::Result,
};
use solana_sdk::signature::Keypair;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    println!("=== Solana SPL Token 集成示例 ===");
    
    // 1. 加载配置
    let config = SolanaConfig::default();
    println!("1. 加载配置完成");
    println!("   RPC URL: {}", config.rpc_url);
    println!("   网络: {}", config.network);
    
    // 2. 创建钱包管理器
    let wallet_manager = WalletManager::from_config(&config)?;
    println!("2. 钱包管理器创建完成");
    
    // 3. 为用户创建钱包（系统托管）
    let user_wallet = wallet_manager.create_user_wallet(100_000_000).await?; // 0.1 SOL
    println!("3. 用户钱包创建完成");
    println!("   钱包地址: {}", user_wallet.get_address());
    println!("   创建交易签名: {}", user_wallet.created_signature);
    
    // 4. 创建 Token 管理器
    let token_manager = TokenManager::from_config(&config);
    println!("4. Token 管理器创建完成");
    
    // 5. 获取稳定币和目标代币 mint 地址
    let stablecoin_mint = config.get_stablecoin_mint()?;
    let target_token_mint = config.get_target_token_mint().unwrap_or_else(|_| {
        // 如果没有配置目标代币，使用一个示例代币
        Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap()
    });
    
    println!("5. 代币配置:");
    println!("   稳定币 (USDC): {}", stablecoin_mint);
    println!("   目标代币: {}", target_token_mint);
    
    // 6. 创建关联 Token 账户
    let user_stablecoin_account = token_manager
        .create_associated_token_account_if_needed(
            &user_wallet.keypair,
            &user_wallet.pubkey(),
            &stablecoin_mint,
        )
        .await?;
    
    let user_target_token_account = token_manager
        .create_associated_token_account_if_needed(
            &user_wallet.keypair,
            &user_wallet.pubkey(),
            &target_token_mint,
        )
        .await?;
    
    println!("6. 关联 Token 账户创建完成");
    println!("   稳定币账户: {}", user_stablecoin_account);
    println!("   目标代币账户: {}", user_target_token_account);
    
    // 7. 创建交换管理器
    let swap_manager = SwapManager::from_config(&config);
    println!("7. 交换管理器创建完成");
    
    // 8. 模拟购买稳定币并转换为目标代币
    println!("8. 开始模拟代币交换流程...");
    
    // 假设用户有 0.05 SOL 要用于购买
    let sol_amount = 50_000_000; // 0.05 SOL
    
    // 获取 SOL mint 地址
    let sol_mint = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();
    
    // 执行两阶段交换：SOL -> USDC -> 目标代币
    let swap_results = swap_manager
        .auto_swap_to_target_token(
            &user_wallet.keypair,
            &sol_mint,
            &stablecoin_mint,
            &target_token_mint,
            sol_amount,
        )
        .await?;
    
    println!("9. 代币交换完成:");
    for (i, result) in swap_results.iter().enumerate() {
        println!("   阶段 {}:", i + 1);
        println!("     从: {} ({})", result.from_token, result.from_amount);
        println!("     到: {} ({})", result.to_token, result.to_amount);
        println!("     滑点: {:.2}%", result.slippage * 100.0);
        println!("     交易签名: {}", result.signature);
    }
    
    // 10. 将代币转账到外部钱包
    println!("10. 模拟代币转账到外部钱包...");
    
    // 创建外部钱包地址
    let external_wallet = Keypair::new();
    let external_address = external_wallet.pubkey();
    
    println!("   外部钱包地址: {}", external_address);
    
    // 获取目标代币余额（模拟）
    let target_token_balance = 1_000_000; // 假设有 1 个代币（考虑小数位数）
    let decimals = 6; // 假设代币有 6 位小数
    
    // 执行转账（这里只是演示，实际需要真正的代币余额）
    println!("   转账 {} 个目标代币到外部钱包", target_token_balance);
    println!("   （注意：这是演示，实际转账需要真正的代币余额）");
    
    // 11. 显示总结
    println!("\n=== 总结 ===");
    println!("✓ 用户钱包创建成功: {}", user_wallet.get_address());
    println!("✓ 关联 Token 账户已设置");
    println!("✓ 代币交换流程模拟完成");
    println!("✓ 代币转账功能就绪");
    println!("\n下一步:");
    println!("1. 配置真正的系统钱包私钥");
    println!("2. 设置目标代币 mint 地址");
    println!("3. 集成真正的 DEX（如 Jupiter API）");
    println!("4. 实现钱包和交易记录的持久化存储");
    
    Ok(())
}
