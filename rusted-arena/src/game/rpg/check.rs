use mockall::*;
use rand::{thread_rng, RngCore};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum CheckResult {
    Success(u32),
    Failure(u32),
}

#[automock]
pub trait Checker {
    fn check(&self, value: i32, difficulty: i32) -> CheckResult;
}

pub struct RandomChecker {
    die_sides: u32,
    rng: Rc<RefCell<Box<dyn RngCore>>>,
}

impl RandomChecker {
    pub fn new(die_sides: u32) -> RandomChecker {
        let rng: Box<dyn RngCore> = Box::new(thread_rng());
        let rng = Rc::new(RefCell::new(rng));
        RandomChecker { die_sides, rng }
    }

    fn roll_die(&self) -> i32 {
        let mut rng = self.rng.borrow_mut();
        let value = rng.next_u32();
        (1 + value % self.die_sides) as i32
    }
}

impl Checker for RandomChecker {
    fn check(&self, value: i32, difficulty: i32) -> CheckResult {
        let positive_die = self.roll_die();
        let negative_die = self.roll_die();
        let random_modifier = positive_die - negative_die;
        let result = value - difficulty + random_modifier;

        println!(
            "value={} difficulty={} random={}-{}={} -> {}",
            value, difficulty, positive_die, negative_die, random_modifier, result
        );

        if result >= 0 {
            CheckResult::Success(result as u32)
        } else {
            CheckResult::Failure(-result as u32)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::rpg::check::CheckResult::*;
    use rand::Error;

    struct MockRng {
        values: Vec<u64>,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.next_u64() as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.values.pop().unwrap()
        }

        #[inline]
        fn fill_bytes(&mut self, _: &mut [u8]) {
            unimplemented!();
        }

        #[inline]
        fn try_fill_bytes(&mut self, _: &mut [u8]) -> Result<(), Error> {
            unimplemented!();
        }
    }

    #[test]
    fn test() {
        assert_check(0, 0, 1, 1, Success(0));
        assert_check(1, 0, 1, 1, Success(1));
        assert_check(0, 2, 1, 1, Failure(2));
        assert_check(0, 0, 4, 1, Success(3));
        assert_check(0, 0, 1, 5, Failure(4));
    }

    fn assert_check(value: i32, difficulty: i32, p: u64, n: u64, result: CheckResult) {
        let rng: Box<dyn RngCore> = Box::new(MockRng {
            values: vec![n - 1, p - 1],
        });
        let rng = Rc::new(RefCell::new(rng));
        let checker = RandomChecker { die_sides: 6, rng };
        assert_eq!(checker.check(value, difficulty), result);
    }
}
