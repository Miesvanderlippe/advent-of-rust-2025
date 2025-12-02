#[derive(Debug, PartialEq, Eq)]
pub struct Pattern {
    pub size: usize,
    pub repeats: usize,
}

pub fn find_suitable_patterns(digits: usize) -> Vec<Pattern> {
    let mut patterns = vec![];

    for size in 1..=digits / 2 {
        let repeats = digits / size;
        if digits % size == 0 && repeats > 1 {
            patterns.push(Pattern { size, repeats })
        }
    }

    patterns
}
