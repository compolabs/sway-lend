use dotenv::dotenv;
use fuels::prelude::*;

const RPC: &str = "node-beta-2.fuel.network";
script_abigen!(Script, "out/debug/functions_test_script-abi.json");

pub async fn setup_wallet() -> (WalletUnlocked, Provider) {
    let provider = match Provider::connect(RPC).await {
        Ok(p) => p,
        Err(error) => panic!("❌ Problem creating provider: {:#?}", error),
    };

    dotenv().ok();
    let secret = match std::env::var("SECRET") {
        Ok(s) => s,
        Err(error) => panic!("❌ Cannot find .env file: {:#?}", error),
    };

    let wallet =
        WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));

    return (wallet, provider);
}

#[tokio::test]
async fn main_test() {
    let wallet = launch_provider_and_get_wallet().await;
    let bin_path = "out/debug/functions_test_script.bin";
    let instance = Script::new(wallet.clone(), bin_path);

    let response = instance.main().simulate().await.unwrap();

    let logs = response.get_logs().unwrap();
    println!("{:#?}", logs);
    let log_u64 = response.get_logs_with_type::<u64>().unwrap();
    println!("{:#?}", log_u64);
}
