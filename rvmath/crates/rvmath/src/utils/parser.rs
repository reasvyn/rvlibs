//! Expression tokenizer and parser using Shunting Yard algorithm.
//!
//! This module converts infix notation expressions to postfix (RPN) notation,
//! respecting proper operator precedence and associativity.

use std::collections::VecDeque;

/// Represents a token in the expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// A numeric constant (e.g., 3.14, 42)
    Number(f64),
    /// A binary operator (+, -, *, /, %, ^)
    BinaryOp(BinaryOp),
    /// A unary operator (-x for negation)
    UnaryOp(UnaryOp),
    /// A mathematical function (sin, cos, sqrt, etc.)
    Function(String),
    /// Left parenthesis
    LParen,
    /// Right parenthesis
    RParen,
    /// Comma separator for function arguments
    Comma,
}

/// Binary operators with their precedence and associativity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

impl BinaryOp {
    /// Returns the precedence level of the operator (higher = higher precedence).
    pub fn precedence(&self) -> u8 {
        match self {
            BinaryOp::Add | BinaryOp::Sub => 1,
            BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => 2,
            BinaryOp::Pow => 3,
        }
    }

    /// Returns true if the operator is right-associative (e.g., ^ is right-associative).
    pub fn is_right_associative(&self) -> bool {
        matches!(self, BinaryOp::Pow)
    }
}

/// Unary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    /// Unary negation (-x)
    Neg,
}

/// Tokenizes a mathematical expression string.
pub struct Tokenizer {
    input: Vec<char>,
    position: usize,
}

impl Tokenizer {
    /// Creates a new tokenizer for the given input string.
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    /// Returns the next token from the input, or None if at the end.
    pub fn next_token(&mut self) -> Result<Option<Token>, String> {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Ok(None);
        }

        let ch = self.current_char();

        // Check for numbers
        if ch.is_ascii_digit()
            || (ch == '.' && self.peek_char().is_some_and(|c| c.is_ascii_digit()))
        {
            return Ok(Some(self.read_number()?));
        }

        // Check for identifiers (functions or variables)
        if ch.is_ascii_alphabetic() || ch == '_' {
            return Ok(Some(self.read_identifier()));
        }

        // Check for operators and parentheses
        match ch {
            '+' => {
                self.position += 1;
                Ok(Some(Token::BinaryOp(BinaryOp::Add)))
            }
            '-' => {
                self.position += 1;
                Ok(Some(Token::BinaryOp(BinaryOp::Sub)))
            }
            '*' => {
                self.position += 1;
                Ok(Some(Token::BinaryOp(BinaryOp::Mul)))
            }
            '/' => {
                self.position += 1;
                Ok(Some(Token::BinaryOp(BinaryOp::Div)))
            }
            '%' => {
                self.position += 1;
                Ok(Some(Token::BinaryOp(BinaryOp::Mod)))
            }
            '^' => {
                self.position += 1;
                Ok(Some(Token::BinaryOp(BinaryOp::Pow)))
            }
            '(' => {
                self.position += 1;
                Ok(Some(Token::LParen))
            }
            ')' => {
                self.position += 1;
                Ok(Some(Token::RParen))
            }
            ',' => {
                self.position += 1;
                Ok(Some(Token::Comma))
            }
            _ => Err(format!("Unexpected character: '{}'", ch)),
        }
    }

    /// Tokenizes the entire input into a vector of tokens.
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token()? {
            tokens.push(token);
        }
        Ok(tokens)
    }

    fn current_char(&self) -> char {
        self.input[self.position]
    }

    fn peek_char(&self) -> Option<char> {
        if self.position + 1 < self.input.len() {
            Some(self.input[self.position + 1])
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.input[self.position].is_whitespace() {
            self.position += 1;
        }
    }

    fn read_number(&mut self) -> Result<Token, String> {
        let mut number_str = String::new();
        let mut has_dot = false;

        while self.position < self.input.len() {
            let ch = self.current_char();
            if ch.is_ascii_digit() {
                number_str.push(ch);
                self.position += 1;
            } else if ch == '.' && !has_dot {
                has_dot = true;
                number_str.push(ch);
                self.position += 1;
            } else {
                break;
            }
        }

        number_str
            .parse::<f64>()
            .map(Token::Number)
            .map_err(|_| format!("Invalid number: {}", number_str))
    }

    fn read_identifier(&mut self) -> Token {
        let mut ident = String::new();
        while self.position < self.input.len()
            && (self.input[self.position].is_alphanumeric() || self.input[self.position] == '_')
        {
            ident.push(self.current_char());
            self.position += 1;
        }
        Token::Function(ident)
    }
}

