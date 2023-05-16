use rand::Rng;

pub(crate) fn sumar(number1: i64, number2: i64) {
    let _ = number1 + number2;
}

pub(crate) fn get_random() {
    rand::thread_rng().gen_range(1..=100);
}
