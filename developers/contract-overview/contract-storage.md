---
description: >-
  Market Storage is a crucial component of our contract that serves as a central
  repository for all data related to the contract's mathematical operations.
---

# Contract storage

### config: Option\<MarketConfiguration> = Option::None

The `config` entity is used to store the market configuration for each individual market. It starts out as `None`, but you can initialize it by calling the `initialize` method and passing in an object of the `MarketConfiguration` type. This object contains information such as the addresses of the administrators who manage the market, the token that can be supplied and borrowed, and details about the pricing, interest rates, rewards, and more.&#x20;

#### The `MarketConfiguration` object has several fields, including:

* **governor** ([Address](https://fuellabs.github.io/fuels-rs/v0.35.1/types/address.html)): The address of the admin who can manage the market.
* **pause\_guardian** ([Address](https://fuellabs.github.io/fuels-rs/v0.35.1/types/address.html)): The address of the admin who can pause the market.
* **base\_token** ([ContractId](https://fuellabs.github.io/fuels-rs/v0.35.1/types/contract-id.html)): The token that can be supplied and borrowed.
* **base\_token\_decimals** (u8): The decimal of the base token.
* **base\_token\_price\_feed** ([ContractId](https://fuellabs.github.io/fuels-rs/v0.35.1/types/contract-id.html)): The address of the price oracle contract where you can check the base token's market price in USD.
* **kink** (u64): The value that shows the optimal disposal.
* **supply\_per\_second\_interest\_rate\_slope\_low** (u64): The coefficient of dependence of the deposit rate on utilization every second if utilization is below optimal.
* **supply\_per\_second\_interest\_rate\_slope\_high** (u64): The coefficient of dependence of the deposit rate on utilization every second if utilization is higher than optimal.
* **borrow\_per\_second\_interest\_rate\_slope\_low** (u64): The coefficient of dependence of the loan rate on utilization every second if utilization is below optimal.
* **borrow\_per\_second\_interest\_rate\_slope\_high** (u64): The coefficient of dependence of the loan rate on utilization every second if utilization is higher than optimal.
* **borrow\_per\_second\_interest\_rate\_base** (u64): The minimum monthly loan rate.
* **store\_front\_price\_factor** (u64): The share of the elimination of the penalty that the liquidator receives (and the rest remains on the score sheet as a protective reserve).
* **base\_tracking\_supply\_speed** (u64): The amount of rewards (liquidity mining) we accrue per second for the entire supply.
* **base\_tracking\_borrow\_speed** (u64): The amount of rewards (liquidity mining) we charge per second for the entire borrow.
* **base\_min\_for\_rewards** (u64): The minimum amount at which rewards are accrued, with the same decimal as the base asset.
* **base\_borrow\_min** (u64): The minimal value of the base borrow amount, with the same decimal as the base asset.
* **target\_reserves** (u64): The maximum number of protective reserves at which the sale of collateral occurs during liquidation.
* **reward\_token** ([ContractId](https://fuellabs.github.io/fuels-rs/v0.35.1/types/contract-id.html)): The address of the token that is used to pay mining rewards.
* **asset\_configs** (Vec<[AssetConfig](contract-storage.md#the-assetconfig-object-has-several-fields-including)>): A vector value that contains the configuration of collateral assets.

#### The `AssetConfig` object has several fields, including:

AssetConfig is used to store the configuration values for each individual collateral token in the market. It contains the following fields:

* **asset** ([ContractId](https://fuellabs.github.io/fuels-rs/v0.35.1/types/contract-id.html)): The address of the token that can be used as collateral.
* **price\_feed** ([ContractId](https://fuellabs.github.io/fuels-rs/v0.35.1/types/contract-id.html)): The address of the price oracle contract where you can check the token's market price in USD.
* **decimals** (u8): The decimal of the token.
* **borrow\_collateral\_factor** (u64): The amount you can borrow relative to the dollar value of the collateral asset.
* **liquidate\_collateral\_factor** (u64): The ratio of the dollar value of the underlying asset to the dollar value of the collateral asset at which the debt will be liquidated.
* **liquidation\_penalty** (u64): The amount of collateral that will be retained upon liquidation.
* **supply\_cap** (u64): The maximum number of supply tokens per protocol.
