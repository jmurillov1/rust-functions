use rand::Rng;
pub fn sumar(number1: i64, number2: i64) -> i64 {
    number1 + number2
}

pub fn get_random(number: i64) -> i64 {
    let mut random: i64 = 0;

    for _ in 1..=number {
        random += rand::thread_rng().gen_range(1..=100);
        println!("Numero {:?}", random);
    }

    random/number
}
