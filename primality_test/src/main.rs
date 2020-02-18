use std::io;

fn main() {
    println!("Enter a number to check if it's prime.");

    loop {
        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        let number: usize = match number.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("You must input an integer ({}).", e);
                continue;
            }
        };

        if number == 0 {
            println!("0 is not a prime number.");
            break;
        } else if number == 1 {
            println!("1 is not a prime number.");
            break;
        } else if number == 2 {
            println!("2 is a prime number.");
            break;
        } else {
            match check_prime(number) {
                false => println!("{} is not a prime number.", number),
                true => println!("{} is a prime number.", number)
            };
            break;
        };
    }
}

fn check_prime(n: usize) -> bool {
    run(n, n - 1)
}

fn run(n: usize, i: usize) -> bool {
    if i > 1 {
        match n % i {
            0 => false,
            _ => run(n, i - 1),
        }
    } else {
        true
    }
}
