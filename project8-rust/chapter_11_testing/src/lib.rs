pub fn plus_values(a: i32,b : i32) -> i32 {
    a + b
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// run with "cargo test" in the project dir. Command options
// cargo test -- --test-threads=1   => specify how many thread to use to run the tests
// cargo test -- --show-output      => always should println output even on success test runs
// cargo test libs::tests::add_two  => only run tests starting with add_two in specific module. Can drop the module name too
// cargo test -- --ignored          => run only ignored tests
// cargo test -- --include-ignored  => run both ignored and non-ignored tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn boolean() {
        assert!((3 + 2) == 5)
    }

    #[test]
    fn plus_values_sums() {
        assert_eq!(10, plus_values(4, 6), "sum must be 10");
    }

    // Specify error message for panic tests
    #[test]
    #[should_panic(expected = "Make this test fail")]
    fn panics() {
        panic!("Make this test fail");
    }

    #[test]
    fn can_hold_larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn can_hold_smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn add_two_with_2() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_two_with_3() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn add_two_with_100() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    #[ignore]
    fn ignored_test() {
        assert!(5 == 5);
    }

    // you can test "internal" functions due to  use super::*; with Rust
    #[test]
    fn internal_adder_with_2() {
        assert_eq!(4, internal_adder(2, 2));
    }

    #[test]
    fn internal_adder_with_3() {
        assert_eq!(5, internal_adder(2, 3));
    }

    #[test]
    fn internal_adder_with_100() {
        assert_eq!(102, internal_adder(2, 100));
    }
}
