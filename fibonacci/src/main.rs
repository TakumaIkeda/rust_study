fn main() {
    println!("Enter the number.");

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number.");
            return;
        }
    };

    println!("The {}th Fibonacci number is {}", input, fibonacci(input));
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}
