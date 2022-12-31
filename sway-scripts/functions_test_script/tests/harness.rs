use fuels::prelude::*;

script_abigen!(Script, "out/debug/functions_test_script-abi.json");

#[tokio::test]
async fn main_test() {
    let wallet = launch_provider_and_get_wallet().await;
    let bin_path = "out/debug/functions_test_script.bin";
    let instance = Script::new(wallet.clone(), bin_path);

    let response = instance.main().call().await.unwrap();

    let logs = response.get_logs().unwrap();
    println!("{:#?}", logs);
    let log_u64 = response.get_logs_with_type::<u64>().unwrap();
    println!("{:#?}", log_u64);
}
