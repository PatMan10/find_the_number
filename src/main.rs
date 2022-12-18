use rand::{thread_rng, Rng};

fn main() {
    let mut numbers: [u32; 10] = [0; 10];
    init_numbers(&mut numbers);
    display_numbers(&numbers);

    let largest = get_largest(&numbers);
    if numbers.len() != 0 {
        println!("largest = {largest}");
    }
}

fn init_numbers(numbers: &mut [u32]) -> () {
    for i in 0..numbers.len() {
        numbers[i] = thread_rng().gen_range(0..100);
    }
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

fn get_largest(numbers: &[u32]) -> &u32 {
    let mut cur_largest: &u32 = &0;
    for num in numbers {
        if num > cur_largest {
            cur_largest = num;
        }
    }
    cur_largest
}
