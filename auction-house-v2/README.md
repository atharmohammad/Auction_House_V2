# Overview
AuctionHouseV2 is a protocol for marketplaces to implement a decentralized sales contract for compressed Nfts.Currently available on  Devnet. Anyone can create an AuctionHouse and accept any SPL token they wish.

## Instructions
 ### 📄 ``create``
  This instruction create an auction house for a wallet and the recipient currency:
  <details>
  <summary>Accounts</summary>

  | Name | Writable | Signer | Description |
  | ---  |   ---    |   ---  |   ---       |
  | auction_house | ✅ |   | Auction house acccount to be initialized. Pda seeds (``["auction_house",authority,treasury_mint]``) | 
  | authority |  |   | Auction house authority to be initialized | 
  | treasury_mint |  |   | Mint for auction house treasury account | 
  | treasury_account | ✅ |   | Auction house treasury account Pda seeds (``["treasury",auction_house]``)| 
  | treasury_withdrawal_account | ✅ |   | Recipient account where treasury can be withdrawn | 
  | treasury_withdrawal_owner |  |   | Treasruy withdrawal owner account | 
  | fee_account |  |   | Auction house fee account to pay for sale related fee if executed by auction house Pda seeds (``["fee",auction_house]``)| 
  | fee_withdrawal_account |  |   |Recipient account where amount in fee account can be withdrawn | 
  | payer | ✅ |  ✅ | Payer of the transaction | 
  | system_program |  |   | ``System program`` account | 
  | token_program |  |   | ``Token program`` account| 
  | associated_token_program |  |   | ``Associated token program`` account | 

</details>

<details>
  <summary>Arguments</summary>
  
  | Name | Description |
  | ---  | ---  |
  | seller_fee_basis_points |  Auction house fee percentage in basis points | 
  | requires_sign_off  | if ``true`` then auction house signature required to execute sale | 

</details>

### 📄 ``sell``
  This instruction creates a sell order to list the compressed nft:
  <details>
  <summary>Accounts</summary>

  | Name | Writable | Signer | Description |
  | ---  |   ---    |   ---  |   ---       |
  | auction_house |  |   | Auction house acccount. Pda seeds (``["auction_house",authority,treasury_mint]``) | 
  | auction_house_authority |  |   | Auction house authority | 
  | treasury_mint |  |   | Mint for auction house treasury account | 
  | tree_config |  |   | Merkle tree authority account of cnft| 
  | owner | ✅ | ✅ | Owner of the cnft | 
  | merkle_tree | ✅ |  | Merkle tree account | 
  | previous_leaf_delegate |  |   |Previous leaf delegate account of cnft| 
  | seller_trade_state | ✅  |   | Trade state account to initialize for creating a sell order.Pda seeds (``["trade_state",owner,auction_house,asset_id,seller_price]``)  | 
  | asset_id |  |  | Asset id of cnft | 
  | program_as_signer |  |  | Program as signer account. Pda seeds (``["program","signer"]``)| 
  | bubblegum_program |  |  | ``Bubblegum program`` account| 
  | compression_program |  |  | ``Compression program`` account| 
  | system_program |  |   | ``System program`` account | 
  | log_wrapper |  |   | ``Noop Program`` account| 
  | remaining_account |  |   | Cnft proofs in remaining accounts| 

</details>

<details>
  <summary>Arguments</summary>
  
  | Name | Description |
  | ---  | ---  |
  | seller_price | Listing price of the cnft | 
  | root  | Cnft root| 
  | data_hash | Hashed data of cnft| 
  | creator_hash  | Creator hash of cnft| 
  | nonce | Cnft nonce | 
  | index  | Cnft index| 

</details>

