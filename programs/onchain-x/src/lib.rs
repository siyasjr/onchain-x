use anchor_lang::{accounts::program, prelude::*, solana_program::example_mocks::solana_sdk::system_program};
declare_id!("BCHAprg6MuE55yWqi75jYXUnWEvCiJrrwWXp1PdGGdqX");


#[derive(Accounts)]
pub struct  SendTweet<'info> {

  #[account(init, payer = author, space = Tweet::LEN)]
  pub tweet: Account<'info, Tweet>,
  #[account(mut)]
  pub author: Signer<'info>,

  #[account(address = system_program::ID)]
  pub system_program: Program<'info, System,>
}


#[account]
pub struct Tweet
 {
    author: Pubkey,
    timestamp: i64,
    topic: String,
    content: String, 
}


impl Tweet {
    pub const LEN: usize = 
      DISCRIMINATOR_LENGTH 
    + PUBLIC_KEY_LENGTH
    + TIME_STAMP_LENGTH
    + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH
    +STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH;
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const  TIME_STAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_TOPIC_LENGTH: usize = 200; //50 * 4 
const MAX_CONTENT_LENGTH: usize = 1120; //280 * 4



