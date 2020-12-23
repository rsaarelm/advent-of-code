use aoc::util;

fn parse(input: &str) -> &str {
    input
}

fn run_1(input: &str) -> u32 {
    0
}

fn run_2(input: &str) -> u32 {
    0
}

fn main() {
    let input = util::slurp_stdin();
    println!("{}", run_1(&input));
    println!("{}", run_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, run_1(""));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, run_2(""));
    }
}
