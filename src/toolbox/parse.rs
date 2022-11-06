use std::fmt::Debug;
use std::str::FromStr;

fn parse<I: FromStr>(s: &str) -> Vec<I> where <I as FromStr>::Err: Debug {
    s.lines().map(|l| l.parse::<I>().unwrap()).collect()
}


#[cfg(test)]
mod tests {
    use crate::toolbox::parse::parse;

    #[test]
    fn test_parse() {
        let input = "1\n2\n3";
        let expected = vec![1, 2, 3];

        assert_eq!(parse::<usize>(input), expected)
    }
}