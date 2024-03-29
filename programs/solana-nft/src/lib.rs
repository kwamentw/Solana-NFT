// anchor modules you have to add when using them
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMasterEditionV3,
        CreateMetadataAccountsV3, Metadata,
    }, //new
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
use mpl_token_metadata::accounts::{MasterEdition,Metadata as MetadataAccount}; //new

declare_id!("9gcy363E4e8to6GCVje5xxLfqe6krVFfot2HHymteCod");

// main program for rust
#[program]
pub mod solana_nft {
    use super::*;

    pub fn init_nft(ctx: Context<InitNFT>, name:String, symbol:String, uri:String,) -> Result<()> {
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

        // create metadata account
        let cpi_context=CpiContext::new(ctx.accounts.token_metadata_program.to_account_info(), CreateMetadataAccountsV3{metadata:ctx.accounts.metadata_account.to_account_info(), mint:ctx.accounts.mint.to_account_info(),mint_authority:ctx.accounts.signer.to_account_info(),update_authority:ctx.accounts.signer.to_account_info(),payer:ctx.accounts.signer.to_account_info(),system_program:ctx.accounts.system_program.to_account_info(),rent:ctx.accounts.rent.to_account_info(),},);
        let data_v2 = DataV2{
            name: name,
            symbol: symbol,
            uri: uri,
            seller_fee_basis_points:0,
            creators: None,
            collection: None,
            uses: None, 
        };

        create_metadata_accounts_v3(cpi_context, data_v2,false,true,None)?;

        //create master edition account
        let cpi_context = CpiContext::new(ctx.accounts.token_metadata_program.to_account_info(), CreateMasterEditionV3{edition:ctx.accounts.master_edition_account.to_account_info(), mint:ctx.accounts.mint.to_account_info(), update_authority: ctx.accounts.signer.to_account_info(),mint_authority: ctx.accounts.signer.to_account_info(),payer: ctx.accounts.signer.to_account_info(), metadata: ctx.accounts.metadata_account.to_account_info(),token_program: ctx.accounts.token_program.to_account_info(),system_program: ctx.accounts.system_program.to_account_info(), rent: ctx.accounts.rent.to_account_info(),},);
        create_master_edition_v3(cpi_context, None)?;

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
    /// CHECK - address
    #[account(mut, address=MetadataAccount::find_pda(&mint.key()).0,)]
    pub metadata_account: AccountInfo<'info>,
    /// CHECK - address
    #[account(mut,address=MasterEdition::find_pda(&mint.key()).0,)]
    pub master_edition_account: AccountInfo<'info>,

    /////////////////////////--------program declaration--------//////////////////////////////
    pub token_program: Program<'info, Token>, // new token program kinda like the mastermind
    pub associated_token_program: Program<'info, AssociatedToken>, // new Associated token program handles the creation of an associated token account (ata)
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>, // new system program responsible for creating all accounts
    pub rent: Sysvar<'info, Rent>, // new rent program responsible for paying rent logic
}
