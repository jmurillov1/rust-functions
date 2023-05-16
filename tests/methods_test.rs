#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use functions::fns;

    #[test]
    fn test_sumar() {
        assert_eq!(fns::sumar(1, 2), 3);
    }

    // #[test]
    // fn test_bad_add() {
    //     // This assert would fire and test will fail.
    //     // Please note, that private functions can be tested too!
    //     assert_eq!(get_random(10), 3);
    // }
}
