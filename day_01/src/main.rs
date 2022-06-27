mod read;
mod lib;


fn main() {

    let values = read::read_int_lines("input.txt".to_owned());

    // Part 1
    let sum_values : i64 = lib::sum_values(None, &values);
    println!("\r🔊 Resulting frequency: '{}' (Part 1)", sum_values);
    
    // Part 2
    let repeated_value : i64 = lib::first_repeated_current(None, &values);
    println!("\r🔊 Repeated frequency: '{}' (Part 2)", repeated_value);
}