fn main() {
    let x = 5;

    println!("The foundational value of x is {x}");

    let x = x + 1;

    println!("The value of x after shadowing is {x}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}