extern crate traits;
use traits::Summarizable;
fn main() {
	let tweet = traits::Tweet {
		username: String::from("Test"),
		content: String::from("some tweet"),
		reply: false,
		retweet: false
	};

	println!("1 new tweet: {:?}", tweet.summary());
}