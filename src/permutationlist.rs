use std::collections::HashSet;

pub struct PermutationList<'a> {
    formula: &'a str,
    pub variables: Vec<char>,
    size: usize,
}

impl PermutationList<'_> {
    pub fn new(formula: &str) -> PermutationList {
        let mut set = HashSet::new();
        let mut variables = formula
            .chars()
            .filter_map(|c| match c {
                'A'..='Z' if set.insert(c) => Some(c),
                _ => None,
            })
            .collect::<Vec<char>>();
        variables.sort();
        PermutationList {
            formula,
            variables: variables,
            size: 0,
        }
    }
}

impl Iterator for PermutationList<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size == 1 << self.variables.len() {
            None
        } else {
            let mut permutation = String::from(self.formula);
            for (i, c) in self.variables.iter().enumerate() {
                permutation = permutation.replace(
                    &c.to_string(),
                    if self.size & (1 << self.variables.len() - 1 - i) == 0 {
                        "0"
                    } else {
                        "1"
                    },
                );
            }
            self.size += 1;
            Some(permutation)
        }
    }
}
