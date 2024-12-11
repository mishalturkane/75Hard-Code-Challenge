use anchor_lang::prelude::*;
use solana_program::keccak;

// Define the program ID for the Solana program
#[program]
pub mod random_number_generator {
    use super::*;

    pub fn generate_random_number(ctx: Context<GenerateRandom>, range: u64) -> Result<u64> {
        let clock = Clock::get()?; // Access Solana's clock to get the current slot.
        let slot_seed = clock.slot.to_le_bytes();
        let blockhash_seed = ctx.accounts.blockhash.to_le_bytes();

        // Combine seeds to generate randomness
        let mut seed = Vec::new();
        seed.extend_from_slice(&slot_seed);
        seed.extend_from_slice(&blockhash_seed);

        let hash = keccak::hash(&seed); // Use keccak hash function for randomness
        let random_number = u64::from_le_bytes(hash.0[..8].try_into().unwrap());
        let bounded_random = random_number % range; // Restrict to range

        msg!("Generated Random Number: {}", bounded_random);
        Ok(bounded_random)
    }
}

#[derive(Accounts)]
pub struct GenerateRandom<'info> {
    /// CHECK: Blockhash is not stored, only used as a randomness source
    #[account(mut)]
    pub blockhash: AccountInfo<'info>,
}

#[error_code]
pub enum RandomError {
    #[msg("Invalid range for random number generation.")]
    InvalidRange,
}
