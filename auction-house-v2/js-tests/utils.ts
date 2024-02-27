import { Program } from "@coral-xyz/anchor";
import {
  Collection,
  MetadataArgs,
  TokenProgramVersion,
  TokenStandard,
  UseMethod,
  Uses,
  Creator,
  createTree,
  fetchMerkleTree,
  mintToCollectionV1,
  findLeafAssetIdPda,
  getAssetWithProof,
  MPL_BUBBLEGUM_PROGRAM_ID,
} from "@metaplex-foundation/mpl-bubblegum";
import { createNft } from "@metaplex-foundation/mpl-token-metadata";
import {
  Option,
  Umi,
  generateSigner,
  percentAmount,
  PublicKey as UmiPublicKey,
  publicKey,
} from "@metaplex-foundation/umi";
import {
  AccountMeta,
  AddressLookupTableProgram,
  Connection,
  Keypair,
  PublicKey,
  SYSVAR_RENT_PUBKEY,
  Signer,
  SystemProgram,
  Transaction,
  TransactionMessage,
  VersionedTransaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import BN from "bn.js";
import { AuctionHouseV2 } from "../target/types/auction_house_v2";
import {
  SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
  SPL_NOOP_PROGRAM_ID,
} from "@solana/spl-account-compression";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { CANOPY_DEPTH } from "./constants";

export type Maybe<T> = T | null | undefined;

export const isNullLike = <T>(v: Maybe<T>): v is null | undefined =>
  v === null || v === undefined;

export const bubblegumProgram = new PublicKey(
  "BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY"
);

const collectionName = "Adventure Ends!";
const collectionUri =
  "https://arweave.net/ey2CEW9W3zuc7nNVzafO22jH3NG0CLhy_1QJUx8jn0s";
let leafIndex = 0;

export const TokenStandardAnchor = {
  NonFungible: { nonFungible: {} },
  FungibleAsset: { fungibleAsset: {} },
  Fungible: { fungible: {} },
  NonFungibleEdition: { nonFungibleEdition: {} },
};
export type TokenStandardAnchor =
  (typeof TokenStandardAnchor)[keyof typeof TokenStandardAnchor];
export const typeCastTokenStandard = (
  t: Option<TokenStandard>
): TokenStandardAnchor | null => {
  if (t.__option === "None") return null;
  switch (t.value) {
    case TokenStandard.Fungible:
      return TokenStandardAnchor.Fungible;
    case TokenStandard.NonFungible:
      return TokenStandardAnchor.NonFungible;
    case TokenStandard.NonFungibleEdition:
      return TokenStandardAnchor.NonFungibleEdition;
    case TokenStandard.FungibleAsset:
      return TokenStandardAnchor.FungibleAsset;
  }
};

export const UseMethodAnchor = {
  Burn: { burn: {} },
  Multiple: { multiple: {} },
  Single: { single: {} },
};
export type UseMethodAnchor =
  (typeof UseMethodAnchor)[keyof typeof UseMethodAnchor];
export const typeCastUseMethod = (u: UseMethod): UseMethodAnchor => {
  switch (u) {
    case UseMethod.Burn:
      return UseMethodAnchor.Burn;
    case UseMethod.Single:
      return UseMethodAnchor.Single;
    case UseMethod.Multiple:
      return UseMethodAnchor.Multiple;
  }
};

export type UsesAnchor = {
  useMethod: UseMethodAnchor;
  remaining: BN;
  total: BN;
};
export const typeCastUses = (u: Option<Uses>): UsesAnchor => {
  if (u.__option === "None") return null;
  return {
    useMethod: typeCastUseMethod(u.value.useMethod),
    remaining: new BN(Number(u.value.remaining)),
    total: new BN(Number(u.value.total)),
  };
};

const TokenProgramVersionAnchor = {
  Original: { original: {} },
  Token2022: { token2022: {} },
};
export type TokenProgramVersionAnchor =
  (typeof TokenProgramVersionAnchor)[keyof typeof TokenProgramVersionAnchor];
export const typeCastTokenProgramVersion = (
  t: TokenProgramVersion
): TokenProgramVersionAnchor => {
  switch (t) {
    case TokenProgramVersion.Original:
      return TokenProgramVersionAnchor.Original;
    case TokenProgramVersion.Token2022:
      return TokenProgramVersionAnchor.Token2022;
  }
};

export type CollectionAnchor = { verified: boolean; key: PublicKey };
export const typeCastCollection = (t: Collection): CollectionAnchor => {
  return {
    ...t,
    key: new PublicKey(t.key),
  };
};

export type CreatorAnchor = Omit<Creator, "address"> & { address: PublicKey };
export const typeCastCreators = (
  creators: Array<Creator>
): Array<CreatorAnchor> => {
  return creators.map((creator) => {
    return {
      ...creator,
      address: new PublicKey(creator.address),
    };
  });
};

export type MetadataArgsAnchor = Omit<
  MetadataArgs,
  | "tokenStandard"
  | "uses"
  | "tokenProgramVersion"
  | "editionNonce"
  | "collection"
  | "creators"
> & {
  tokenStandard: TokenStandardAnchor;
  uses: UsesAnchor;
  tokenProgramVersion: TokenProgramVersionAnchor;
  editionNonce: number;
  collection: CollectionAnchor;
  creators: Array<CreatorAnchor>;
};
export const typeCastMetadata = (m: MetadataArgs): MetadataArgsAnchor => {
  let editionNonce: number = null,
    collection: Collection = null;
  if (m.editionNonce.__option === "Some") {
    editionNonce = m.editionNonce.value;
  }
  if (m.collection.__option === "Some") {
    collection = m.collection.value;
  }
  return {
    ...m,
    editionNonce,
    tokenStandard: typeCastTokenStandard(m.tokenStandard),
    uses: typeCastUses(m.uses),
    tokenProgramVersion: typeCastTokenProgramVersion(m.tokenProgramVersion),
    collection: typeCastCollection(collection),
    creators: typeCastCreators(m.creators),
  };
};

export async function createLookupTable(
  connection: Connection,
  signer: Keypair,
  addresses: PublicKey[]
) {
  try {
    const latestBlockhash = await connection.getLatestBlockhash({
      commitment: "confirmed",
    });
    const currentSlot = await connection.getSlot("recent");
    const slots = await connection.getBlocks(
      currentSlot - 20,
      undefined,
      "confirmed"
    );
    const [instruction, address] = AddressLookupTableProgram.createLookupTable({
      authority: signer.publicKey,
      payer: signer.publicKey,
      recentSlot: slots[0],
    });
    const extendInstruction = AddressLookupTableProgram.extendLookupTable({
      payer: signer.publicKey,
      authority: signer.publicKey,
      lookupTable: address,
      addresses: [...addresses, SystemProgram.programId],
    });
    const messageV0 = new TransactionMessage({
      payerKey: signer.publicKey,
      recentBlockhash: latestBlockhash.blockhash,
      instructions: [instruction, extendInstruction],
    }).compileToV0Message();

    const transaction = new VersionedTransaction(messageV0);
    transaction.sign([signer]);
    const txid = await connection.sendTransaction(transaction);

    await connection.confirmTransaction(
      {
        signature: txid,
        blockhash: latestBlockhash.blockhash,
        lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
      },
      "confirmed"
    );
    console.log(`Lookup table ${address.toBase58()} created`);
    return address.toBase58();
  } catch (e) {
    console.error("Create Lookup Table failed :", e);
  }
}

export async function transferSol(
  connection: Connection,
  sender: Signer,
  recipient: PublicKey,
  amount: number
) {
  const transferInstruction = SystemProgram.transfer({
    toPubkey: recipient,
    fromPubkey: sender.publicKey,
    lamports: amount,
  });
  const latestBlockHash = await connection.getLatestBlockhash();
  const transferTransaction = new Transaction(latestBlockHash).add(
    transferInstruction
  );
  const transferSignature = await sendAndConfirmTransaction(
    connection,
    transferTransaction,
    [sender]
  );
  console.log(
    `${amount} lamports has been transferred to ${recipient.toString()} : `,
    transferSignature
  );
}

export async function createMerkleTree(umi: Umi) {
  const collectionMint = generateSigner(umi);
  await createNft(umi, {
    mint: collectionMint,
    name: collectionName,
    uri: collectionUri,
    sellerFeeBasisPoints: percentAmount(5), // 5%
    isCollection: true,
  }).sendAndConfirm(umi);
  console.log("Collection Nft Created: ", collectionMint.publicKey.toString());

  const merkleTree = generateSigner(umi);
  const builder = await createTree(umi, {
    merkleTree,
    maxDepth: 5,
    maxBufferSize: 8,
    canopyDepth: CANOPY_DEPTH,
    public: true,
  });
  const signedTransaction = await builder.buildAndSign(umi);
  const signature = await umi.rpc.sendTransaction(signedTransaction);
  console.log("Transaction signature: ", signature.toString());
  const latestBlockHash = await umi.rpc.getLatestBlockhash();
  const response = await umi.rpc.confirmTransaction(signature, {
    strategy: {
      type: "blockhash",
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    },
    commitment: "finalized",
  });
  if (response.value.err) {
    console.log("Confirmed Transaction Result: ", response);
  }
  const merkleTreeInfo = await fetchMerkleTree(umi, merkleTree.publicKey);
  console.log("MERKLE TREE CREATED:", {
    publicKey: merkleTree.publicKey.toString(),
    canopy: merkleTreeInfo.canopy.length,
    header: merkleTreeInfo.header,
    activeIndex: merkleTreeInfo.tree.activeIndex,
    bufferSize: merkleTreeInfo.tree.bufferSize,
    changeLogs: merkleTreeInfo.tree.changeLogs[0],
    authority: merkleTreeInfo.treeHeader.authority.toString(),
    maxDepth: merkleTreeInfo.treeHeader.maxDepth,
    maxBufferSize: merkleTreeInfo.treeHeader.maxBufferSize,
    discriminator: merkleTreeInfo.discriminator.toString(),
  });
  return {
    merkleTree: merkleTree.publicKey,
    collection: collectionMint.publicKey,
  };
}

export async function mint(
  umi: Umi,
  tree: UmiPublicKey,
  collection: UmiPublicKey
) {
  const tx = await mintToCollectionV1(umi, {
    leafOwner: umi.identity.publicKey,
    merkleTree: tree,
    collectionMint: collection,
    metadata: {
      name: collectionName,
      uri: collectionUri,
      sellerFeeBasisPoints: 500,
      collection: { key: collection, verified: true },
      creators: [
        { address: umi.identity.publicKey, verified: true, share: 100 },
      ],
    },
  }).sendAndConfirm(umi);
  const [assetId] = findLeafAssetIdPda(umi, {
    merkleTree: tree,
    leafIndex: leafIndex++,
  });
  return assetId;
}

export async function createSellOrder(
  umi: Umi,
  program: Program<AuctionHouseV2>,
  merkleTree: PublicKey,
  collection: PublicKey,
  authority: Signer,
  treasuryMint: PublicKey,
  assetId?: UmiPublicKey
) {
  const assetPda =
    assetId ?? (await mint(umi, publicKey(merkleTree), publicKey(collection)));
  const { root, creatorHash, dataHash, proof, nonce, index, leafDelegate } =
    await getAssetWithProof(umi, assetPda);

  const remainingAccounts: AccountMeta[] = proof
    .slice(0, proof.length - CANOPY_DEPTH)
    .map((p) => {
      return {
        isSigner: false,
        isWritable: false,
        pubkey: new PublicKey(p.toString()),
      };
    });

  const sellerPrice = new BN(10000);
  const treeConfig = pda([merkleTree.toBuffer()], bubblegumProgram);
  const tx = await program.methods
    .sell(
      sellerPrice,
      [...root],
      [...dataHash],
      [...creatorHash],
      new BN(nonce),
      index
    )
    .accounts({
      auctionHouseAuthority: authority.publicKey,
      treasuryMint,
      merkleTree,
      owner: authority.publicKey,
      previousLeafDelegate: leafDelegate,
      treeConfig,
      assetId: assetPda,
      compressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
      logWrapper: SPL_NOOP_PROGRAM_ID,
      bubblegumProgram: MPL_BUBBLEGUM_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    })
    .remainingAccounts(remainingAccounts)
    .signers([authority])
    .rpc({ skipPreflight: true });
  console.log("Compressed nft listed successfully", tx);
  return { assetId: new PublicKey(assetPda), sellerPrice };
}

export async function createBuyOrder(
  umi: Umi,
  program: Program<AuctionHouseV2>,
  connection: Connection,
  authority: Signer,
  bidder: Signer,
  merkleTree: PublicKey,
  collection: PublicKey,
  treasuryMint: PublicKey,
  assetId?: PublicKey,
  buyerPrice?: BN
) {
  const assetPda =
    assetId ?? (await mint(umi, publicKey(merkleTree), publicKey(collection)));
  await transferSol(connection, authority, bidder.publicKey, 2000000);
  const bidPrice = buyerPrice ?? new BN(10000);
  const signature = await program.methods
    .bid(bidPrice)
    .accounts({
      paymentAccount: authority.publicKey,
      auctionHouseAuthority: authority.publicKey,
      treasuryMint,
      assetId: assetPda,
      bidder: bidder.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
      rent: SYSVAR_RENT_PUBKEY,
      systemProgram: SystemProgram.programId,
    })
    .signers([bidder])
    .rpc();
  console.log("Bid order has been created successfully: ", signature);
  return { assetId: new PublicKey(assetPda), buyerPrice: bidPrice };
}
export function pda(seeds: (Buffer | Uint8Array)[], programId: PublicKey) {
  const [pdaKey] = PublicKey.findProgramAddressSync(seeds, programId);
  return pdaKey;
}
