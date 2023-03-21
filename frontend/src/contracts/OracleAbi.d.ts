/* Autogenerated file. Do not edit manually. */

/* tslint:disable */
/* eslint-disable */

/*
  Fuels version: 0.35.0
  Forc version: 0.35.3
  Fuel-Core version: 0.17.3
*/

import type {
  BigNumberish,
  BN,
  BytesLike,
  Contract,
  DecodedValue,
  FunctionFragment,
  Interface,
  InvokeFunction,
} from 'fuels';

import type { Vec } from "./common";

export type AddressInput = { value: string };
export type AddressOutput = AddressInput;
export type ContractIdInput = { value: string };
export type ContractIdOutput = ContractIdInput;
export type PriceInput = { asset_id: ContractIdInput, price: BigNumberish, last_update: BigNumberish };
export type PriceOutput = { asset_id: ContractIdOutput, price: BN, last_update: BN };

interface OracleAbiInterface extends Interface {
  functions: {
    get_price: FunctionFragment;
    initialize: FunctionFragment;
    owner: FunctionFragment;
    set_price: FunctionFragment;
    set_prices: FunctionFragment;
  };

  encodeFunctionData(functionFragment: 'get_price', values: [ContractIdInput]): Uint8Array;
  encodeFunctionData(functionFragment: 'initialize', values: [AddressInput]): Uint8Array;
  encodeFunctionData(functionFragment: 'owner', values: []): Uint8Array;
  encodeFunctionData(functionFragment: 'set_price', values: [ContractIdInput, BigNumberish]): Uint8Array;
  encodeFunctionData(functionFragment: 'set_prices', values: [Vec<[ContractIdInput, BigNumberish]>]): Uint8Array;

  decodeFunctionData(functionFragment: 'get_price', data: BytesLike): DecodedValue;
  decodeFunctionData(functionFragment: 'initialize', data: BytesLike): DecodedValue;
  decodeFunctionData(functionFragment: 'owner', data: BytesLike): DecodedValue;
  decodeFunctionData(functionFragment: 'set_price', data: BytesLike): DecodedValue;
  decodeFunctionData(functionFragment: 'set_prices', data: BytesLike): DecodedValue;
}

export class OracleAbi extends Contract {
  interface: OracleAbiInterface;
  functions: {
    get_price: InvokeFunction<[asset_id: ContractIdInput], PriceOutput>;
    initialize: InvokeFunction<[owner: AddressInput], void>;
    owner: InvokeFunction<[], AddressOutput>;
    set_price: InvokeFunction<[asset_id: ContractIdInput, price: BigNumberish], void>;
    set_prices: InvokeFunction<[prices: Vec<[ContractIdInput, BigNumberish]>], void>;
  };
}
