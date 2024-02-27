import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AuctionHouseV2 } from "../target/types/auction_house_v2";
import { config } from "dotenv";
import {
  AccountMeta,
  Keypair,
  PublicKey,
  SYSVAR_RENT_PUBKEY,
  SystemProgram,
  Transaction,
  TransactionMessage,
  VersionedTransaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import * as AES from "crypto-js/aes";
import * as Utf8 from "crypto-js/enc-utf8";
import { BN } from "bn.js";
import {
  SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
  SPL_NOOP_PROGRAM_ID,
} from "@solana/spl-account-compression";
import {
  MPL_BUBBLEGUM_PROGRAM_ID,
  getAssetWithProof,
  mplBubblegum,
} from "@metaplex-foundation/mpl-bubblegum";
import { mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { fromWeb3JsKeypair } from "@metaplex-foundation/umi-web3js-adapters";
import {
  Umi,
  keypairIdentity,
  publicKey,
  PublicKey as UmiPublicKey,
} from "@metaplex-foundation/umi";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  AccountLayout,
  TOKEN_PROGRAM_ID,
  createAssociatedTokenAccount,
  createMint,
  getAssociatedTokenAddressSync,
  mintTo,
} from "@solana/spl-token";
import {
  bubblegumProgram,
  createBuyOrder,
  createLookupTable,
  createMerkleTree,
  createSellOrder,
  mint,
  pda,
  transferSol,
  typeCastMetadata,
} from "./utils";
import { expect } from "chai";
import {
  AUCTION_HOUSE,
  CANOPY_DEPTH,
  ESCROW,
  FEE,
  TRADE_STATE,
} from "./constants";

let bidder: Keypair;
let merkleTree: PublicKey, collection: PublicKey;
export function setup() {
  const umi = createUmi(process.env.RPC)
    .use(mplBubblegum())
    .use(mplTokenMetadata());
  const keypair = decodeKeypair(
    process.env.TREASURY_PRIVATE_KEY,
    process.env.TREASURY_SECRET
  );
  bidder = decodeKeypair(
    process.env.BIDDER_PRIVATE_KEY,
    process.env.BIDDER_SECRET
  );
  const signer = fromWeb3JsKeypair(keypair);
  return umi.use(keypairIdentity(signer));
}

export const decodeKeypair = (privateKey: string, secret: string) => {
  const wallet = AES.decrypt(privateKey, secret);
  const keypair = Keypair.fromSecretKey(
    Buffer.from(JSON.parse(wallet.toString(Utf8)))
  );
  return keypair;
};

describe("auction-house-v2", () => {
  config();
  const umi = setup();
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();

  const program = anchor.workspace.AuctionHouseV2 as Program<AuctionHouseV2>;
  const authority = decodeKeypair(
    process.env.TREASURY_PRIVATE_KEY,
    process.env.TREASURY_SECRET
  );
  before(async () => {
    const response = await createMerkleTree(umi);
    merkleTree = new PublicKey(response.merkleTree);
    collection = new PublicKey(response.collection);
  });

  describe("Native auction house", () => {
    const treasuryMint = new PublicKey(
      "So11111111111111111111111111111111111111112"
    );
    const auctionHousePda = pda(
      [
        Buffer.from(AUCTION_HOUSE),
        authority.publicKey.toBuffer(),
        treasuryMint.toBuffer(),
      ],
      program.programId
    );
    const sellerFeeBasisPoints = 500;
    before(async () => {
      const auctionHouseInfo = await program.account.auctionHouseV2Data.fetch(
        auctionHousePda
      );
      if (!auctionHouseInfo) {
        const tx = await program.methods
          .create(sellerFeeBasisPoints, false)
          .accounts({
            authority: authority.publicKey,
            treasuryMint,
            treasuryWithdrawalAccount: authority.publicKey,
            treasuryWithdrawalOwner: authority.publicKey,
            feeWithdrawalAccount: authority.publicKey,
            payer: authority.publicKey,
            systemProgram: SystemProgram.programId,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          })
          .signers([authority])
          .rpc();
        console.log("Auction House created successfully :", tx);
      }
    });
    it("Create Auction house", async () => {
      const auctionHouseInfo = await program.account.auctionHouseV2Data.fetch(
        auctionHousePda
      );
      expect(auctionHouseInfo.authority).to.eql(authority.publicKey);
      expect(auctionHouseInfo.requiresSignOff).to.be.false;
      expect(auctionHouseInfo.sellerFeeBasisPoints).to.eql(
        sellerFeeBasisPoints
      );
      expect(auctionHouseInfo.treasuryMint).to.eql(treasuryMint);
      expect(auctionHouseInfo.treasuryWithdrawalAccount).to.eql(
        authority.publicKey
      );
    });

    it("Make a sell order", async () => {
      const { assetId, sellerPrice } = await createSellOrder(
        umi,
        program,
        merkleTree,
        collection,
        authority,
        treasuryMint
      );
      const sellerTradeState = pda(
        [
          Buffer.from(TRADE_STATE),
          authority.publicKey.toBuffer(),
          auctionHousePda.toBuffer(),
          assetId.toBuffer(),
          sellerPrice.toBuffer("le", 8),
        ],
        program.programId
      );
      const sellerTradeStateInfo = await provider.connection.getAccountInfo(
        sellerTradeState
      );
      expect(sellerTradeStateInfo).to.not.null;
    });

    it("Make a buy order", async () => {
      const { assetId, buyerPrice } = await createBuyOrder(
        umi,
        program,
        provider.connection,
        authority,
        bidder,
        merkleTree,
        collection,
        treasuryMint
      );
      const [buyerTradeState] = PublicKey.findProgramAddressSync(
        [
          Buffer.from(TRADE_STATE),
          bidder.publicKey.toBuffer(),
          auctionHousePda.toBuffer(),
          assetId.toBuffer(),
          buyerPrice.toBuffer("le", 8),
        ],
        program.programId
      );
      const buyerTradeStateInfo = await provider.connection.getAccountInfo(
        buyerTradeState
      );
      const [buyerEscrow] = PublicKey.findProgramAddressSync(
        [
          Buffer.from(ESCROW),
          auctionHousePda.toBuffer(),
          bidder.publicKey.toBuffer(),
        ],
        program.programId
      );
      const buyerEscrowInfo = await provider.connection.getAccountInfo(
        buyerEscrow
      );

      expect(buyerTradeStateInfo).to.not.null;
      expect(buyerEscrowInfo).to.not.null;
      expect(buyerEscrowInfo.lamports).greaterThanOrEqual(10000);
    });

    it("Execute Sale of an Nft", async () => {
      const { assetId, sellerPrice } = await createSellOrder(
        umi,
        program,
        merkleTree,
        collection,
        authority,
        treasuryMint
      );
      const { buyerPrice } = await createBuyOrder(
        umi,
        program,
        provider.connection,
        authority,
        bidder,
        merkleTree,
        collection,
        treasuryMint,
        assetId,
        sellerPrice
      );
      const [buyerEscrow] = PublicKey.findProgramAddressSync(
        [
          Buffer.from(ESCROW),
          auctionHousePda.toBuffer(),
          bidder.publicKey.toBuffer(),
        ],
        program.programId
      );
      const buyerEscrowInfo = await provider.connection.getAccountInfo(
        buyerEscrow
      );
      const sellerInfo = await provider.connection.getAccountInfo(
        authority.publicKey
      );

      const { root, creatorHash, dataHash, proof, nonce, index, metadata } =
        await getAssetWithProof(umi, publicKey(assetId));
      const remainingAccounts: AccountMeta[] = [];
      remainingAccounts.push(
        ...metadata.creators.map((creator): AccountMeta => {
          return {
            isSigner: false,
            isWritable: false,
            pubkey: new PublicKey(creator.address),
          };
        })
      );
      remainingAccounts.push(
        ...proof.slice(0, proof.length - CANOPY_DEPTH).map((p) => {
          return {
            isSigner: false,
            isWritable: false,
            pubkey: new PublicKey(p),
          };
        })
      );
      const treeConfig = pda([merkleTree.toBuffer()], bubblegumProgram);
      const signature = await program.methods
        .executeSale(
          buyerPrice,
          [...root],
          [...dataHash],
          [...creatorHash],
          new BN(nonce),
          index,
          500,
          typeCastMetadata(metadata)
        )
        .accounts({
          auctionHouseAuthority: authority.publicKey,
          treasuryMint,
          merkleTree,
          treeConfig,
          buyer: bidder.publicKey,
          seller: authority.publicKey,
          sellerReceiptAccount: authority.publicKey,
          assetId,
          compressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
          logWrapper: SPL_NOOP_PROGRAM_ID,
          bubblegumProgram: MPL_BUBBLEGUM_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .remainingAccounts(remainingAccounts)
        .signers([authority])
        .rpc();
      const [sellerTradeState] = PublicKey.findProgramAddressSync(
        [
          Buffer.from(TRADE_STATE),
          authority.publicKey.toBuffer(),
          auctionHousePda.toBuffer(),
          assetId.toBuffer(),
          buyerPrice.toBuffer("le", 8),
        ],
        program.programId
      );

      const [buyerTradeState] = PublicKey.findProgramAddressSync(
        [
          Buffer.from(TRADE_STATE),
          bidder.publicKey.toBuffer(),
          auctionHousePda.toBuffer(),
          assetId.toBuffer(),
          buyerPrice.toBuffer("le", 8),
        ],
        program.programId
      );

      const newBuyerEscrowInfo = await provider.connection.getAccountInfo(
        buyerEscrow
      );

      const buyerTradeStateInfo = await provider.connection.getAccountInfo(
        buyerTradeState
      );
      const sellerTradeStateInfo = await provider.connection.getAccountInfo(
        sellerTradeState
      );
      const asset = await umi.rpc.getAsset(publicKey(assetId));
      const newSellerInfo = await provider.connection.getAccountInfo(
        authority.publicKey
      );

      expect(Number(newSellerInfo.lamports)).to.greaterThan(
        sellerInfo.lamports
      );
      expect(Number(newBuyerEscrowInfo.lamports)).to.lessThan(
        Number(buyerEscrowInfo.lamports)
      );
      expect(asset.ownership.owner.toString()).to.eql(
        bidder.publicKey.toString()
      );
      expect(buyerTradeStateInfo).to.be.null;
      expect(sellerTradeStateInfo).to.be.null;

      console.log("Sale executed successfully: ", signature);
    });
  });
});
