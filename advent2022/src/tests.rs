macro_rules! test_day {
    ($day:ident, $part1:pat, $part2:pat) => {
        mod $day {
            #[test]
            fn part1() {
                assert!(matches!(crate::$day::part1(), $part1));
            }

            #[test]
            fn part2() {
                assert!(matches!(crate::$day::part2(), $part2));
            }
        }
    };
}

test_day!(day1, Ok(66616), Ok(199172));
test_day!(day2, Ok(14264), Ok(12382));
test_day!(day3, 8185, 2817);
test_day!(day4, 485, 857);

#[cfg(test)]
mod day5 {
    #[test]
    fn part1() {
        assert!(matches!(crate::day5::part1().as_deref(), Ok("TWSGQHNHL")));
    }

    #[test]
    fn part2() {
        assert!(matches!(crate::day5::part2().as_deref(), Ok("JNRSCDWPP")));
    }
}

test_day!(day6, Some(1155), Some(2789));