### 📄 ``bid``
  This instruction creates a bid order to bid on a compressed nft:
  <details>
  <summary>Accounts</summary>

  | Name | Writable | Signer | Description |
  | ---  |   ---    |   ---  |   ---       |
  | auction_house |  |   | Auction house acccount. Pda seeds (``["auction_house",authority,treasury_mint]``) | 
  | auction_house_authority |  |   | Auction house authority | 
  | treasury_mint |  |   | Mint for auction house treasury account | 
  | bidder | ✅ | ✅ | Wallet placing bid on cnft | 
  | asset_id |  |  | Asset id of cnft | 
  | payment_account | ✅ |  | Payer of bid | 
  | buyer_escrow | ✅ |   | Buyer escrow account | 
  | buyer_trade_state | ✅  |   | Trade state account to initialize for creating a bid order.Pda seeds (``["trade_state",bidder,auction_house,asset_id,buyer_price]``)  | 
  | auction_house_fee_account | ✅ |  | Auction house fee account to pay for sale related fee if executed by auction house Pda seeds (``["fee",auction_house]``)| 
  | system_program |  |   | ``System program`` account | 
  | token_program |  |   | ``Token Program`` account| 
  | rent |  |   | ``Rent`` Sysvar| 

</details>

<details>
  <summary>Arguments</summary>
  
  | Name | Description |
  | ---  | ---  |
  | buyer_price | Bidding price of the cnft | 

</details>

### 📄 ``execute_sale``
  This instruction execute sale for matching orders:
  <details>
  <summary>Accounts</summary>

  | Name | Writable | Signer | Description |
  | ---  |   ---    |   ---  |   ---       |
  | auction_house |  |   | Auction house acccount. Pda seeds (``["auction_house",authority,treasury_mint]``) | 
  | auction_house_authority |  |   | Auction house authority | 
  | treasury_mint |  |   | Mint for auction house treasury account | 
  | treasury_account | ✅ |   | Auction house treasury account Pda seeds (``["treasury",auction_house]``)| 
  | tree_config |  |   | Merkle tree authority account of cnft| 
  | seller | ✅ |  | Owner of the cnft | 
  | seller_receipt_account | ✅ |  | Receipt account of seller for listing amount | 
  | merkle_tree | ✅ |  | Merkle tree account | 
  | seller_trade_state | ✅  |   | Trade state account to initialize for creating a sell order.Pda seeds (``["trade_state",owner,auction_house,asset_id,seller_price]``)  | 
  | buyer | ✅ |  | Wallet placing bid on cnft | 
  | buyer_escrow | ✅ |   | Buyer escrow account | 
  | buyer_trade_state | ✅  |   | Trade state account to initialize for creating a bid order.Pda seeds (``["trade_state",bidder,auction_house,asset_id,buyer_price]``)  |
  | asset_id |  |  | Asset id of cnft | 
  | auction_house_fee_account | ✅ |  | Auction house fee account to pay for sale related fee if executed by auction house Pda seeds (``["fee",auction_house]``)| 
  | program_as_signer |  |  | Program as signer account. Pda seeds (``["program","signer"]``)| 
  | bubblegum_program |  |  | ``Bubblegum program`` account| 
  | compression_program |  |  | ``Compression program`` account| 
  | system_program |  |   | ``System program`` account | 
  | token_program |  |   | ``Token program`` account| 
  | associated_token_program |  |   | ``Associated token program`` account | 
  | log_wrapper |  |   | ``Noop Program`` account| 
  | remaining_account |  |   | Creator accounts + Cnft proofs in remaining accounts| 

</details>

<details>
  <summary>Arguments</summary>
  
  | Name | Description |
  | ---  | ---  |
  | buyer_price | Buying price of the cnft | 
  | root  | Cnft root| 
  | data_hash | Hashed data of cnft| 
  | creator_hash  | Creator hash of cnft| 
  | nonce | Cnft nonce | 
  | index  | Cnft index| 
  | royalty_basis_points  | Auction house royalty precent in basis points| 
  | metadata  | Metadata arguments of Cnft| 

</details>
