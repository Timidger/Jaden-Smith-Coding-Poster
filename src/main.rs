extern crate twitter_api as twitter;
extern crate oauth_client as oauth;

use oauth::Token;

/// API keys embedded in source file, I'm lazy
const CONSUMER_KEY: &'static str = "";
const CONSUMER_SECRET: &'static str = "";
const ACCESS_KEY: &'static str = "";
const ACCESS_SECRET: &'static str = "";

const TWEETS_FILE: &'static str = "tweets.txt";

fn main() {
    let consumer = Token::new(CONSUMER_KEY, CONSUMER_SECRET);
    let access = Token::new(ACCESS_KEY, ACCESS_SECRET);

    // updates the status
    let status = "hello world from a rust script";
    twitter::update_status(&consumer, &access, &status).unwrap();
}
