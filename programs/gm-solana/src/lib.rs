
//this imports the rust libraries needed
//hardcodes the address where the program will
//be deployed
use anchor_lang::prelude::*;

declare_id!("HnqfyUBbMowVeuBAig8MctDh3VvV9yoFAkGs8VpLDJcV");

//business logic is handled here
// #[program] is a Rust macro that abstracts
//the boilerplate and extra code needed to turn Rust into Anchor
#[program]
pub mod gm_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // &mut means we are letting the compiler know that we are mutating this value
        let base_account = &mut ctx.accounts.base_account;

        base_account.gm_count = 0;

        Ok(())
    }

    // receive a message and store it into gm_list with some metadata
    pub fn say_gm(ctx: Context<SayGm>, message: String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;

        // grab a copy of the input data
        let message = message.clone();
        // get the current Solana network time
        let timestamp = Clock::get().unwrap().unix_timestamp;
        // grab the public key of the transaction sender
        // * dereferences the pointer
        let user = *ctx.accounts.user.to_account_info().key;

        let gm = GmMessage {
            user,
            message,
            timestamp,
        };

        base_account.gm_list.push(gm);
        base_account.gm_count += 1;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize <'info> {
    #[account(init, payer=user, space= 64+ 1024)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct SayGm<'info>{
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    pub user: Signer<'info>,
}

//all the contents from the struct below
// are stored into gm_list
#[account]
pub struct BaseAccount {
    pub gm_count: u64,
    pub gm_list: Vec<GmMessage>,
}
// this contains a message, a sender and a timestamp
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct GmMessage{
    pub message: String,
    pub user: Pubkey,
    pub timestamp: i64,
}

