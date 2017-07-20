fn main() {
	let str1 = String::from("abcd");
	let str2 = "zyxad";

	let result = longest(str1.as_str(), str2);
	println!("Longest is {:?}", result);
}

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
	if str1.len() > str2.len() {
		str1
	} else {
		str2
	}
}