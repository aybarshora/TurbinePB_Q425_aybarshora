use anchor_lang::prelude::*;

declare_id!("EFKurnXFdcHeATysb79GmL5a7v53r4Uae531vXRrxfsj");

#[program]
pub mod counter_anchor {
    use super::*;

    pub fn initialize_counter(ctx: Context<InitializeCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.authority = ctx.accounts.payer.key(); // owner = creator by default
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let c = &mut ctx.accounts.counter;
        require_keys_eq!(c.authority, ctx.accounts.authority.key(), CounterError::Unauthorized);
        require!(c.count < u64::MAX, CounterError::Overflow);
        c.count = c.count.checked_add(1).unwrap();
        Ok(())
    }

    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        let c = &mut ctx.accounts.counter;
        require_keys_eq!(c.authority, ctx.accounts.authority.key(), CounterError::Unauthorized);
        require!(c.count > 0, CounterError::Underflow);
        c.count = c.count.checked_sub(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        space = 8 + Counter::INIT_SPACE,
        payer = payer
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: u64,
    pub authority: Pubkey, 
}

#[error_code]
pub enum CounterError {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Decrement underflow")]
    Underflow,
    #[msg("Increment overflow")]
    Overflow,
}