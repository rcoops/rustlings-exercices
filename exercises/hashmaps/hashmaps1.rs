// hashmaps1.rs
// A basket of fruits in the form of a hash map needs to be defined.
// The key represents the name of the fruit and the value represents
// how many of that particular fruit is in the basket. You have to put
// at least three different types of fruits (e.g apple, banana, mango)
// in the basket and the total count of all the fruits should be at
// least five.
//
// Make me compile and pass the tests!
//
// Execute `rustlings hint hashmaps1` or use the `hint` watch subcommand for a hint.


use std::collections::HashMap;


fn fruit_basket() -> HashMap<String, u32> {
    let fruit_options = ["apple", "banana", "mango"];

    let basket: HashMap<String, u32> = (0..5)
        .map(|i| (String::from(fruit_options[i % fruit_options.len()]), 1))
        .fold(
            HashMap::new(),
            |mut acc, (next_key, next_value)| {
                acc.entry(next_key).and_modify(|old_value| *old_value += next_value).or_insert(next_value);
                acc
            });

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
