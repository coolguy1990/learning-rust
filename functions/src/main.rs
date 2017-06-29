//fn main() {
//    print_number(5);
//}

//fn print_number(x: i32) {
//    println!("Number is: {}", x);
//}
//

fn main () {
    let x: i32 = add_number(10);

    println!("Number is: {}", x);
}

fn add_number(x: i32) -> i32 {
    x + 10
}
