use rand::{thread_rng, Rng};
use std::io::stdin;

const READ_FAIL: &str = "Failed to read input.";

fn main() {
    clear_cli();
    println!("------------ Find The Number ------------\n");

    let size = get_array_size();
    let min_or_max = get_min_or_max();

    let mut numbers = vec![0; size.try_into().unwrap()];
    for i in 0..numbers.len() {
        numbers[i] = thread_rng().gen_range(0..=100);
    }
    let number = find_number(&numbers, &min_or_max);

    if numbers.len() > 0 {
        display_numbers(&numbers);
        println!("{min_or_max} = {number}",);
    } else {
        println!("can't look for a number in a empty list...");
    }
}

fn clear_cli() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn get_array_size() -> u32 {
    let size: u32;
    loop {
        println!("How many random numbers should be in the list?");
        let mut input = String::new();

        stdin().read_line(&mut input).expect(READ_FAIL);

        size = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                clear_cli();
                println!("Please enter a valid number.");
                continue;
            }
        };
        break;
    }
    clear_cli();
    size
}

fn get_min_or_max() -> String {
    let mut input = String::new();
    loop {
        println!("Look for min or max?");
        stdin().read_line(&mut input).expect(READ_FAIL);

        input = match input.trim().parse() {
            Ok(s) => s,
            Err(_) => continue,
        };

        if input.as_str() == "min" || input.as_str() == "max" {
            break;
        }
        input = String::new();
        clear_cli();
        println!("Please enter \"min\" or \"max\".");
    }
    clear_cli();
    input
}

fn find_number<'a>(numbers: &'a [u32], min_or_max: &'a String) -> &'a u32 {
    let mut cur_number: &u32 = if min_or_max == "min" { &1000 } else { &0 };
    for num in numbers {
        if min_or_max == "min" {
            if num < cur_number {
                cur_number = num;
            }
        } else {
            if num > cur_number {
                cur_number = num;
            }
        }
    }
    cur_number
}

fn display_numbers(numbers: &[u32]) -> () {
    print!("numbers = [");
    for i in 0..numbers.len() {
        if i < numbers.len() - 1 {
            print!("{}, ", numbers[i]);
        } else {
            print!("{}", numbers[i]);
        }
    }
    println!("]");
}
