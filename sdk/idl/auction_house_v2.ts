export type AuctionHouseV2 = {
  "version": "0.1.0",
  "name": "auction_house_v2",
  "instructions": [
    {
      "name": "create",
      "accounts": [
        {
          "name": "auctionHouse",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "treasuryMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "treasuryAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryWithdrawalAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryWithdrawalOwner",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "feeAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "feeWithdrawalAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "sellerFeeBasisPoints",
          "type": "u16"
        },
        {
          "name": "requiresSignOff",
          "type": "bool"
        }
      ]
    },
    {
      "name": "sell",
      "accounts": [
        {
          "name": "auctionHouse",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "auctionHouseAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "treasuryMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "treeConfig",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "merkeTree",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "previousLeafDelegate",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "sellerTradeState",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "assetId",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "auctionHouseFeeAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "programAsSigner",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "bubblegumProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "compressionProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "logWrapper",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "sellerPrice",
          "type": "u64"
        },
        {
          "name": "root",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "dataHash",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "creatorHash",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "nonce",
          "type": "u64"
        },
        {
          "name": "index",
          "type": "u32"
        }
      ]
    },
    {
      "name": "bid",
      "accounts": [
        {
          "name": "auctionHouse",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "auctionHouseAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "treasuryMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "bidder",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "paymentAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "assetId",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "buyerEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerTradeState",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "auctionHouseFeeAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "buyerPrice",
          "type": "u64"
        }
      ]
    },
    {
      "name": "executeSale",
      "accounts": [
        {
          "name": "auctionHouse",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "auctionHouseAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "treasuryMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "treasuryAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treeConfig",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "seller",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerReceiptAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "merkeTree",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerTradeState",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyer",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerTradeState",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "assetId",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "auctionHouseFeeAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "programAsSigner",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "bubblegumProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "compressionProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "logWrapper",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "buyerPrice",
          "type": "u64"
        },
        {
          "name": "root",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "dataHash",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "creatorHash",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "nonce",
          "type": "u64"
        },
        {
          "name": "index",
          "type": "u32"
        },
        {
          "name": "royaltyBasisPoints",
          "type": "u16"
        },
        {
          "name": "metadata",
          "type": {
            "defined": "MetadataArgs"
          }
        }
      ]
    },
    {
      "name": "cancel",
      "accounts": [
        {
          "name": "auctionHouse",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "treasuryMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "assetId",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tradeState",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "sellerPrice",
          "type": "u64"
        },
        {
          "name": "root",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "dataHash",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "creatorHash",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "nonce",
          "type": "u64"
        },
        {
          "name": "index",
          "type": "u32"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "auctionHouseV2Data",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "treasuryMint",
            "type": "publicKey"
          },
          {
            "name": "sellerFeeBasisPoints",
            "type": "u16"
          },
          {
            "name": "requiresSignOff",
            "type": "bool"
          },
          {
            "name": "treasuryAccount",
            "type": "publicKey"
          },
          {
            "name": "treasuryWithdrawalAccount",
            "type": "publicKey"
          },
          {
            "name": "feeAccount",
            "type": "publicKey"
          },
          {
            "name": "feeWithdrawalAccount",
            "type": "publicKey"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "treasuryBump",
            "type": "u8"
          },
          {
            "name": "feeAccountBump",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "Uses",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "useMethod",
            "type": {
              "defined": "UseMethod"
            }
          },
          {
            "name": "remaining",
            "type": "u64"
          },
          {
            "name": "total",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "Collection",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "verified",
            "type": "bool"
          },
          {
            "name": "key",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "Creator",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "address",
            "type": "publicKey"
          },
          {
            "name": "verified",
            "type": "bool"
          },
          {
            "name": "share",
            "docs": [
              "The percentage share.",
              "",
              "The value is a percentage, not basis points."
            ],
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "MetadataArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "docs": [
              "The name of the asset"
            ],
            "type": "string"
          },
          {
            "name": "symbol",
            "docs": [
              "The symbol for the asset"
            ],
            "type": "string"
          },
          {
            "name": "uri",
            "docs": [
              "URI pointing to JSON representing the asset"
            ],
            "type": "string"
          },
          {
            "name": "sellerFeeBasisPoints",
            "docs": [
              "Royalty basis points that goes to creators in secondary sales (0-10000)"
            ],
            "type": "u16"
          },
          {
            "name": "primarySaleHappened",
            "type": "bool"
          },
          {
            "name": "isMutable",
            "type": "bool"
          },
          {
            "name": "editionNonce",
            "docs": [
              "nonce for easy calculation of editions, if present"
            ],
            "type": {
              "option": "u8"
            }
          },
          {
            "name": "tokenStandard",
            "docs": [
              "Since we cannot easily change Metadata, we add the new DataV2 fields here at the end."
            ],
            "type": {
              "option": {
                "defined": "TokenStandard"
              }
            }
          },
          {
            "name": "collection",
            "docs": [
              "Collection"
            ],
            "type": {
              "option": {
                "defined": "Collection"
              }
            }
          },
          {
            "name": "uses",
            "docs": [
              "Uses"
            ],
            "type": {
              "option": {
                "defined": "Uses"
              }
            }
          },
          {
            "name": "tokenProgramVersion",
            "type": {
              "defined": "TokenProgramVersion"
            }
          },
          {
            "name": "creators",
            "type": {
              "vec": {
                "defined": "Creator"
              }
            }
          }
        ]
      }
    },
    {
      "name": "TokenStandard",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "NonFungible"
          },
          {
            "name": "FungibleAsset"
          },
          {
            "name": "Fungible"
          },
          {
            "name": "NonFungibleEdition"
          }
        ]
      }
    },
    {
      "name": "TokenProgramVersion",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Original"
          },
          {
            "name": "Token2022"
          }
        ]
      }
    },
    {
      "name": "UseMethod",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Burn"
          },
          {
            "name": "Multiple"
          },
          {
            "name": "Single"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "BumpSeedNotInHashMap",
      "msg": "Bump Seed Not In HashMap"
    },
    {
      "code": 6001,
      "name": "AccountNotInitialized",
      "msg": "Account not initialized"
    },
    {
      "code": 6002,
      "name": "InvalidSellerFeeBasisPoints",
      "msg": "Invalid Seller Fee Basis Points"
    },
    {
      "code": 6003,
      "name": "NumericOverflow",
      "msg": "Numeric Overflow"
    },
    {
      "code": 6004,
      "name": "NotEnoughFunds",
      "msg": "Not Enough Funds"
    },
    {
      "code": 6005,
      "name": "InvalidBuyingOrSellingOrder",
      "msg": "Invalid buying or selling order"
    },
    {
      "code": 6006,
      "name": "InvalidBuyingOrderPrice",
      "msg": "Invalid buying order price don't match with selling order"
    },
    {
      "code": 6007,
      "name": "MetadataHashMismatch",
      "msg": "Metadata hash does not match"
    },
    {
      "code": 6008,
      "name": "InvalidBuyerTradeState",
      "msg": "Buyer trade state is invalid"
    },
    {
      "code": 6009,
      "name": "BothPartiesNeedToAgreeToSale",
      "msg": "Both parties need to agree on a price"
    },
    {
      "code": 6010,
      "name": "InvalidSellerTradeState",
      "msg": "Seller trade is invalid"
    },
    {
      "code": 6011,
      "name": "PublicKeyMismatch",
      "msg": "Provided keys don't match"
    },
    {
      "code": 6012,
      "name": "PayerNotProvided",
      "msg": "Payer not provided"
    },
    {
      "code": 6013,
      "name": "RequireAuctionHouseSignOff",
      "msg": "Require auction house to sign off"
    },
    {
      "code": 6014,
      "name": "SellerTokenAccountCannotHaveDelegate",
      "msg": "Seller token account cannot have delegate"
    }
  ]
};

