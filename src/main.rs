use rand::{thread_rng, Rng};

fn main() {
    let mut numbers: [u32; 10] = [0; 10];
    init_numbers(&mut numbers);
    display_numbers(&numbers);

    let largest = largest(&numbers);
    println!("the largest number is {largest}.");
}

fn init_numbers(numbers: &mut [u32]) {
    for i in 0..numbers.len() {
        numbers[i] = thread_rng().gen_range(0..100);
    }
}

fn display_numbers(numbers: &[u32]) {
    for i in 0..numbers.len() {
        print!("{}, ", numbers[i]);
    }
    println!();
}

fn largest(numbers: &[u32]) -> &u32 {
    let mut cur_largest: &u32 = &0;
    for num in numbers {
        if num >= cur_largest {
            cur_largest = num;
        }
    }
    cur_largest
}
