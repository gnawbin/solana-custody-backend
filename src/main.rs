
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::CommitmentConfig;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    signer::keypair::read_keypair_file,
    transaction::Transaction,
};
use spl_token::instruction::{transfer, transfer_checked};
use spl_associated_token_account::{
    get_associated_token_address,
    instruction::create_associated_token_account,
};
use std::str::FromStr;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use anyhow::{Context, anyhow};

mod config;

// ==================== 配置区 ====================
// 改成你的代币 Mint 地址
const TOKEN_MINT: &str = "YOUR_TOKEN_MINT_HERE11111111111111111111111111111111";
// RPC（开发先用 devnet）
const RPC_URL: &str = "https://api.devnet.solana.com";
// 空投用的资金钱包（放点代币就行）
const TREASURY_KEYPAIR_PATH: &str = "treasury.json"; // 放项目根目录
// ==================== 全局状态 ====================
type UserId = u64;
static WALLETS: once_cell::sync::Lazy<Arc<Mutex<HashMap<UserId, Keypair>>>> =
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));
fn client() -> RpcClient {
    RpcClient::new(RPC_URL.to_string())
}
// ==================== 核心函数 ====================
///1&2 为用户创建/获取托管钱包
pub fn get_or_create_wallet(user_id: UserId)->Keypair{
    let mut map=WALLETS.lock().unwrap();
    map.entry(user_id).or_insert_with(Keypair::new).insecure_clone()
}
/// 3 自动空投代币
pub async fn airdrop_to_user(user_id: UserId,amount_ui: f64)->anyhow::Result<String>{
    let client = client();
    let treasury = read_keypair_file(TREASURY_KEYPAIR_PATH)
        .map_err(|e| anyhow!("读取资金库密钥文件失败: {}", e))?;
    let mint = Pubkey::from_str(TOKEN_MINT)?;
    let user_kp = get_or_create_wallet(user_id);

    // 创建用户 ATA（如果不存在）
    let user_ata = get_or_create_ata(&client, &treasury, &user_kp.pubkey(), &mint).await?;

    // 资金库 ATA
    let treasury_ata = get_or_create_ata(&client, &treasury, &treasury.pubkey(), &mint).await?;

    let decimals = client.get_token_supply(&mint)?.decimals; // 自动获取小数位
    let amount = (amount_ui * 10u64.pow(decimals as u32) as f64) as u64;

    let ix = transfer_checked(
        &spl_token::id(),
        &treasury_ata,
        &mint,
        &user_ata,
        &treasury.pubkey(),
        &[],
        amount,
        decimals,
    )?;

    let recent = client.get_latest_blockhash()?;
    let mut tx = Transaction::new_with_payer(&[ix], Some(&treasury.pubkey()));
    tx.sign(&[&treasury], recent);
    let sig = client.send_and_confirm_transaction(&tx)?;
    Ok(sig.to_string())
}
/// 4 用户自己转账给任意地址
pub async fn user_send_from_user(
    user_id: UserId,
    to_address: &str,
    amount_ui: f64,
) -> anyhow::Result<String> {
    let client = client();
    let mint = Pubkey::from_str(TOKEN_MINT)?;
    let user_kp = get_or_create_wallet(user_id);

    let user_ata = get_or_create_ata(&client, &user_kp, &user_kp.pubkey(), &mint).await?;
    let to_pubkey = Pubkey::from_str(to_address)?;
    let to_ata = get_or_create_ata(&client, &user_kp, &to_pubkey, &mint).await?;

    let decimals = client.get_token_supply(&mint)?.decimals;
    let amount = (amount_ui * 10u64.pow(decimals as u32) as f64) as u64;

    let ix = transfer_checked(
        &spl_token::id(),
        &user_ata,
        &mint,
        &to_ata,
        &user_kp.pubkey(),
        &[],
        amount,
        decimals,
    )?;

    let recent = client.get_latest_blockhash()?;
    let mut tx = Transaction::new_with_payer(&[ix], Some(&user_kp.pubkey()));
    tx.sign(&[&user_kp], recent);

    let sig = client.send_and_confirm_transaction(&tx)?;
    Ok(sig.to_string())
}
/// 获取或创建关联代币账户
async fn get_or_create_ata(
    client: &RpcClient,
    payer: &Keypair,
    owner: &Pubkey,
    mint: &Pubkey,
) -> anyhow::Result<Pubkey> {
    let ata = get_associated_token_address(owner, mint);
    // 检查账户是否存在
    let account = client.get_account_with_commitment(&ata, CommitmentConfig::confirmed())?;
    if let Some(acc) = account.value {
        if acc.owner == spl_token::id() {
            return Ok(ata);
        }
    }
    // 创建账户
    let ix = create_associated_token_account(&payer.pubkey(), owner, mint, &spl_token::id());
    let recent_blockhash = client.get_latest_blockhash()?;
    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[payer], recent_blockhash);
    client.send_and_confirm_transaction(&tx)?;
    Ok(ata)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let user_id = 12345;

    // 1&2 获取用户钱包地址
    let wallet = get_or_create_wallet(user_id);
    println!("用户钱包地址: {}", wallet.pubkey());

    // 3 空投 100 个代币
    let sig1 = airdrop_to_user(user_id, 100.0).await?;
    println!("空投成功: https://explorer.solana.com/tx/{}?cluster=devnet", sig1);

    // 4 用户转 30 个代币给别人
    let sig2 = user_send_from_user(
        user_id,
        "888888888888888888888888888888888888888888888888", // 随便填接收地址
        30.0,
    ).await?;
    println!("转账成功: https://explorer.solana.com/tx/{}?cluster=devnet", sig2);

    Ok(())
}
