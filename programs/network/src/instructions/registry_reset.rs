use {
  crate::state::*,
  anchor_lang::{prelude::*, solana_program::system_program},
};
use std::mem::size_of;

#[derive(Accounts)]
pub struct RegistryReset<'info> {
  #[account(
    mut,
    address = config.admin
  )]
  pub admin: Signer<'info>,

  #[account(
      address = Config::pubkey(),
      has_one = admin
  )]
  pub config: Account<'info, Config>,

  #[account(
      mut,
      seeds = [SEED_REGISTRY],
      constraint = registry.current_epoch.gt(&0),
      bump
  )]
  pub registry: Account<'info, Registry>,

  #[account(
      init_if_needed,
      seeds = [
          SEED_SNAPSHOT,
          (0 as u64).to_be_bytes().as_ref(),
      ],
      bump,
      payer = admin,
      space = 8 + size_of::<Snapshot>(),
  )]
  pub snapshot: Account<'info, Snapshot>,

  #[account(
    init_if_needed,
    payer = payer,
    space = 8 + std::mem::size_of::<RegistryFee>(),
    seeds = [SEED_REGISTRY_FEE, registry.key().as_ref()],
    bump
  )]
  pub registry_fee: Account<'info, RegistryFee>,

  #[account(address = system_program::ID)]
  pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegistryReset>) -> Result<()> {
  // Get accounts
  let registry = &mut ctx.accounts.registry;
  let registry_fee = &mut ctx.accounts.registry_fee;
  let snapshot = &mut ctx.accounts.snapshot;

  // Reset accounts to their initial state
  registry.reset()?;
  registry_fee.init(registry.key())?;
  snapshot.init(0)?;

  Ok(())
}
