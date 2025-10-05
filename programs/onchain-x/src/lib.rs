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

const DISCRIMINATOR_LENGTH: usize = 4;
const PUBLIC_KEY_LENGTH: usize = 32;
const  TIME_STAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_TOPIC_LENGTH: usize = 200; //50 * 4 
const MAX_CONTENT_LENGTH: usize = 1120; //280 * 4



