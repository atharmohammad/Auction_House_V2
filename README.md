# Auction house V2

## 🔖 About
- Auction House V2 is an open-source smart contract with a low level SDK for dApps, designed to easily integrate support for compressed NFT auctions. Compressed NFTs, a new standard on Solana, currently lack support in developer tools. With Auction House V2, dApps can effortlessly incorporate auction and marketplace features without the need to invest time and money in building a secure contract, especially beneficial for early dApps that can concentrate on developing their core product.

## 🚀 Features
- Create a sell order for Compressed Nfts
- Make bid orders on a Compressed Nfts
- Match bid and sell orders and execute sale
- Cancel bid or sell orders
- Supports multiple currencies

## How to Use
To integrate in your application, simply install the sdk 
```
npm install auction-house-v2
```
To build and test program:

- Install anchor

- Add (Devnet)RPC URL and wallet path in ``Anchor.toml``

```
[provider]
cluster = "<RPC_URL>"
wallet = "<WALLET_PATH>"
```

```
cd auction-house-v2
anchor build
anchor deploy
anchor run test
```

## ⛳ Milestones
- Build a high level wrapper on current sdk
- Fully tested and audited program with an sdk
- Support timed auctions
- Build on feature requests from ecosystem

## 📲 Contact
- We're gonna use github discussions for any questions, feature requests and related topics. Feel free to drop by 👋: [Discussion](https://github.com/atharmohammad/Auction_House_V2/discussions/1)

*** CODE IS NOT AUDITED AND IS WORK IN PROGRESS ***
