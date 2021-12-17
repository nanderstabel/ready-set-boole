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

    pub fn evaluate_nnf(&mut self, formula: &str) -> Result<String> {
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
                        let mut count = 1;
                        while let Some(c) = stack.pop() {
                            // println!("{}:\t{:?}", c, stack);
                            count += match c {
                                '|' | '&' => 1,
                                '!' => 0,
                                _ => -1,
                            };
                            tmp_stack.push(c);
                            if count < 0 {
                                break;
                            }
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

    pub fn rewrite_material_condition(&mut self, stack: &mut Vec<char>) {
        let (rhs, lhs) = (stack.pop().unwrap(), stack.pop().unwrap());
        stack.push(lhs);
        stack.push('!');
        stack.push(rhs);
        stack.push('|');
    }
    pub fn rewrite_equivalence(&mut self, stack: &mut Vec<char>) {
        let (rhs, lhs) = (stack.pop().unwrap(), stack.pop().unwrap());
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
