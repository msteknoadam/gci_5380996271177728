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
            let inputlen = trimmed.len() as u32;
            match trimmed.parse::<u32>() {
                Ok(input_int) => {
                    // println!("The number you entered: {}", input_int);
                    // too many type conversions...
                    let mut nums = Vec::new();
                    let mut sum = 0;
                    let mut sumlist = Vec::new();
                    for input_num in trimmed.chars() {
                        let input_num_int = input_num.to_digit(10).unwrap();
                        nums.push(input_num_int);
                        sum += input_num_int.pow(inputlen);
                        let text = format!(
                            "{current_num}^{input_len}",
                            current_num = input_num_int,
                            input_len = inputlen
                        );
                        sumlist.push(text);
                    }
                    // println!("{:?}", sumlist);
                    if sum == input_int {
                        println!(
                            "{} is an Armstrong Number because: {} = {} = {}",
                            input_int,
                            sumlist.join(" + "),
                            sum,
                            input_int
                        );
                    } else {
                        println!(
                            "{} is not an Armstrong Number because: {} = {} != {}",
                            input_int,
                            sumlist.join(" + "),
                            sum,
                            input_int
                        );
                    }
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
