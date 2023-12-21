pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[allow(dead_code)]
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn greetings(name: &str) -> String {
    format!("Greetings, {}", name)
    // format!("Greetings, human")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("this will fail!")
    // }

    #[test]
    fn one_is_smaller_than_two() {
        assert!(1 < 2);
    }

    #[test]
    fn equality() {
        assert_eq!(2, 2);
    }

    #[test]
    fn not_equal() {
        assert_ne!(1, 2);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greetings("Carol");
        assert!(
            result.contains("Carol"),
            "greeting did not contain name, value was: {}",
            result
        )
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn guess_greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        println!("will this be printed?"); // no, it won't
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2))
    }
}
