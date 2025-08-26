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

    let mut y = ("cat", 2, true);

    println!("The first item is {}; the second item is {}, and the third item is {}.", {y.0}, {y.1}, {y.2});

    y.2 = false;

    println!("Now the third item is {}.", {y.2});

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    println!("My birthday is in the third month, which is {}.", {months[2]});

}