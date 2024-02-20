use anchor_lang::prelude::*;

declare_id!("9gcy363E4e8to6GCVje5xxLfqe6krVFfot2HHymteCod");

#[program]
pub mod solana_nft {
    use super::*;

    pub fn InitNFT(ctx: Context<InitNFT>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitNFT<'info> {
    /// CHECK: ok, we are passing this account in ourselves
    #[account(mut, signer)]
    signer: AccountInfo<'info>,
}
