use anyhow::{Context, Result};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

pub struct PermutationList<'a> {
    formula: &'a str,
    variables: Vec<char>,
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

pub struct TruthTable {
    variables: Vec<char>,
    results: Vec<bool>,
}

impl TruthTable {
    pub fn new() -> Self {
        TruthTable {
            variables: Vec::new(),
            results: Vec::new(),
        }
    }
}

impl fmt::Display for TruthTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let len = self.variables.len();
        for v in &self.variables {
            write!(f, "| {} ", v)?;
        }
        write!(f, "| = |\n")?;
        write!(f, "{}|\n", "|---".repeat(len + 1))?;
        for (i, result) in self.results.iter().enumerate() {
            for b in 0..len {
                write!(
                    f,
                    "| {} ",
                    if i & (1 << (len - 1 - b)) == 0 { 0 } else { 1 }
                )?
            }
            write!(f, "| {} |\n", if *result { 1 } else { 0 })?;
        }
        write!(f, "")
    }
}

pub struct Parser {
    pub result: Option<bool>,
}

impl Parser {
    pub fn new() -> Self {
        Parser { result: None }
    }

    pub fn evaluate(&mut self, formula: &str) -> Result<()> {
        let mut lexer = formula.chars();
        let mut stack = Vec::new();
        while let Some(c) = lexer.next() {
            match c {
                '0' => stack.push(false),
                '1' => stack.push(true),
                '!' => {
                    let child = stack.pop().context("Unexpected end of formula")?;
                    stack.push(!child);
                }
                '&' | '|' | '^' | '>' | '=' => {
                    let (rhs, lhs) = (
                        stack.pop().context("Unexpected end of formula")?,
                        stack.pop().context("Unexpected end of formula")?,
                    );
                    stack.push(match c {
                        '&' => lhs & rhs,
                        '=' => lhs == rhs,
                        '>' => !lhs | rhs,
                        '|' => lhs | rhs,
                        '^' => lhs ^ rhs,
                        _ => false,
                    });
                }
                _ => break,
            }
        }
        self.result = stack.pop();
        Ok(())
    }

    pub fn truth_table_from(&mut self, formula: &str) -> Result<TruthTable> {
        let mut table = TruthTable::new();
        let mut permutationlist = PermutationList::new(formula);
        while let Some(permutation) = permutationlist.next() {
            self.evaluate(&permutation)?;
            table.results.push(self.result.unwrap());
        }
        table.variables.append(&mut permutationlist.variables);
        Ok(table)
    }

    pub fn is_satisfiable(&mut self, formula: &str) -> bool {
        let mut permutationlist = PermutationList::new(formula);
        while let Some(permutation) = permutationlist.next() {
            if self.evaluate(&permutation).is_ok() {
                if self.result.unwrap() {
                    return true;
                }
            }
        }
        false
    }

    pub fn negation_normal_form_from(&mut self, formula: &str) -> Result<String> {
        if let Ok(table) = self.truth_table_from(formula) {
            for v in &table.variables {
                print!("{}", v);
            }
            println!("");
            for n in 0..(table.variables.len() as u32) {
                println!(
                    "{:04b}\t{} --> {}",
                    gray_code(n),
                    gray_code(n),
                    table.results[n as usize]
                );
            }

            let mut bitmap: Bitmap = Bitmap::new(table);

            println!("\n\n{}", bitmap);

            // let hm = HashMap::new();

            // for j in 0..4 {
            //     for i in 0..4 {

            //     }
            // }
        }

        Ok(String::from(formula))
    }
}

pub struct Bitmap {
    map: Vec<Vec<(u32, bool)>>,
}

impl Bitmap {
    pub fn new(table: TruthTable) -> Self {
        let (y, x) = match table.variables.len() {
            2 => (2, 2),
            3 => (4, 2),
            4 => (4, 4),
            _ => panic!("Not implemented yet"),
        };
        Bitmap {
            map: (0..y)
                .map(|j| gray_code(j))
                .map(|j| {
                    (0..x)
                        .map(|i| gray_code(i))
                        .map(|i| {
                            (
                                (i + (j << x / 2)),
                                table.results[(i + (j << x / 2)) as usize],
                            )
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

impl fmt::Display for Bitmap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in &self.map {
            for (bit, b) in y {
                write!(f, " {:04b}:{:5} ", bit, b)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

pub fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}
