use crate::kmap::KMap;
use crate::permutationlist::PermutationList;
use crate::truthtable::TruthTable;
use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};

pub enum Set {
    Set(HashSet<i32>),
    Negation(HashSet<i32>),
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

    fn get_child(&mut self, stack: &mut Vec<char>) -> Vec<char> {
        let mut child = Vec::new();
        let mut count = 0;
        while let Some(c) = stack.pop() {
            count += match c {
                '|' | '&' => 1,
                '!' => 0,
                _ => -1,
            };
            child.push(c);
            if count < 0 {
                break;
            }
        }
        child
    }

    fn append(&mut self, stack: &mut Vec<char>, append: &mut Vec<char>) {
        while let Some(c) = append.pop() {
            match c {
                'A'..='Z' => {
                    stack.push(c);
                    match append.pop() {
                        Some('!') => (),
                        Some(c) => {
                            stack.push('!');
                            append.push(c);
                        }
                        None => stack.push('!'),
                    }
                }
                '&' => stack.push('|'),
                '|' => stack.push('&'),
                _ => (),
            }
        }
    }

    fn apply_de_morgan(&mut self, stack: &mut Vec<char>) {
        let c = stack.pop().unwrap();
        if c == '&' || c == '|' {
            let mut children = self.get_child(stack);
            children.append(&mut self.get_child(stack));
            self.append(stack, &mut children);
            stack.push(if c == '&' { '|' } else { '&' });
        } else if c != '!' {
            stack.push(c);
            stack.push('!');
        }
    }

    fn rewrite_exclusive_or(&mut self, stack: &mut Vec<char>) {
        let (mut right, mut left) = (self.get_child(stack), self.get_child(stack));
        right.reverse();
        left.reverse();
        let (mut right_clone, mut left_clone) = (right.clone(), left.clone());
        stack.append(&mut left);
        stack.append(&mut right);
        stack.push('|');
        stack.append(&mut left_clone);
        stack.append(&mut right_clone);
        stack.push('&');
        self.apply_de_morgan(stack);
        stack.push('&');
    }

    fn rewrite_material_condition(&mut self, stack: &mut Vec<char>) {
        let (mut right, mut left) = (self.get_child(stack), self.get_child(stack));
        self.append(stack, &mut left);
        right.reverse();
        stack.append(&mut right);
        stack.push('|');
    }

    fn rewrite_equivalence(&mut self, stack: &mut Vec<char>) {
        let (mut right, mut left) = (self.get_child(stack), self.get_child(stack));
        right.reverse();
        left.reverse();
        let (mut right_clone, mut left_clone) = (right.clone(), left.clone());
        left.append(&mut right_clone);
        right.append(&mut left_clone);
        self.rewrite_material_condition(&mut left);
        self.rewrite_material_condition(&mut right);
        stack.append(&mut left);
        stack.append(&mut right);
        stack.push('&');
    }

    pub fn evaluate_nnf(&mut self, formula: &str) -> Result<String> {
        let mut lexer = formula.chars();
        let mut stack = Vec::new();
        while let Some(c) = lexer.next() {
            match c {
                'A'..='Z' | '&' | '|' => stack.push(c),
                '!' => self.apply_de_morgan(&mut stack),
                '^' => self.rewrite_exclusive_or(&mut stack),
                '>' => self.rewrite_material_condition(&mut stack),
                '=' => self.rewrite_equivalence(&mut stack),
                _ => break,
            }
        }
        Ok(String::from_iter(stack))
    }

