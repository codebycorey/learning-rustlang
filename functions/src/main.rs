fn print_measure(value: i32, label: char) {
    println!("The measurement is: {}{}", value, label);
}

fn func_params(x: i32) {
    println!("The Value of x is {}", x);
}

fn test_function() {
    println!("Test function'");
}

fn statements_expressions() {
    let y = 6;
    println!("Value of y: {}", y);

    let z = {
        let x = 6;
        x + 1
    };
    println!("Value of z: {}", z);
}

fn five() -> i32 {
    // Expressions done use ; or it turns into a statement
    5
}

fn plus_one(x: i32) -> i32 {
   x + 1
}

fn main() {
    println!("Hello, world!");
    test_function();
    func_params(52);
    print_measure(5, 'h');
    statements_expressions();

    let x = five();
    println!("Value of x:five(): {}", x);

    let p_o = plus_one(5);
    println!("Value of p_o: {}", p_o);
}
