import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaNft } from "../target/types/solana_nft";
import { walletAdapterIdentity } from "@metaplex-foundation/umi-signer-wallet-adapters";
import { MPL_TOKEN_METADATA_PROGRAM_ID, findMasterEditionPda, findMetadataPda, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, getAssociatedTokenAddress } from "@solana/spl-token";
import { createGenericFile, createUmi, publicKey } from "@metaplex-foundation/umi";

describe("solana-nft", async () => {
  // Configure the client to use the local cluster.
  const provider =anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SolanaNftAnchor as Program<SolanaNft>;

  const signer = provider.wallet;

  const umi = createUmi().use(walletAdapterIdentity(signer)).use(mplTokenMetadata());

  //generating our mint account
  const mint = anchor.web3.Keypair.generate();

  //Derive the associated token address for the mint
  const associatedTokenAccount = await getAssociatedTokenAddress(mint.publicKey,signer.publicKey);
  
  //Derive the metadata account
  let metadataAccount=findMetadataPda(umi,{mint:publicKey(mint.publicKey)})[0];

  // derive the master edition pda
  let masterEditionAccount = findMasterEditionPda(umi,{mint: publicKey(mint.publicKey)})[0];

  // const anotherumi = createUmi();
  // anotherumi.use(nftStorageUploader({token: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJkaWQ6ZXRocjoweGRBOTI2NzJlZTYxMTk3NEUxRTIxNkQ3QzA5OGM4NjdGQmNENzdCODgiLCJpc3MiOiJuZnQtc3RvcmFnZSIsImlhdCI6MTcwOTYzMzAxNTAxMiwibmFtZSI6ImZpcnN0TmZ0In0.6uPqFnt2FhwQLiY3T1rArhIaQHb_eu99IkmBI7Zqo3c"}));

  // const imageBuffer = readFileSync('https://bafkreigd7k6aapovwhdejyava5kpiry5nbv6udzlyshdkkthe27smwbbf4.ipfs.nftstorage.link/');
  // async function uploader(){
  //   const [imageUri] = await umi.uploader.upload([
  //     createGenericFile(imageBuffer,'st-bernard.png'),
  //   ])

  //   //upload json metadata
  //   const uri = await umi.uploader.uploadJson({
  //     name: 'FirstNFt',
  //     description:'My first custom nft',
  //     image: imageUri,
  //   })
  //   console.log("uri:",uri);
  // }

  // uploader();

  const metadata ={
    name: "Dawg",
    symbol: "DWG",
    uri:"https://raw.githubusercontent.com/687c/solana-nft-native-client/main/metadata.json"
  }
  


  it("mints nft!", async () => {
    // Add your test here.
    const tx = await program.methods.initNft(metadata.name,metadata.symbol,metadata.uri).accounts({
      signer:provider.publicKey,
      mint: mint.publicKey,
      associatedTokenAccount,
      metadataAccount,
      masterEditionAccount,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
      rent:anchor.web3.SYSVAR_RENT_PUBKEY,
    }).signers([mint]).rpc();
    console.log(`mint nft tx: https:explorer.solana.com/tx/${tx}?cluster=devnet`);
    console.log(`minted nft: https://explorer.solana.com/address/${mint.publicKey}?cluster=devnet`);
    console.log("Your transaction signature", tx);
  });
});
// function nftStorageUploader(arg0: { token: string; }): import("@metaplex-foundation/umi").UmiPlugin {
//   throw new Error("Function not implemented.");
// }

// function readFileSync(arg0: string) {
//   throw new Error("Function not implemented.");
// }