    pub fn evaluate_cnf(&mut self, formula: &str) -> Result<String> {
        let mut form = String::new();
        if let Ok(table) = self.truth_table_from(formula) {
            let mut kmap = KMap::from(table);

            if let Some(minterms) = kmap.get_minterms() {
                for term in &minterms {
                    let mut tmp = vec![0, 0, 0, 0];
                    let mut count = 0;
                    for b in &term.0 {
                        for bit in 0..(kmap.variables.len()) {
                            if b & (1 << ((kmap.variables.len() - 1) - bit)) != 0 {
                                tmp[bit] += 1;
                            }
                        }
                    }
                    for (i, v) in kmap.variables.iter().enumerate() {
                        if tmp[i] == 0 {
                            form.push(*v);
                            count += 1;
                        } else if tmp[i] == term.len() {
                            form.push(*v);
                            form.push('!');
                            count += 1;
                        }
                    }
                    if count > 1 {
                        form.push_str(&"|".repeat(count - 1));
                    }
                }
                form.push_str(&"&".repeat(minterms.len() - 1));
            }
        }

        Ok(String::from(form))
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

    pub fn evaluate_set(&mut self, formula: &str, sets: &[&[i32]]) -> Result<Vec<i32>> {
        let formula = self.evaluate_nnf(formula.clone())?;
        if let Ok(table) = self.truth_table_from(&formula) {
            let mut lexer = formula.chars();
            let mut u: Vec<i32> = Vec::new();
            for set in sets.into_iter() {
                for i in set.into_iter() {
                    u.push(*i);
                }
            }
            let u: HashSet<i32> = u.iter().cloned().collect::<HashSet<i32>>();
            let map: HashMap<char, &&[i32]> =
                table.variables.into_iter().zip(sets.iter()).collect();
            let mut stack: Vec<Set> = Vec::new();
            while let Some(c) = lexer.next() {
                match c {
                    'A'..='Z' => {
                        let tmp = map.get(&c).unwrap();
                        stack.push(Set::Set(HashSet::from_iter(
                            tmp.iter().cloned().collect::<HashSet<i32>>(),
                        )));
                    }
                    '!' => {
                        if let Set::Set(child) = stack.pop().unwrap() {
                            stack.push(Set::Negation(child));
                        }
                    }
                    '&' => {
                        let (rhs, lhs) = (
                            stack.pop().context("Unexpected end of formula")?,
                            stack.pop().context("Unexpected end of formula")?,
                        );
                        match (lhs, rhs) {
                            (Set::Set(l), Set::Set(r)) => {
                                stack.push(Set::Set(l.intersection(&r).cloned().collect()))
                            }

                            (Set::Set(l), Set::Negation(r)) => {
                                stack.push(Set::Set(l.difference(&r).cloned().collect()))
                            }

                            (Set::Negation(l), Set::Set(r)) => {
                                stack.push(Set::Set(r.difference(&l).cloned().collect()))
                            }

                            (Set::Negation(l), Set::Negation(r)) => {
                                stack.push(Set::Set(
                                    (u.difference(&l).cloned().collect::<HashSet<i32>>())
                                        .difference(&r)
                                        .cloned()
                                        .collect(),
                                ));
                            }
                        }
                    }
                    '|' => {
                        let (rhs, lhs) = (
                            stack.pop().context("Unexpected end of formula")?,
                            stack.pop().context("Unexpected end of formula")?,
                        );
                        match (lhs, rhs) {
                            (Set::Set(l), Set::Set(r)) => {
                                stack.push(Set::Set(l.union(&r).cloned().collect()))
                            }
                            (Set::Set(l), Set::Negation(r)) => {
                                stack.push(Set::Set(
                                    (u.difference(&r).cloned().collect::<HashSet<i32>>())
                                        .union(&l)
                                        .cloned()
                                        .collect(),
                                ));
                            }
                            (Set::Negation(l), Set::Set(r)) => {
                                stack.push(Set::Set(
                                    (u.difference(&l).cloned().collect::<HashSet<i32>>())
                                        .union(&r)
                                        .cloned()
                                        .collect(),
                                ));
                            }
                            (Set::Negation(l), Set::Negation(r)) => stack.push(Set::Set(
                                u.difference(
                                    &(l.intersection(&r).cloned().collect::<HashSet<i32>>()),
                                )
                                .cloned()
                                .collect(),
                            )),
                        }
                    }
                    _ => {}
                }
            }
            if let Some(res) = stack.pop() {
                match res {
                    Set::Set(hs) => {
                        let mut v = Vec::from_iter(hs);
                        v.sort();
                        return Ok(v);
                    }
                    Set::Negation(_) => return Ok(Vec::new()),
                }
            }
        }

        Ok(Vec::from([]))
    }
}
