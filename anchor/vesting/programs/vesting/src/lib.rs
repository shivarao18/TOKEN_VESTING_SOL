use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{ self, Mint, TokenAccount, TokenInterface, TransferChecked};

declare_id!("...");

#[program]
pub mod vesting {
    use super::*;

    pub fn create_vesting_account( ctx : Context<CreateVestingAccount>, company_name : String) -> Result<()> {
        *ctx.accounts.vesting_account = VestingAccount {
            owner: ctx.accounts.signer.key(),
            mint: ctx.accounts.mint.key(),
            treasury_token_account: ctx.accounts.treasury_token_account.key(),
            company_name,
            treasury_bump: ctx.bumps.treasury_token_account,
            bump: ctx.bumps.vesting_account,   
        };

        Ok(())
    }

    
    
}

#[derive(Account)]
#[instruction(company_name: String)]
pub CreateVestingAccount<'info> {
    #[account(mut)]
    pub signer : Signer<'info>,

    pub mint : InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = signer,
        space = 8 + VestingAccount::INIT_SPACE,
        seeds = [b"company_name".as_ref()],
        bump,
    )]
    pub vesting_account: Account<'info, VestingAccount>,

    #[account(
        init,
        token::mint = mint,
        token::authority = treasury_token_account,
        payer = signer,
        seeds = [b"vesting_treasury" , company_name.as_bytes()],
        bump,
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    
}

#[account]
#[derive(InitSpace)]
pub struct VestingAccount {
    pub owner: Pubkey,
    pub mint : Pubkey,
    pub treasury_token_account: Pubkey,
    #[max_len(50)]
    pub company_name: String,
    pub treasury_bump: u8,
    pub bump: u8,
}