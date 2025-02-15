use {crate::state::*, anchor_lang::prelude::*};

/// Accounts required by the `thread_delete` instruction.
#[derive(Accounts)]
pub struct ThreadDelete<'info> {
    /// The authority (owner) of the thread.
    #[account(
        constraint = authority.key().eq(&thread.authority) || authority.key().eq(&thread.key())
    )]
    pub authority: Signer<'info>,

    /// The address to return the data rent lamports to.
    #[account(mut)]
    pub close_to: SystemAccount<'info>,

    /// The thread to be deleted.
    #[account(
        mut,
        seeds = [
            SEED_THREAD,
            thread.authority.as_ref(),
            thread.id.as_slice(),
        ],
        bump = thread.bump
    )]
    pub thread: Account<'info, Thread>,
}

pub fn handler(
    ctx: Context<ThreadDelete>
) -> Result<()> {
    let close_to: &SystemAccount = &mut ctx.accounts.close_to;
    let thread: &mut Account<Thread> = &mut ctx.accounts.thread;

    thread.close(close_to.to_account_info())?;
    Ok(())
}