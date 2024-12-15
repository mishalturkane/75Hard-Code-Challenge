use anchor_lang::prelude::*;

declare_id!("YourProgramPublicKeyHere");

#[program]
pub mod simple_counter {
    use super::*;

    // Function to initialize the counter
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.value = 0;
        Ok(())
    }

    // Function to increment the counter
    pub fn increment(ctx: Context<UpdateCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.value += 1;
        Ok(())
    }
}

// Define the account structure to hold the counter data
#[account]
pub struct Counter {
    pub value: u64,
}

// Context for initializing the counter
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)] // 8 bytes for discriminator, 8 bytes for the counter value
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Context for updating the counter
#[derive(Accounts)]
pub struct UpdateCounter<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}
