use std::io;

fn main() {
    println!("Hello! Welcome to Armstrong Number checker!");
    inputandprocess();
}

fn inputandprocess() {
    println!("Please enter a number to check if it's a Armstrong Number or not:");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed = input.trim();
            match trimmed.parse::<u32>() {
                Ok(input_int) => {
                    println!("The number you entered: {}", input_int);
                }
                Err(..) => {
                    println!("Your input doesn't look like a number. Please enter a number without any commas etc.");
                    inputandprocess();
                }
            };
        }
        Err(e) => println!(
            "Something went wrong when processing your input. Error: {}",
            e
        ),
    }
}
