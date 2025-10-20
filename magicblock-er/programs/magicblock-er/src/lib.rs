#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::anchor::ephemeral;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;


pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("F3H1NHuNn35n5R1Cus2uzCraBJfg772ogTdwoyNGJYfz");


#[ephemeral]
#[program]
pub mod magicblock_er {
    use super::*;

    pub fn initialize(ctx: Context<InitUser>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;
        Ok(())
    }

    pub fn update(ctx: Context<UpdateUser>, seed: [u8; 32]) -> Result<()> {
        ctx.accounts.update(seed)?;

        Ok(())
    }

    pub fn update_commit(ctx:Context<UpdateCommit>, new_data: u64) -> Result<()> {
        ctx.accounts.update_commit(new_data)?;

        Ok(())
    }

    pub fn delegate(ctx: Context<Delegate>) -> Result<()> {
        ctx.accounts.delegate()?;
        
        Ok(())
    }

    pub fn undelegate(ctx: Context<Undelegate>) -> Result<()> {
        ctx.accounts.undelegate()?;
        
        Ok(())
    }

    pub fn close(ctx: Context<CloseUser>) -> Result<()> {
        ctx.accounts.close()?;
        
        Ok(())
    }

    pub fn request_randomness(ctx: Context<FetchRandomness>, seed: u8) -> Result<()> {
        FetchRandomness::handler(ctx, seed)
    }
}