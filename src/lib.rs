use anyhow::{Context, Result};

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

#[derive(Debug)]
pub struct Node {
    c: char,
    pub fact: bool,
    left: Branch,
    right: Branch,
}

impl Node {
    pub fn new(c: char, fact: bool, left: Branch, right: Branch) -> Node {
        Node {
            c,
            fact,
            left,
            right,
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

    pub fn parse(&mut self, input: &str) -> Result<()> {
        let mut lexer = input.chars().peekable();
        let mut stack = Vec::new();
        while let Some(c) = lexer.next() {
            match c {
                '0' => stack.push(node!(c, false)),
                '1' => stack.push(node!(c, true)),
                '&' | '|' | '^' | '>' | '=' => {
                    let (rhs, lhs) = (stack.pop().context("")?, stack.pop().context("")?);
                    let node = node!(
                        c,
                        match c {
                            '&' | '>' | '=' => lhs.fact & rhs.fact,
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
}
