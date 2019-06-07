pub mod helpers;

fn main() {
    let a = 7;
    let b = 6;

    let result = helpers::add(a, b);

    assert_eq!(result, 13);

    println!("result: {}", result);
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck::{quickcheck, QuickCheck, StdGen, StdThreadGen};

    #[test]
    fn test_cases() {
        assert_eq!(helpers::add(1,1), 2);
        assert_eq!(helpers::add(2,3), 5);
        assert_eq!(helpers::add(10,10), 20);
        assert_eq!(helpers::add(100,200), 300);
    }

    // #[test]
    // quickcheck! {
    //     fn prop_test_add(x: u32, y: u32) -> bool {
    //         helpers::add(x, y) == x + y
    //     }
    // }

    #[test]
    fn prop_test_add() {
        fn test_add(x: u32, y: u32) -> bool {
            println!("x: {} y: {}", x, y);
            helpers::add(x, y) == x + y
        }

        QuickCheck::with_gen(StdThreadGen::new(100000))
            .tests(100000000)
            .quickcheck(test_add as fn(u32, u32) -> bool);

    }
}
