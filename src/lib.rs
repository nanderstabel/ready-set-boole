use anyhow::{Context, Result};
// use std::collections::HashMap;
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
