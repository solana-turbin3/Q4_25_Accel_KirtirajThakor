#![no_std]
#![allow(unexpected_cfgs)]

use pinocchio_pubkey::declare_id;

mod entrypoint;
mod state;
mod instructions;
mod helper;
mod error;
mod constants;

declare_id!("8PUUQdLNp2KBgKSRyaWX8n5EEsfh1oRAYNCXJ6n1QqxG");