export const IDL: AuctionHouseV2 = {
  "version": "0.1.0",
  "name": "auction_house_v2",
  "instructions": [
    {
      "name": "create",
      "accounts": [
        {
          "name": "auctionHouse",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "treasuryMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "treasuryAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryWithdrawalAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treasuryWithdrawalOwner",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "feeAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "feeWithdrawalAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "sellerFeeBasisPoints",
          "type": "u16"
        },
        {
          "name": "requiresSignOff",
          "type": "bool"
        }
      ]
    },
    {
      "name": "sell",
      "accounts": [
        {
          "name": "auctionHouse",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "auctionHouseAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "treasuryMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "treeConfig",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "merkeTree",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "previousLeafDelegate",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "sellerTradeState",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "assetId",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "auctionHouseFeeAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "programAsSigner",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "bubblegumProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "compressionProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "logWrapper",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "sellerPrice",
          "type": "u64"
        },
        {
          "name": "root",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "dataHash",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "creatorHash",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "nonce",
          "type": "u64"
        },
        {
          "name": "index",
          "type": "u32"
        }
      ]
    },
    {
      "name": "bid",
      "accounts": [
        {
          "name": "auctionHouse",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "auctionHouseAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "treasuryMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "bidder",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "paymentAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "assetId",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "buyerEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerTradeState",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "auctionHouseFeeAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "buyerPrice",
          "type": "u64"
        }
      ]
    },
    {
      "name": "executeSale",
      "accounts": [
        {
          "name": "auctionHouse",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "auctionHouseAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "treasuryMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "treasuryAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "treeConfig",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "seller",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerReceiptAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "merkeTree",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerTradeState",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyer",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerTradeState",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "assetId",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "auctionHouseFeeAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "programAsSigner",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "bubblegumProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "compressionProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "logWrapper",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "buyerPrice",
          "type": "u64"
        },
        {
          "name": "root",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "dataHash",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "creatorHash",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "nonce",
          "type": "u64"
        },
        {
          "name": "index",
          "type": "u32"
        },
        {
          "name": "royaltyBasisPoints",
          "type": "u16"
        },
        {
          "name": "metadata",
          "type": {
            "defined": "MetadataArgs"
          }
        }
      ]
    },
    {
      "name": "cancel",
      "accounts": [
        {
          "name": "auctionHouse",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "treasuryMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "assetId",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "wallet",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tradeState",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "sellerPrice",
          "type": "u64"
        },
        {
          "name": "root",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "dataHash",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "creatorHash",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "nonce",
          "type": "u64"
        },
        {
          "name": "index",
          "type": "u32"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "auctionHouseV2Data",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "treasuryMint",
            "type": "publicKey"
          },
          {
            "name": "sellerFeeBasisPoints",
            "type": "u16"
          },
          {
            "name": "requiresSignOff",
            "type": "bool"
          },
          {
            "name": "treasuryAccount",
            "type": "publicKey"
          },
          {
            "name": "treasuryWithdrawalAccount",
            "type": "publicKey"
          },
          {
            "name": "feeAccount",
            "type": "publicKey"
          },
          {
            "name": "feeWithdrawalAccount",
            "type": "publicKey"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "treasuryBump",
            "type": "u8"
          },
          {
            "name": "feeAccountBump",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "Uses",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "useMethod",
            "type": {
              "defined": "UseMethod"
            }
          },
          {
            "name": "remaining",
            "type": "u64"
          },
          {
            "name": "total",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "Collection",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "verified",
            "type": "bool"
          },
          {
            "name": "key",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "Creator",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "address",
            "type": "publicKey"
          },
          {
            "name": "verified",
            "type": "bool"
          },
          {
            "name": "share",
            "docs": [
              "The percentage share.",
              "",
              "The value is a percentage, not basis points."
            ],
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "MetadataArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "docs": [
              "The name of the asset"
            ],
            "type": "string"
          },
          {
            "name": "symbol",
            "docs": [
              "The symbol for the asset"
            ],
            "type": "string"
          },
          {
            "name": "uri",
            "docs": [
              "URI pointing to JSON representing the asset"
            ],
            "type": "string"
          },
          {
            "name": "sellerFeeBasisPoints",
            "docs": [
              "Royalty basis points that goes to creators in secondary sales (0-10000)"
            ],
            "type": "u16"
          },
          {
            "name": "primarySaleHappened",
            "type": "bool"
          },
          {
            "name": "isMutable",
            "type": "bool"
          },
          {
            "name": "editionNonce",
            "docs": [
              "nonce for easy calculation of editions, if present"
            ],
            "type": {
              "option": "u8"
            }
          },
          {
            "name": "tokenStandard",
            "docs": [
              "Since we cannot easily change Metadata, we add the new DataV2 fields here at the end."
            ],
            "type": {
              "option": {
                "defined": "TokenStandard"
              }
            }
          },
          {
            "name": "collection",
            "docs": [
              "Collection"
            ],
            "type": {
              "option": {
                "defined": "Collection"
              }
            }
          },
          {
            "name": "uses",
            "docs": [
              "Uses"
            ],
            "type": {
              "option": {
                "defined": "Uses"
              }
            }
          },
          {
            "name": "tokenProgramVersion",
            "type": {
              "defined": "TokenProgramVersion"
            }
          },
          {
            "name": "creators",
            "type": {
              "vec": {
                "defined": "Creator"
              }
            }
          }
        ]
      }
    },
    {
      "name": "TokenStandard",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "NonFungible"
          },
          {
            "name": "FungibleAsset"
          },
          {
            "name": "Fungible"
          },
          {
            "name": "NonFungibleEdition"
          }
        ]
      }
    },
    {
      "name": "TokenProgramVersion",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Original"
          },
          {
            "name": "Token2022"
          }
        ]
      }
    },
    {
      "name": "UseMethod",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Burn"
          },
          {
            "name": "Multiple"
          },
          {
            "name": "Single"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "BumpSeedNotInHashMap",
      "msg": "Bump Seed Not In HashMap"
    },
    {
      "code": 6001,
      "name": "AccountNotInitialized",
      "msg": "Account not initialized"
    },
    {
      "code": 6002,
      "name": "InvalidSellerFeeBasisPoints",
      "msg": "Invalid Seller Fee Basis Points"
    },
    {
      "code": 6003,
      "name": "NumericOverflow",
      "msg": "Numeric Overflow"
    },
    {
      "code": 6004,
      "name": "NotEnoughFunds",
      "msg": "Not Enough Funds"
    },
    {
      "code": 6005,
      "name": "InvalidBuyingOrSellingOrder",
      "msg": "Invalid buying or selling order"
    },
    {
      "code": 6006,
      "name": "InvalidBuyingOrderPrice",
      "msg": "Invalid buying order price don't match with selling order"
    },
    {
      "code": 6007,
      "name": "MetadataHashMismatch",
      "msg": "Metadata hash does not match"
    },
    {
      "code": 6008,
      "name": "InvalidBuyerTradeState",
      "msg": "Buyer trade state is invalid"
    },
    {
      "code": 6009,
      "name": "BothPartiesNeedToAgreeToSale",
      "msg": "Both parties need to agree on a price"
    },
    {
      "code": 6010,
      "name": "InvalidSellerTradeState",
      "msg": "Seller trade is invalid"
    },
    {
      "code": 6011,
      "name": "PublicKeyMismatch",
      "msg": "Provided keys don't match"
    },
    {
      "code": 6012,
      "name": "PayerNotProvided",
      "msg": "Payer not provided"
    },
    {
      "code": 6013,
      "name": "RequireAuctionHouseSignOff",
      "msg": "Require auction house to sign off"
    },
    {
      "code": 6014,
      "name": "SellerTokenAccountCannotHaveDelegate",
      "msg": "Seller token account cannot have delegate"
    }
  ]
};
