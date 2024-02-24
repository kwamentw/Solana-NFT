// anchor modules you have to declare when using them
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::Metadata, //new
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
use mpl_token_metadata::{
    pda::{find_master_edition_account, find_metadata_account}, //new
};

declare_id!("9gcy363E4e8to6GCVje5xxLfqe6krVFfot2HHymteCod");

// main program for rust
#[program]
pub mod solana_nft {
    use super::*;

    pub fn InitNFT(ctx: Context<InitNFT>) -> Result<()> {
        // creating mint account
        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            },
        );
        mint_to(cpi_context, 1)?;
        Ok(())
    }
}

// where accounts are initialized
#[derive(Accounts)]
pub struct InitNFT<'info> {
    /// CHECK: ok, we are passing this account in ourselves
    // This is the account of the signer
    #[account(mut, signer)]
    // initialising it with an AccountInfo type
    pub signer: AccountInfo<'info>,
    //This is the mint Acount
    #[account(init, payer = signer,mint::decimals=0,mint::authority=signer.key(),mint::freeze_authority=signer.key())]
    // it has the Account type Account so that we can have more constraints to keep it safe
    pub mint: Account<'info, Mint>,
    // This is the token Account
    #[account(init_if_needed,payer = signer,associated_token::mint=mint,associated_token::authority=signer)]
    // initialising token account with account type since it has many constraints
    pub associated_token_account: Account<'info, TokenAccount>,

    /////////////////////////--------program declaration--------//////////////////////////////
    pub token_program: Program<'info, Token>, // new token program kinda like the mastermind
    pub associated_token_program: Program<'info, AssociatedToken>, // new Associated token program handles the creation of an associated token account (ata)
    pub system_program: Program<'info, System>, // new system program responsible for creating all accounts
    pub rent: Sysvar<'info, Rent>, // new rent program responsible for paying rent logic
}
