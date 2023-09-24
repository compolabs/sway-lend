use self::abigen_bindings::oracle_contract_mod;
use fuels::prelude::{abigen, WalletUnlocked};

abigen!(Contract(
    name = "OracleContract",
    abi = "src/artefacts/oracle-abi.json"
));

pub mod oracle_abi_calls {

    use fuels::types::Bits256;

    use super::*;

    pub async fn get_price(contract: &OracleContract<WalletUnlocked>, asset_id: Bits256) -> Price {
        println!(
            "asset_id = {:?}",
            fuels::types::AssetId::from(asset_id.0).to_string()
        );
        let asset_id = oracle_contract_mod::AssetId { value: asset_id };
        contract
            .methods()
            .get_price(asset_id)
            .simulate()
            .await
            .unwrap()
            .value
    }
}
