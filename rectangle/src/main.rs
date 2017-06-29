#[derive(Debug)]
struct Rectangle {
	length: u32,
	width: u32
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.length * self.width
	}

	// fn can_hold(&self, other: &Rectangle) -> bool {
	// 	self.length > other.length && self.width > other.width
	// }

	fn square(size: u32) -> Rectangle {
		Rectangle { length: size, width: size }
	}
}

fn main() {
	// let rect = Rectangle {
	// 	length: 50,
	// 	width: 50
	// };

	// println!("rect is {:?}", rect);

    // println!("Area is {}", rect.area());

//     let ret1 = Rectangle { length: 50, width: 50 };
//     let ret2 = Rectangle { length: 40, width: 40 };
//     let ret3 = Rectangle { length: 40, width: 60 };

//     println!("ret1 hold ret2.. {}", ret1.can_hold(&ret2));
//     println!("ret1 hold ret3.. {}", ret1.can_hold(&ret3));

	let square = Rectangle::square(3);

	println!("Square is {:?}", square);

	println!("Area is {}", square.area());
}

// fn area(ret: &Rectangle) -> u32 {
//     ret.length * ret.width
// }
