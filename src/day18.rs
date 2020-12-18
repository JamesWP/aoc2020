use std::{
    cmp::max, cmp::min, collections::HashMap, collections::HashSet, convert::TryFrom,
    convert::TryInto, fs, mem, ops::RangeInclusive,
};

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Literal(i64),
    Op(char),
    ExprStart,
    ExprEnd,
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut iter = input.chars().peekable();
    loop {
        let ch = iter.peek();

        if ch.is_none() {
            break;
        }
        let ch = ch.unwrap().clone();

        if ch.is_whitespace() {
            iter.next();
            continue;
        }

        match ch {
            '0'..='9' => {
                let mut num = vec![];
                while iter.peek().is_some() && iter.peek().unwrap().is_numeric() {
                    num.push(iter.next().unwrap());
                }
                let num = num.iter().collect::<String>().parse::<i64>().unwrap();
                tokens.push(Token::Literal(num));
            }
            '+' | '*' => {
                iter.next();
                tokens.push(Token::Op(ch));
            }
            '(' => {
                iter.next();
                tokens.push(Token::ExprStart);
            }
            ')' => {
                iter.next();
                tokens.push(Token::ExprEnd);
            }
            _ => panic!("unknown token: {}", ch),
        }
    }
    tokens
}

#[derive(Debug, PartialEq)]
enum Tree {
    Val(i64),
    Op(char, Box<Tree>, Box<Tree>),
}

struct Parser {
    tokens: Vec<Token>,
    curent: usize,
}

impl Parser {
    fn new(t: Vec<Token>) -> Parser {
        Parser {
            tokens: t,
            curent: 0,
        }
    }

    fn next(&mut self) -> Option<Token> {
        let next = self.peek();
        if next.is_some() {
            self.curent += 1;
        }
        next
    }
    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.curent).cloned()
    }
    fn operation(&mut self) -> char {
        match self.next() {
            Some(Token::Op(c)) => c,
            _ => panic!("bad token"),
        }
    }
    fn term(&mut self) -> Tree {
        let mut factor = self.factor();
        while Some(Token::Op('+')) == self.peek() {
            let op = self.operation();
            let next_factor = self.factor();
            factor = Tree::Op(op, Box::new(factor), Box::new(next_factor));
        }
        factor
    }
    fn expression(&mut self) -> Tree {
        let mut term = self.term();
        while Some(Token::Op('*')) == self.peek() {
            let op = self.operation();
            let next_term = self.term();
            term = Tree::Op(op, Box::new(term), Box::new(next_term));
        }
        term
    }
    fn factor(&mut self) -> Tree {
        match self.next() {
            Some(Token::Literal(v)) => {
                Tree::Val(v)
            }
            Some(Token::ExprStart) => {
                let expr = self.expression();
                if self.tokens[self.curent] != Token::ExprEnd {
                    panic!("bad token");
                }
                self.next();
                expr
            }
            _ => panic!("bad token"),
        }
    }
}

impl Tree {
    fn eval(&self) -> i64 {
        match self {
            Tree::Val(v) => *v,
            Tree::Op('+', a, b) => a.eval() + b.eval(),
            Tree::Op('*', a, b) => a.eval() * b.eval(),
            _ => panic!("unknown operation")
        }
    }
}
#[test]
fn test_lex() {
    let tokens = tokenize("1 + 2 * 3 + 4 * 5 + 6");
    println!("tokens: {:?}", tokens);

    let tokens = tokenize("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
    println!("tokens: {:?}", tokens);

    let parse = Parser::new(tokens).expression();
    println!("parse: {:?}", parse);
    println!("parse: {:?}", parse.eval());
}

#[test]
fn part1() {
    let contents = fs::read_to_string("day18.txt").expect("Something went wrong reading the file");

    let mut sum = 0;

    for line in contents.split("\n") {
        let val = Parser::new(tokenize(line)).expression().eval();
        sum += val;
    }

    println!("sum: {}", sum);
}
