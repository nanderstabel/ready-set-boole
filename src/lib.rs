use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};

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

    pub fn evaluateNNF(&mut self, formula: &str) -> Result<String> {
        let mut lexer = formula.chars();
        let mut stack = Vec::new();
        while let Some(c) = lexer.next() {
            // println!("{}:\t{:?}", c, stack);
            match c {
                'A'..='Z' | '&' | '|' => stack.push(c),
                '!' => {
                    // println!("{}:\t{:?}", c, stack);
                    let c = stack.pop().context("Unexpected end of formula")?;
                    if c == '&' || c == '|' {
                        let mut tmp_stack = Vec::new();
                        while let Some(c) = stack.pop() {
                            tmp_stack.push(c);
                        }
                        while let Some(c) = tmp_stack.pop() {
                            match c {
                                'A'..='Z' => {
                                    stack.push(c);
                                    match tmp_stack.pop() {
                                        Some('!') => (),
                                        Some(c) => {
                                            stack.push('!');
                                            tmp_stack.push(c);
                                        }
                                        None => stack.push('!'),
                                    }
                                }
                                '&' => stack.push('|'),
                                '|' => stack.push('&'),
                                _ => (),
                            }
                        }
                        stack.push(if c == '&' { '|' } else { '&' });
                    } else if c != '!' {
                        stack.push(c);
                        stack.push(c);
                    }
                }
                '>' => {
                    let (rhs, lhs) = (
                        stack.pop().context("Unexpected end of formula")?,
                        stack.pop().context("Unexpected end of formula")?,
                    );
                    stack.push(lhs);
                    stack.push('!');
                    stack.push(rhs);
                    stack.push('|');
                }
                '=' => {
                    let (rhs, lhs) = (
                        stack.pop().context("Unexpected end of formula")?,
                        stack.pop().context("Unexpected end of formula")?,
                    );
                    stack.push(lhs);
                    stack.push('!');
                    stack.push(rhs);
                    stack.push('|');
                    stack.push(rhs);
                    stack.push('!');
                    stack.push(lhs);
                    stack.push('|');
                    stack.push('&');
                }
                '^' => stack.push(c),
                _ => break,
            }
        }
        Ok(String::from_iter(stack))
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

    pub fn evaluate_cnf(&mut self, formula: &str) -> Result<String> {
        if let Ok(table) = self.truth_table_from(formula) {
            let mut kmap = KMap::from(table);

            println!("\n\n{}", kmap);
            let minterms = kmap.get_minterms();
            println!("{:?}", minterms);
            println!("\n\n{}", kmap);
        }

        Ok(String::from(formula))
    }
}

#[derive(Debug)]
pub struct Group(HashSet<u32>);

impl Group {
    pub fn new() -> Self {
        Group(HashSet::new())
    }

    pub fn insert(&mut self, i: u32) -> bool {
        self.0.insert(i)
    }

    pub fn union<'a>(&'a self, other: &'a Group) -> Group {
        let this = self.0.iter().cloned();
        let that = other.0.iter().cloned();

        Group(this.union(&that))
    }
}

impl PartialEq for Group {
    fn eq(&self, other: &Group) -> bool {
        self.0.is_subset(&other.0) && other.0.is_subset(&self.0)
    }
}

impl Eq for Group {}

impl Hash for Group {
    fn hash<H>(&self, group: &mut H)
    where
        H: Hasher,
    {
        let mut a: Vec<&u32> = self.0.iter().collect();
        a.sort();
        for i in a.iter() {
            i.hash(group);
        }
    }
}

impl FromIterator<u32> for Group {
    fn from_iter<I: IntoIterator<Item = u32>>(iter: I) -> Self {
        let mut g = Group::new();
        for i in iter {
            g.insert(i);
        }
        g
    }
}

pub struct KMap {
    map: Vec<Vec<(u32, bool)>>,
}

impl KMap {
    pub fn from(table: TruthTable) -> Self {
        let (y, x) = match table.variables.len() {
            2 => (2, 2),
            3 => (4, 2),
            4 => (4, 4),
            _ => panic!("Not implemented yet"),
        };
        KMap {
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

    fn get_transpose(&mut self) -> KMap {
        let mut transpose: Vec<Vec<(u32, bool)>> = Vec::new();

        for x in 0..self.map.len() {
            let mut v: Vec<(u32, bool)> = Vec::new();
            for y in 0..self.map[0].len() {
                v.push(self.map[y][x]);
            }
            transpose.push(v);
        }
        KMap { map: transpose }
    }

    fn find_groups(&mut self, j: usize, i: usize) -> Group {
        let mut set = vec![self.map[j][i].0];
        for i2 in (i + 1)..(i + 4) {
            if self.map[j][i2 % 4].1 == false {
                break;
            }
            set.push(self.map[j][i2 % 4].0);
        }
        if set.len() == 3 {
            set.truncate(2);
        }
        if set.len() == 2 {
            for j2 in (j + 1)..(j + 4) {
                if self.map[j2 % 4][i].1 == false || self.map[j2 % 4][(i + 1) % 4].1 == false {
                    break;
                }
                set.push(self.map[j2 % 4][i].0);
                set.push(self.map[j2 % 4][(i + 1) % 4].0);
            }
        }
        if set.len() == 6 {
            set.truncate(4);
        }
        set.into_iter().collect()
    }

    fn get_groups(&mut self) -> HashSet<Group> {
        let mut sets: HashSet<Group> = HashSet::new();
        for j in 0..4 {
            //TODO: make dynamic
            for i in 0..4 {
                //TODO: make dynamic
                if self.map[j][i].1 == true {
                    sets.insert(vec![self.map[j][i].0].into_iter().collect());
                    sets.insert(self.find_groups(j, i));
                    sets.insert(self.get_transpose().find_groups(i, j));
                }
            }
        }
        sets
    }

    pub fn get_minterms(&mut self) -> HashSet<Group> {
        let groups = self.get_groups();

        let mut union = Group::new();
        for group in groups.iter() {
            union = union.union(group);
        }
        println!("{:?}", union);

        groups
    }
}

impl fmt::Display for KMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in &self.map {
            for (bit, b) in y {
                write!(f, " {:2}:{:5} ", bit, b)?;
                // write!(f, " {:04b}:{:5} ", bit, b)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

pub fn adder(a: u32, b: u32) -> u32 {
    let (mut a, mut b) = (a, b);
    while b != 0 {
        let carry = (a & b) << 1;
        a = a ^ b;
        b = carry;
    }
    a
}

pub fn multiplier(a: u32, b: u32) -> u32 {
    let (mut a, mut b, mut res) = (a, b, 0);
    while b > 0 {
        if b & 1 != 0 {
            res = adder(res, a);
        }
        a = a << 1;
        b = b >> 1;
    }
    res
}

pub fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}
