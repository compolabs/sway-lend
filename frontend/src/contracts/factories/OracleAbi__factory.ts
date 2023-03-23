/* Autogenerated file. Do not edit manually. */

/* tslint:disable */
/* eslint-disable */

/*
  Fuels version: 0.35.0
  Forc version: 0.35.3
  Fuel-Core version: 0.17.3
*/

import { Interface, Contract } from "fuels";
import type { Provider, Account, AbstractAddress } from "fuels";
import type { OracleAbi, OracleAbiInterface } from "../OracleAbi";

const _abi = {
  "types": [
    {
      "typeId": 0,
      "type": "()",
      "components": [],
      "typeParameters": null
    },
    {
      "typeId": 1,
      "type": "(_, _)",
      "components": [
        {
          "name": "__tuple_element",
          "type": 8,
          "typeArguments": null
        },
        {
          "name": "__tuple_element",
          "type": 12,
          "typeArguments": null
        }
      ],
      "typeParameters": null
    },
    {
      "typeId": 2,
      "type": "b256",
      "components": null,
      "typeParameters": null
    },
    {
      "typeId": 3,
      "type": "generic T",
      "components": null,
      "typeParameters": null
    },
    {
      "typeId": 4,
      "type": "raw untyped ptr",
      "components": null,
      "typeParameters": null
    },
    {
      "typeId": 5,
      "type": "str[13]",
      "components": null,
      "typeParameters": null
    },
    {
      "typeId": 6,
      "type": "str[19]",
      "components": null,
      "typeParameters": null
    },
    {
      "typeId": 7,
      "type": "struct Address",
      "components": [
        {
          "name": "value",
          "type": 2,
          "typeArguments": null
        }
      ],
      "typeParameters": null
    },
    {
      "typeId": 8,
      "type": "struct ContractId",
      "components": [
        {
          "name": "value",
          "type": 2,
          "typeArguments": null
        }
      ],
      "typeParameters": null
    },
    {
      "typeId": 9,
      "type": "struct Price",
      "components": [
        {
          "name": "asset_id",
          "type": 8,
          "typeArguments": null
        },
        {
          "name": "price",
          "type": 12,
          "typeArguments": null
        },
        {
          "name": "last_update",
          "type": 12,
          "typeArguments": null
        }
      ],
      "typeParameters": null
    },
    {
      "typeId": 10,
      "type": "struct RawVec",
      "components": [
        {
          "name": "ptr",
          "type": 4,
          "typeArguments": null
        },
        {
          "name": "cap",
          "type": 12,
          "typeArguments": null
        }
      ],
      "typeParameters": [
        3
      ]
    },
    {
      "typeId": 11,
      "type": "struct Vec",
      "components": [
        {
          "name": "buf",
          "type": 10,
          "typeArguments": [
            {
              "name": "",
              "type": 3,
              "typeArguments": null
            }
          ]
        },
        {
          "name": "len",
          "type": 12,
          "typeArguments": null
        }
      ],
      "typeParameters": [
        3
      ]
    },
    {
      "typeId": 12,
      "type": "u64",
      "components": null,
      "typeParameters": null
    }
  ],
  "functions": [
    {
      "inputs": [
        {
          "name": "asset_id",
          "type": 8,
          "typeArguments": null
        }
      ],
      "name": "get_price",
      "output": {
        "name": "",
        "type": 9,
        "typeArguments": null
      },
      "attributes": [
        {
          "name": "storage",
          "arguments": [
            "read"
          ]
        }
      ]
    },
    {
      "inputs": [
        {
          "name": "owner",
          "type": 7,
          "typeArguments": null
        }
      ],
      "name": "initialize",
      "output": {
        "name": "",
        "type": 0,
        "typeArguments": null
      },
      "attributes": [
        {
          "name": "storage",
          "arguments": [
            "read",
            "write"
          ]
        }
      ]
    },
    {
      "inputs": [],
      "name": "owner",
      "output": {
        "name": "",
        "type": 7,
        "typeArguments": null
      },
      "attributes": [
        {
          "name": "storage",
          "arguments": [
            "read"
          ]
        }
      ]
    },
    {
      "inputs": [
        {
          "name": "asset_id",
          "type": 8,
          "typeArguments": null
        },
        {
          "name": "price",
          "type": 12,
          "typeArguments": null
        }
      ],
      "name": "set_price",
      "output": {
        "name": "",
        "type": 0,
        "typeArguments": null
      },
      "attributes": [
        {
          "name": "storage",
          "arguments": [
            "read",
            "write"
          ]
        }
      ]
    },
    {
      "inputs": [
        {
          "name": "prices",
          "type": 11,
          "typeArguments": [
            {
              "name": "",
              "type": 1,
              "typeArguments": null
            }
          ]
        }
      ],
      "name": "set_prices",
      "output": {
        "name": "",
        "type": 0,
        "typeArguments": null
      },
      "attributes": [
        {
          "name": "storage",
          "arguments": [
            "read",
            "write"
          ]
        }
      ]
    }
  ],
  "loggedTypes": [
    {
      "logId": 0,
      "loggedType": {
        "name": "",
        "type": 6,
        "typeArguments": null
      }
    },
    {
      "logId": 1,
      "loggedType": {
        "name": "",
        "type": 5,
        "typeArguments": null
      }
    },
    {
      "logId": 2,
      "loggedType": {
        "name": "",
        "type": 5,
        "typeArguments": null
      }
    }
  ],
  "messagesTypes": [],
  "configurables": []
}

export class OracleAbi__factory {
  static readonly abi = _abi
  static createInterface(): OracleAbiInterface {
    return new Interface(_abi) as unknown as OracleAbiInterface
  }
  static connect(
    id: string | AbstractAddress,
    accountOrProvider: Account | Provider
  ): OracleAbi {
    return new Contract(id, _abi, accountOrProvider) as unknown as OracleAbi
  }
}
