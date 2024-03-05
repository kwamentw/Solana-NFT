import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaNftAnchor } from "../target/types/solana_nft_anchor";

describe("solana-nft", () => {
  // Configure the client to use the local cluster.
  const provider =anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SolanaNftAnchor as Program<SolanaNftAnchor>;


  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initNft().rpc();
    console.log("Your transaction signature", tx);
  });
});
