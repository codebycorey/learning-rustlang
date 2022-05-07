fn if_statement() {
    let x = 3;

    if x < 5 {
        print!("True");
    } else {
        print!("False");
    }
}

fn is_greater_than(x: i32, y: i32) -> bool {
    x > y
}

fn fail_non_bool() {
    // let number = 3;
    //
    // if number {
    //     println!("Truthy!");
    // }
}

fn not_equal_zero() {
    let number = 3;

    if number != 0 {
        println!("Number:{} is not zero!!!", number)
    }
}

fn multiple_if_statements() {
    let number = 6;

    if number % 4 == 0 {
        println!("number: {} is divisible by 4", number);
    } else if number % 3 == 0 {
        println!("number: {} is divisible by 3", number);
    } else if number % 2 == 0 {
        println!("number: {} is divisible by 2", number);
    } else {
        println!("Number: {} didn't work", number);
    }
}

fn use_if_right_of_let() {
    let condition = true;
    // Must be same data type, compiler will help
    let number = if condition { 5 } else { 6 };

    println!("The value is: {}", number);
}

fn infinite() {
    // loop {
    //     println!("again!");
    // }
}

fn break_continue_label() {
    let mut count = 0;

    'count_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'count_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {}", count);
}

fn return_value_of_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The loop result is {}", result);
}

fn conditional_loop() {
    let mut number = 3;

    while number !=  0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("Lift off !!");
}

fn for_loop_array() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("Value is: {}", element);
    }
}

fn for_range_rev() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("Lift off !! 2");
}

fn main() {
    println!("Hello, world!");

    if_statement();

    let is_greater = is_greater_than(5, 3);
    println!("Is Greater: {}", is_greater);

    fail_non_bool();

    not_equal_zero();

    multiple_if_statements();

    use_if_right_of_let();

    infinite();

    break_continue_label();

    return_value_of_loop();

    conditional_loop();

    for_loop_array();

    for_range_rev();
}
