use anchor_lang::prelude::*;
use  anchor_lang::accounts;

#[account]

pub struct Tweet
 {
    author: Pubkey,
    timestamp: i64,
    topic: String,
    content: String, 
}


