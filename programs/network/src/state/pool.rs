use anchor_lang::{prelude::*, AnchorDeserialize};

pub const SEED_POOL: &[u8] = b"pool";

const DEFAULT_POOL_SIZE: u64 = 1;

/**
 * Pool
 */

#[account]
#[derive(Debug)]
pub struct Pool {
    pub id: u64,
    pub size: u64,
    pub workers: Vec<Pubkey>,
}

impl Pool {
    pub fn pubkey(id: u64) -> Pubkey {
        Pubkey::find_program_address(&[SEED_POOL, id.to_be_bytes().as_ref()], &crate::ID).0
    }
}

/**
 * PoolSettings
 */

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct PoolSettings {
    pub size: u64,
}

/**
 * PoolAccount
 */

pub trait PoolAccount {
    fn pubkey(&self) -> Pubkey;

    fn init(&mut self, id: u64) -> Result<()>;

    fn rotate(&mut self, worker: Pubkey) -> Result<()>;

    fn update(&mut self, settings: &PoolSettings) -> Result<()>;
}

impl PoolAccount for Account<'_, Pool> {
    fn pubkey(&self) -> Pubkey {
        Pool::pubkey(self.id)
    }

    fn init(&mut self, id: u64) -> Result<()> {
        self.id = id;
        self.size = DEFAULT_POOL_SIZE;
        self.workers = Vec::new();
        Ok(())
    }

    fn rotate(&mut self, worker: Pubkey) -> Result<()> {
        let old_size = self.size;
        // Push new worker into the pool.
        self.workers.push(worker);
        // Drain pool to the configured size limit.
        self.workers.truncate(old_size as usize);
        Ok(())
    }

    fn update(&mut self, settings: &PoolSettings) -> Result<()> {
        let new_size = settings.size;  // Store the size locally first
        self.size = new_size; 

        // Drain pool to the configured size limit.
        self.workers.truncate(new_size as usize);
        Ok(())
    }
}
