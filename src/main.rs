use std::io;
mod methods;

fn main() {
    println!("Dame un número 1!");

    let mut n1 = String::new();

    println!("Dame un número 2");

    let mut n2 = String::new();

    io::stdin().read_line(&mut n1).expect("Failed to read line");

    io::stdin().read_line(&mut n2).expect("Failed to read line");

    let number1: i64 = n1.parse().unwrap();
    let number2: i64 = n2.parse().unwrap();

    let res = methods::sumar(number1, number2);

    println!("La suma es:!{:?}", res);
    let guess = methods::get_random();

    println!("El numero es:!{:?}", guess);
}
