use std::cmp::PartialOrd;

// fn largest_i32(list: &[i32]) -> i32 {
// 	let mut largest = list[0];

// 	for &num in list.iter() {
// 		if num > largest {
// 			largest = num;
// 		}
// 	}

// 	largest
// }

// fn largest_char(list: &[char]) -> char {
// 	let mut largest = list[0];

// 	for &num in list.iter() {
// 		if num > largest {
// 			largest = num;
// 		}
// 	}

// 	largest
// }

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
	let mut largest = list[0];

	for &num in list.iter() {
		if num > largest {
			largest = num;
		}
	}

	largest
}

fn main() {
	// let numbers = vec![20, 30, 34, 550, 100];

	// let mut largest = numbers[0];

	// for num in numbers {
	// 	if num > largest {
	// 		largest = num;
	// 	}
	// }

	// println!("Largest is {:?}", largest);

	// let numbers = vec![20, 40, 1, 99, 100];
	// let result = largest(&numbers);
	// println!("Largest is {:?}", result);

	// let numbers = vec![10, 20, 1, 20, 120, 11];
	// let result = largest_i32(&numbers);
	// println!("Largest is {:?}", result);

	// let chars = vec!['y', 'm', 'a', 'q'];
 //    let result = largest_char(&chars);
 //    println!("The largest char is {}", result);

 	let numbers = vec![10, 20, 1, 20, 120, 11];
	let result = largest(&numbers);
	println!("Largest is {:?}", result);

	let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars);
    println!("The largest char is {}", result);
}

