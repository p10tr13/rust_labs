use std::{fs::File, io::{self, Write}};
use rand::Rng;

fn main() {

    let result = loop{
        let mut guess = String::new();
        println!("Enter the number!");
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let mut number: u64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(error) => {
                println!("{}", error);
                break true;
            }
        };

        if number == 0 {
            break false;
        }
        
        number += rand::thread_rng().gen_range(0..=5);
        println!("New x value: {}", number);

        let array:[u64; 10] = pow_table(number);
        println!("{:?}", array);
        let mut collatz_res_arr = [false; 10];
        for i in 0..10 {
            collatz_res_arr[i] = is_collatz(array[i], 100);
        } 
        println!("{:?}", collatz_res_arr);

        let (desc, avg, has_prime) = analyze_results(array);
        println!("Description: {desc}, Average: {avg}, Has prime: {has_prime}");

        match save_to_file(collatz_res_arr, "xyz.txt".to_string()) {
            Ok(..) => continue,
            Err(error) => {
                println!("{}", error);
                break true;
            }
        };
    };

    if result {
        println!("Loop ended because of error.")
    }
    else {
        println!("Loop ended because user wanted it to end.")
    }

}

fn pow_table<const LEN: usize>(x: u64) -> [u64; LEN] {
    let mut arr = [x; LEN];
    let mut val = x;
    for item in arr.iter_mut() {
        *item = val;
        val *= x;
    }
    arr
}

fn is_collatz(mut x: u64, limit: u32) -> bool {
    for _ in 0..=limit {
        x = collatz(x);
        if x == 1 {
            return true;
        }
    }
    false
}

fn collatz(x: u64) -> u64 {
    if x % 2 == 1 {
        return 3 * x + 1;
    }
    x/2
}

fn save_to_file(arr: [bool; 10], file_name: String) -> io::Result<()>{
    let mut file = File::create(file_name).expect("Unable to create or open file.");
    let mut text = String::new();

    for value in arr.iter() {
        text.push_str(&value.to_string());
        text.push(',');
    }

    if text.ends_with(',') {
        text.pop();
    }

    file.write_all(text.as_bytes()).expect("Unable to write to file.");
    Ok(())
}

fn analyze_results(values: [u64;10]) -> (String, f64, bool) {
    let mut sum = 0;
    let mut found_prime = false;

    for &value in values.iter() {
        sum += value;
    }

    'outer: for &value in values.iter() {
        if value <= 1 {
            continue 'outer;
        }

        if value == 2 || value == 3 {
            found_prime = true;
            break 'outer;
        }

        if value % 2 == 0 || value % 3 == 0 {
            continue 'outer;
        }

        let mut i = 5;
        loop {
            if value % i == 0 || value % (i + 2) == 0 {
                continue 'outer;
            }
            i += 6;
            if i > (value as f64).sqrt() as u64 {
                found_prime = true;
                break 'outer;
            }
        }
    }

    let desc = if found_prime {
        "Found prime".to_string()
    } else {
        "Not found prime".to_string()
    };

    (desc, sum as f64 / values.len() as f64, found_prime)
}