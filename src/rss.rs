use rss::{Channel, Item};
use std::error::Error;

pub fn fetch_feed_items(url: &str) -> Result<Vec<Item>, Box<dyn Error>> {
    let body = reqwest::blocking::get(url)?.bytes()?;

    let channel = Channel::read_from(&body[..])?;

    Ok(channel.into_items())
}
