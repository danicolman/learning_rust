fn main() {
    print_labeled_measurement(5, 'h');
    let add_two = plus_one(2);
    println!("Adding one to two gives you {add_two}.");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}