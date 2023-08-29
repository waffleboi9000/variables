use std::io;
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
    
    // testing math.
    
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("{} {}", "Sum is: ", {sum});
    println!("{} {}", "Difference is:", {difference});
    println!("{} {}", "Product is:",{product});
    println!("{} {}", "Quotient is:",{quotient});
    println!("{} {}", "Truncated is:",{truncated});
    println!("{} {}", "Remainder is:",{remainder});
    
    //let c = 'z';
    //let z: char = 'ℤ'; // with explicit type annotation
    //let heart_eyed_cat = '😻';
    
    //let x: (i32, f64, u8) = (500, 6.4, 1);
    
    //let five_hundred = x.0;

    //let six_point_four = x.1;

    //let one = x.2;

    //let months = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];

    //let array1: [i32; 5] = [1,2,3,4,5];

    //let first_of_array1 = array1[0];

    println!("Please enter an array index.");

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

}
