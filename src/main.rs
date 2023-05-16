use std::{io, println};
mod methods;

fn main() {
    println!("Dame un número 1");

    let mut n1: String = String::new();
    io::stdin().read_line(&mut n1).expect("Failed to read line");

    println!("Dame un número 2");

    let mut n2: String = String::new();
    io::stdin().read_line(&mut n2).expect("Failed to read line");

    let number1: i64 = n1.trim().parse().unwrap();
    let number2: i64 = n2.trim().parse().unwrap();

    let res = methods::sumar(number1, number2);
    let guess = methods::get_random();

    println!("El valor de la suma: {:?}", res);
    println!("El numero es: {:?}", guess);
}
