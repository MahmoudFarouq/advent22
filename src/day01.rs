use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Vec<usize> {
    input.lines().fold(vec![0], |mut all, line| match line {
        "" => {
            all.push(0);
            all
        }
        some_number => {
            let t = all.pop().unwrap_or(0) + some_number.parse::<usize>().unwrap();
            all.push(t);
            all
        }
    })
}

/// 72602
#[aoc(day1, part1)]
fn day1_part1(input: &[usize]) -> usize {
    let mut total = input.to_vec();

    total.sort_unstable();

    *total.last().unwrap()
}

/// 207410
#[aoc(day1, part2)]
fn day1_part2(input: &[usize]) -> usize {
    let mut total = input.to_vec();

    total.sort_unstable();

    total.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";
        assert_eq!(day1_part1(&parse_input_day1(input)), 24000);
    }

    #[test]
    fn test_part2() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";
        assert_eq!(day1_part2(&parse_input_day1(input)), 45000);
    }
}
