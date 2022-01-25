use crate::kmap::KMap;
use crate::permutationlist::PermutationList;
use crate::truthtable::TruthTable;
use anyhow::{Context, Result};

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

    pub fn evaluate_nnf(&mut self, formula: &str) -> Result<String> {
        let mut lexer = formula.chars();
        let mut stack = Vec::new();
        while let Some(c) = lexer.next() {
            println!("{}:\t{:?}", c, stack);
            match c {
                'A'..='Z' | '&' | '|' => stack.push(c),
                '!' => {
                    // println!("{}:\t{:?}", c, stack);
                    let c = stack.pop().context("Unexpected end of formula")?;
                    if c == '&' || c == '|' {
                        let mut children = self.get_child(&mut stack);
                        children.append(&mut self.get_child(&mut stack));
                        while let Some(c) = children.pop() {
                            match c {
                                'A'..='Z' => {
                                    stack.push(c);
                                    match children.pop() {
                                        Some('!') => (),
                                        Some(c) => {
                                            stack.push('!');
                                            children.push(c);
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
                        stack.push('!');
                    }
                }
                '>' => self.rewrite_material_condition(&mut stack),
                '=' => self.rewrite_equivalence(&mut stack),
                '^' => {
                    self.rewrite_equivalence(&mut stack);
                    stack.push('!');
                }
                _ => break,
            }
        }
        Ok(String::from_iter(stack))
    }

    fn rewrite_material_condition(&mut self, stack: &mut Vec<char>) {
        let (mut right, mut left) = (self.get_child(stack), self.get_child(stack));
        while let Some(c) = left.pop() {
            match c {
                'A'..='Z' => {
                    stack.push(c);
                    match left.pop() {
                        Some('!') => (),
                        Some(c) => {
                            stack.push('!');
                            left.push(c);
                        }
                        None => stack.push('!'),
                    }
                }
                '&' => stack.push('|'),
                '|' => stack.push('&'),
                _ => (),
            }
        }
        while let Some(c) = right.pop() {
            stack.push(c);
        }
        stack.push('|');
    }

    fn rewrite_equivalence(&mut self, stack: &mut Vec<char>) {
        let (mut right, mut left) = (self.get_child(stack), self.get_child(stack));
        let (mut right_clone, mut left_clone) = (right.clone(), left.clone);
        left.push('>');
        right.push('>');
        left.append(right_clone);
        right.append(left_clone);
    }

    pub fn evaluate_cnf(&mut self, formula: &str) -> Result<String> {
        if let Ok(table) = self.truth_table_from(formula) {
            let mut kmap = KMap::from(table);
            println!("\n\n{}", kmap);
            if let Some(minterms) = kmap.get_minterms() {
                for term in minterms {
                    for b in term.0 {
                        println!("{:04b}: {}", b, b);
                    }
                    println!("");
                }
            }
        }

        Ok(String::from(formula))
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
}