/// Parser that converts infix notation to postfix (RPN) using Shunting Yard algorithm.
pub struct Parser {
    input: String,
}

impl Parser {
    /// Creates a new parser for the given expression string.
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
        }
    }

    /// Parses the expression and returns tokens in postfix (RPN) notation.
    pub fn parse(&self) -> Result<Vec<Token>, String> {
        let mut tokenizer = Tokenizer::new(&self.input);
        let infix_tokens = tokenizer.tokenize()?;

        // Convert infix to postfix using Shunting Yard algorithm
        self.shunting_yard(&infix_tokens)
    }

    /// Shunting Yard algorithm to convert infix to postfix notation.
    fn shunting_yard(&self, tokens: &[Token]) -> Result<Vec<Token>, String> {
        let mut output_queue: VecDeque<Token> = VecDeque::new();
        let mut operator_stack: Vec<Token> = Vec::new();
        let mut prev_token: Option<&Token> = None;

        for current_token in tokens {
            match current_token {
                Token::Number(_) => {
                    output_queue.push_back(current_token.clone());
                    prev_token = Some(current_token);
                }
                Token::Function(_) => {
                    operator_stack.push(current_token.clone());
                    prev_token = Some(current_token);
                }
                Token::BinaryOp(op) => {
                    // Check if this should be a unary minus
                    let is_unary = prev_token.is_none()
                        || matches!(
                            prev_token,
                            Some(Token::BinaryOp(_)) | Some(Token::LParen) | Some(Token::Comma)
                        );

                    if is_unary && matches!(op, BinaryOp::Sub) {
                        operator_stack.push(Token::UnaryOp(UnaryOp::Neg));
                    } else {
                        while let Some(top) = operator_stack.last() {
                            let should_pop = match top {
                                Token::BinaryOp(top_op) => {
                                    let precedence_cmp = top_op.precedence() > op.precedence()
                                        || (top_op.precedence() == op.precedence()
                                            && !op.is_right_associative());
                                    precedence_cmp && !matches!(top, Token::Function(_))
                                }
                                Token::UnaryOp(_) => true,
                                _ => false,
                            };

                            if should_pop {
                                output_queue.push_back(operator_stack.pop().unwrap());
                            } else {
                                break;
                            }
                        }
                        operator_stack.push(current_token.clone());
                    }
                    prev_token = Some(current_token);
                }
                Token::LParen => {
                    operator_stack.push(current_token.clone());
                    prev_token = Some(current_token);
                }
                Token::RParen => {
                    let mut found_lparen = false;
                    while let Some(top) = operator_stack.pop() {
                        if matches!(top, Token::LParen) {
                            found_lparen = true;
                            break;
                        }
                        output_queue.push_back(top);
                    }
                    if !found_lparen {
                        return Err("Mismatched parentheses".to_string());
                    }

                    // If the token at the top of the stack is a function, pop it
                    if let Some(Token::Function(_)) = operator_stack.last() {
                        output_queue.push_back(operator_stack.pop().unwrap());
                    }
                    prev_token = Some(current_token);
                }
                Token::Comma => {
                    while let Some(top) = operator_stack.last() {
                        if matches!(top, Token::LParen) {
                            break;
                        }
                        output_queue.push_back(operator_stack.pop().unwrap());
                    }
                    if operator_stack.is_empty() {
                        return Err("Comma outside of function call".to_string());
                    }
                    prev_token = Some(current_token);
                }
                Token::UnaryOp(_) => {
                    operator_stack.push(current_token.clone());
                    prev_token = Some(current_token);
                }
            }
        }

        // Pop remaining operators
        while let Some(top) = operator_stack.pop() {
            if matches!(top, Token::LParen | Token::RParen) {
                return Err("Mismatched parentheses".to_string());
            }
            output_queue.push_back(top);
        }

        Ok(output_queue.into_iter().collect())
    }
}
