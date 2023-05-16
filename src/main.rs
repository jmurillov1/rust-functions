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

    let sumar2 = |number1: i64, number2: i64| number1 + number2;

    println!("La suma: {:?}", sumar2(number1, number2));

    let random_number = methods::sumar(number1, number2);
    let guess = methods::get_random(random_number);

    println!("El valor de la suma: {:?}", random_number);
    println!("El numero es: {:?}", guess);
}
