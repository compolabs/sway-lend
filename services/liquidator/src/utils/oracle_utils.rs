use fuels::prelude::{abigen, WalletUnlocked};

abigen!(Contract(
    name = "OracleContract",
    abi = "src/artefacts/oracle-abi.json"
));

pub mod oracle_abi_calls {

    use fuels::types::Bits256;

    use super::*;

    pub async fn get_price(contract: &OracleContract<WalletUnlocked>, asset_id: Bits256) -> Price {
        contract
            .methods()
            .get_price(asset_id)
            .simulate()
            .await
            .unwrap()
            .value
    }
}
