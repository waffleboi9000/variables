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


}
