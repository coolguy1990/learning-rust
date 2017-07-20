fn main() {
	// let s1 = String::from("hello");
	// let s2 = String::from(", world");

	// // s1 is not accessible anymore
	// //let s3 = s1 + &s2;

	// //format only takes reference, so s1 is still accessible
	// let s3 = format!("{}{}", s1, s2);

	// println!("{:?}", s1);
	// for b in "नमस्ते".bytes() {
	//     println!("{}", b);
	// }	
	for b in "नमस्ते".chars() {
	    println!("{}", b);
	}
}