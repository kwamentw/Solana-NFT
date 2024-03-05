import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaNftAnchor } from "../target/types/solana_nft_anchor";
import { walletAdapterIdentity } from "@metaplex-foundation/umi-signer-wallet-adapters";
import { findMasterEditionPda, findMetadataPda, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";
import { getAssociatedTokenAddress } from "@solana/spl-token";
import { createUmi, publicKey } from "@metaplex-foundation/umi";

describe("solana-nft", async () => {
  // Configure the client to use the local cluster.
  const provider =anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SolanaNftAnchor as Program<SolanaNftAnchor>;

  const signer = provider.wallet;

  const umi = createUmi("https://api.devnet.solana.com").use(walletAdapterIdentity(signer)).use(mplTokenMetadata());

  //generating our mint account
  const mint = anchor.web3.Keypair.generate();

  //Derive the associated token address for the mint
  const associatedTokenAccount = await getAssociatedTokenAddress(mint.publicKey,signer.publicKey);
  
  //Derive the metadata account
  let metadataAccount=findMetadataPda(umi,{mint:publicKey(mint.publicKey)})[0];

  // derive the master edition pda
  let masterEditionAccount = findMasterEditionPda(umi,{mint: publicKey(mint.publicKey)})[0];
  


  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initNft().rpc();
    console.log("Your transaction signature", tx);
  });
});
