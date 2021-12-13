use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fmt;

macro_rules! node {
    ($c:expr, $fact:expr) => {
        node!($c, $fact, None, None)
    };
    ($c:expr, $fact:expr ,$left:expr) => {
        node!($c, $fact, $left, None)
    };
    ($c:expr, $fact:expr, $left:expr, $right:expr) => {
        Node::new($c, $fact, Box::new($left), Box::new($right))
    };
}

pub type Branch = Box<Option<Node>>;

pub struct TruthTable {
    variables: Vec<char>,
    results: Vec<bool>,
}

impl TruthTable {
    pub fn new(variables: Vec<char>) -> Self {
        TruthTable {
            variables,
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

#[derive(Debug)]
pub struct Node {
    _c: char,
    pub fact: bool,
    _left: Branch,
    _right: Branch,
}

impl Node {
    pub fn new(_c: char, fact: bool, _left: Branch, _right: Branch) -> Node {
        Node {
            _c,
            fact,
            _left,
            _right,
        }
    }
}

pub struct Parser {
    pub tree: Option<Node>,
}

impl Parser {
    pub fn new() -> Self {
        Parser { tree: None }
    }

    pub fn evaluate(&mut self, formula: &str) -> Result<()> {
        let mut lexer = formula.chars();
        let mut stack = Vec::new();
        while let Some(c) = lexer.next() {
            match c {
                '0' => stack.push(node!(c, false)),
                '1' => stack.push(node!(c, true)),
                '!' => {
                    let child = stack.pop().context("")?;
                    stack.push(node!(c, !child.fact, Some(child)));
                }
                '&' | '|' | '^' | '>' | '=' => {
                    let (rhs, lhs) = (stack.pop().context("")?, stack.pop().context("")?);
                    let node = node!(
                        c,
                        match c {
                            '&' => lhs.fact & rhs.fact,
                            '=' => lhs.fact == rhs.fact,
                            '>' => !lhs.fact | rhs.fact,
                            '|' => lhs.fact | rhs.fact,
                            '^' => lhs.fact ^ rhs.fact,
                            _ => false,
                        },
                        Some(lhs),
                        Some(rhs)
                    );
                    stack.push(node);
                }
                _ => break,
            }
        }
        self.tree = stack.pop();
        Ok(())
    }

    pub fn truth_table_from(&mut self, formula: &str) -> Result<TruthTable> {
        let mut set = HashSet::new();
        let mut table = TruthTable::new(
            formula
                .chars()
                .filter_map(|c| match c {
                    'A'..='Z' if set.insert(c) => Some(c),
                    _ => None,
                })
                .collect::<Vec<char>>(),
        );
        table.variables.sort();
        for permutation in (0..(1 << table.variables.len())).collect::<Vec<u32>>() {
            let mut alt = String::from(formula);
            for (i, c) in table.variables.iter().enumerate() {
                alt = alt.replace(
                    &c.to_string(),
                    if permutation & (1 << table.variables.len() - 1 - i) == 0 {
                        "0"
                    } else {
                        "1"
                    },
                );
            }
            self.evaluate(&alt)?;
            table.results.push(self.tree.as_ref().unwrap().fact);
        }
        Ok(table)
    }
}
