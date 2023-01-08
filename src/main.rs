use std::io;

fn main() {
    loop {
        println!("What is the temperature to convert?");
        println!("Examples: 70F, 20C, 294K.");

        let mut temp = String::new();

        // Read from stdin
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        // Remove whitespace
        let temp = temp.trim();

        // Check if there are least 2 chars
        // (presumably 1 number and 1 unit)
        if temp.len() < 2 {
            continue;
        }

        let (num, unit) = temp.split_at(temp.len() - 1);

        // Validate the number
        let num = match num.parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Prevent large numbers that overflow
        if num > 5000 {
            continue;
        }

        // Validate the unit and convert
        match unit.chars().next().unwrap() {
            'C' => {
                convert_from_c(num);
                break;
            }
            'F' => {
                convert_from_f(num);
                break;
            }
            'K' => {
                convert_from_k(num);
                break;
            }
            _ => continue,
        };
    }
}

// Conversion equation:
// (C/5) = (F-32)/9 = (K-273)/5

fn convert_from_c(num: i32) {
    // F = (9C/5)+32
    // K = C+273
    println!("{} Fahrenheit", ((9 * num) / 5) + 32);
    println!("{} Kelvin", num + 273);
}

fn convert_from_f(num: i32) {
    // C = 5*(F-32)/9
    // K = ((5*(F-32))/9) + 273
    println!("{} Celsius", (5 * (num - 32)) / 9);
    println!("{} Kelvin", ((5 * (num - 32)) / 9) + 273);
}

fn convert_from_k(num: i32) {
    // C = K-273
    // F = ((9*(K-273))/5)-32
    println!("{} Celsius", num - 273);
    println!("{} Fahrenheit", ((9 * (num - 273)) / 5) - 32);
}
