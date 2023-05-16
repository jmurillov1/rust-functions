use rand::Rng;
pub fn sumar(number1: i64, number2: i64) -> i64 {
    number1 + number2
}

pub fn get_random() -> i64 {
    rand::thread_rng().gen_range(1..=100)
}
