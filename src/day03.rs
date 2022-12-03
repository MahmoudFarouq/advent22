use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
fn day3_part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(f, s)| {
            for x in f.chars() {
                for y in s.chars() {
                    if x == y {
                        return x;
                    }
                }
            }
            unreachable!()
        })
        .map(|c| match c {
            'a'..='z' => c as usize - 'a' as usize + 1,
            'A'..='Z' => c as usize - 'A' as usize + 26 + 1,
            _ => unreachable!(),
        })
        .sum()
}

#[aoc(day3, part2)]
fn day3_part2(input: &str) -> usize {
    input
        .lines()
        .array_chunks::<3>()
        .map(|[x, y, z]| {
            for x in x.chars() {
                for y in y.chars() {
                    for z in z.chars() {
                        if x == y && x == z {
                            return x;
                        }
                    }
                }
            }
            unreachable!()
        })
        .map(|c| match c {
            'a'..='z' => c as usize - 'a' as usize + 1,
            'A'..='Z' => c as usize - 'A' as usize + 26 + 1,
            _ => unreachable!(),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(day3_part1(&input), 157);
    }

    #[test]
    fn test_part2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(day3_part2(&input), 70);
    }
}
