use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("BCHAprg6MuE55yWqi75jYXUnWEvCiJrrwWXp1PdGGdqX");


#[program]
pub mod onchain_x {

  use crate::ErrorCode;

use super::*; 
  pub fn send_tweet(ctx: Context<SendTweet>, topic: String, content: String) -> Result<()> {
    let tweet = &mut ctx.accounts.tweet;
    let author = &ctx.accounts.author;
    let clock = Clock::get()?;

    if topic.chars().count() >=50 {
      
      return Err(ErrorCode::TopicTooLong.into());

    }

    if content.chars().count() >= 280 {
      
      return Err(ErrorCode::ContentTooLong.into());
    }


    tweet.author = author.key();
    tweet.topic = topic;
    tweet.content = content; 
    tweet.timestamp = clock.unix_timestamp;

    Ok(())

  }
  
}

#[derive(Accounts)]
pub struct  SendTweet<'info> {

  #[account(init, payer = author, space = Tweet::LEN)]
  pub tweet: Account<'info, Tweet>,
  #[account(mut)]
  pub author: Signer<'info>,

  #[account(address = system_program::ID)]
  pub system_program: Program<'info, System>
}


#[account]
pub struct Tweet
 {
    pub author: Pubkey,
    pub timestamp: i64,
    pub topic: String,
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


#[error_code]
pub enum ErrorCode {
  #[msg("The provided topic should be 50 character long maximum")]
  TopicTooLong,

  #[msg("The provided content should be 280 characters long maximum")]
  ContentTooLong,
    
}



