use std::env;

fn main() {
    // 1) Collect command-line arguments into a Vec<String>
    let arguments: Vec<String> = env::args().collect();

    // Expect exactly one user argument: the positive integer
    if arguments.len() != 2 {
        eprintln!("Usage: {} <positive_integer>", arguments[0]);
        return; // end gracefully (no non-zero exit code)
    }

    // 2) Parse and validate the number
    let mut input_number: u128 = match arguments[1].parse() {
        Ok(parsed_number) if parsed_number > 0 => parsed_number,
        Ok(_) => {
            eprintln!("The number must be > 0.");
            return;
        }
        Err(_) => {
            eprintln!("'{}' is not a valid integer.", arguments[1]);
            return;
        }
    };

    // Special case: 1 has no prime factors
    if input_number == 1 {
        println!("0");
        return;
    }

    // 3) Count prime factors using simple trial division
    let mut prime_factor_count: u32 = 0;

    // Remove all factors of 2
    while input_number % 2 == 0 {
        prime_factor_count += 1;
        input_number /= 2;
    }

    // Try odd factors: 3, 5, 7, ... up to sqrt(n)
    // Use "factor <= n / factor" to avoid overflow of factor*factor
    let mut trial_factor: u128 = 3;
    while trial_factor <= input_number / trial_factor {
        while input_number % trial_factor == 0 {
            prime_factor_count += 1;
            input_number /= trial_factor;
        }
        trial_factor += 2;
    }

    // If a large prime remains, it counts as one more factor
    if input_number > 1 {
        prime_factor_count += 1;
    }

    // 4) Print ONLY the count
    println!("{prime_factor_count}");
}
