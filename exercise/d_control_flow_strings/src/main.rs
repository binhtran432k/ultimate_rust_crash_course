// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_mut, unused_variables)]

fn main() {
    // This collects any command-line arguments into a vector of Strings.
    // For example:
    //
    //     cargo run apple banana
    //
    // ...produces the equivalent of
    //
    //     vec!["apple".to_string(), "banana".to_string()]
    let args: Vec<String> = std::env::args().skip(1).collect();

    // This consumes the `args` vector to iterate through each String
    for arg in args {
        match arg.as_str() {
            "sum" => sum(),
            "double" => double(),
            _ => count(arg),
        }
    }
}

fn sum() {
    let mut sum = 0;
    for n in 7..=23 {
        sum += n;
    }
    println!("The sum is {}", sum);
}

fn double() {
    let mut count = 0;
    let mut x = 1;
    while x <= 500 {
        count += 1;
        x *= 2;
    }
    println!(
        "You can double x {} times until x is larger than 500",
        count
    );
}

fn count(arg: String) {
    // Challenge: Use an unconditional loop (`loop`) to print `arg` 8 times, and then break.
    // You will need to count your loops, somehow.  Run it with `cargo run bananas`
    let mut count = 0;
    loop {
        if count == 8 {
            break;
        }
        print!("{} ", arg); // Execute this line 8 times, and then break. `print!` doesn't add a newline.
        count += 1;
    }
    println!(); // This will output just a newline at the end for cleanliness.
}
