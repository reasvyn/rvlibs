//! Recursive descent parser for algebraic expressions.
/// Parser for algebraic expressions.

/// Converts string expressions into AST using recursive descent parsing
/// with proper operator precedence (PEMDAS).
use super::expr::Expr;

/// Token types for the lexer.
///
/// Represents the different kinds of tokens that can appear in an algebraic expression.
///
/// # Variants
///
/// - `Num(f64)` - A numeric literal (e.g., `3.14`, `42`)
/// - `Var(String)` - A variable name (e.g., `x`, `y`, `alpha`)
/// - `Plus` - Addition operator `+`
/// - `Minus` - Subtraction operator `-` or negation
/// - `Star` - Multiplication operator `*`
/// - `Slash` - Division operator `/`
/// - `Caret` - Exponentiation operator `^`
/// - `LParen` - Left parenthesis `(`
/// - `RParen` - Right parenthesis `)`
/// - `Comma` - Argument separator for functions `,`
/// - `Func(String)` - Function name (e.g., `sin`, `cos`, `sqrt`)
#[derive(Debug, Clone, PartialEq)]
enum Token {
    /// A numeric constant (e.g., `3.14`)
    Num(f64),
    /// A variable identifier (e.g., `x`, `y`, `angle`)
    Var(String),
    /// Addition operator `+`
    Plus,
    /// Subtraction or negation operator `-`
    Minus,
    /// Multiplication operator `*`
    Star,
    /// Division operator `/`
    Slash,
    /// Exponentiation operator `^`
    Caret,
    /// Left parenthesis `(`
    LParen,
    /// Right parenthesis `)`
    RParen,
    /// Function argument separator `,`
    Comma,
    /// Function name (e.g., `sin`, `cos`, `sqrt`, `log`)
    Func(String),
}

/// Lexer for tokenizing algebraic expressions.
///
/// Converts a string input into a stream of tokens for parsing.
/// Handles numbers, variables, operators, parentheses, and function names.
///
/// # Examples
///
/// The lexer is used internally by the parser:
struct Lexer {
    /// Characters of the input string
    chars: Vec<char>,
    /// Current position in the input
    pos: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Lexer {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    fn current(&self) -> Option<char> {
        if self.pos < self.chars.len() {
            Some(self.chars[self.pos])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> f64 {
        let mut num_str = String::new();
        while let Some(c) = self.current() {
            if c.is_ascii_digit() || c == '.' {
                num_str.push(c);
                self.advance();
            } else {
                break;
            }
        }
        num_str.parse().unwrap_or(0.0)
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        while let Some(c) = self.current() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }
        ident
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        match self.current()? {
            '+' => {
                self.advance();
                Some(Token::Plus)
            }
            '-' => {
                self.advance();
                Some(Token::Minus)
            }
            '*' => {
                self.advance();
                Some(Token::Star)
            }
            '/' => {
                self.advance();
                Some(Token::Slash)
            }
            '^' => {
                self.advance();
                Some(Token::Caret)
            }
            '(' => {
                self.advance();
                Some(Token::LParen)
            }
            ')' => {
                self.advance();
                Some(Token::RParen)
            }
            ',' => {
                self.advance();
                Some(Token::Comma)
            }
            c if c.is_ascii_digit() => {
                let num = self.read_number();
                Some(Token::Num(num))
            }
            c if c.is_alphabetic() || c == '_' => {
                let ident = self.read_identifier();
                // Common mathematical functions
                if matches!(
                    ident.as_str(),
                    "sin"
                        | "cos"
                        | "tan"
                        | "sqrt"
                        | "cbrt"
                        | "ln"
                        | "log"
                        | "log10"
                        | "exp"
                        | "abs"
                        | "asin"
                        | "acos"
                        | "atan"
                        | "sinh"
                        | "cosh"
                        | "tanh"
                ) {
                    Some(Token::Func(ident))
                } else {
                    Some(Token::Var(ident))
                }
            }
            _ => None,
        }
    }

    fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        Ok(tokens)
    }
}

/// Parser for algebraic expressions.
///
/// Implements a recursive descent parser that converts tokens into an Abstract Syntax Tree (AST).
/// It respects operator precedence according to PEMDAS/BODMAS rules:
/// 1. **P**arentheses / Parentheses
/// 2. **E**xponents / Exponentiation (`^`)
/// 3. **M**ultiplication (`*`) and **D**ivision (`/`)  
/// 4. **A**ddition (`+`) and **S**ubtraction (`-`)
///
/// # Parsing Strategy
///
/// The parser uses recursive descent with separate methods for each precedence level:
/// - `parse_expr()` - Handles addition and subtraction (lowest precedence)
/// - `parse_term()` - Handles multiplication and division (higher precedence)
/// - `parse_factor()` - Handles exponentiation (higher precedence)
/// - `parse_unary()` - Handles unary operators and negation
/// - `parse_atom()` - Handles atoms (numbers, variables, parenthesized expressions, functions)
///
/// # Examples
///
/// The parser handles complex expressions with proper operator precedence:
pub struct Parser {
    /// Sequence of tokens to parse
    tokens: Vec<Token>,
    /// Current position in the token stream
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if self.current() == Some(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", expected, self.current()))
        }
    }

    /// Parse an expression with lowest precedence (addition/subtraction).
    fn parse_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;

        loop {
            match self.current() {
                Some(Token::Plus) => {
                    self.advance();
                    let right = self.parse_term()?;
                    left = Expr::Add(Box::new(left), Box::new(right));
                }
                Some(Token::Minus) => {
                    self.advance();
                    let right = self.parse_term()?;
                    left = Expr::Sub(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    /// Parse a term (multiplication/division).
    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_factor()?;

        loop {
            match self.current() {
                Some(Token::Star) => {
                    self.advance();
                    let right = self.parse_factor()?;
                    left = Expr::Mul(Box::new(left), Box::new(right));
                }
                Some(Token::Slash) => {
                    self.advance();
                    let right = self.parse_factor()?;
                    left = Expr::Div(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    /// Parse a factor (exponentiation).
    fn parse_factor(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;

        // Right-associative: 2^3^2 = 2^(3^2)
        if matches!(self.current(), Some(Token::Caret)) {
            self.advance();
            let right = self.parse_factor()?; // Recursive for right-associativity
            left = Expr::Pow(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    /// Parse unary operations (negation).
    fn parse_unary(&mut self) -> Result<Expr, String> {
        match self.current() {
            Some(Token::Minus) => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::Neg(Box::new(expr)))
            }
            Some(Token::Plus) => {
                self.advance();
                self.parse_unary()
            }
            _ => self.parse_primary(),
        }
    }

    /// Parse primary expressions (numbers, variables, functions, parentheses).
    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.current().cloned() {
            Some(Token::Num(n)) => {
                self.advance();
                Ok(Expr::Const(n))
            }
            Some(Token::Var(v)) => {
                self.advance();
                Ok(Expr::Var(v))
            }
            Some(Token::Func(fname)) => {
                self.advance();
                self.expect(Token::LParen)?;
                let args = self.parse_args()?;
                self.expect(Token::RParen)?;
                Ok(Expr::Func(fname, args))
            }
            Some(Token::LParen) => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            _ => Err(format!("Unexpected token: {:?}", self.current())),
        }
    }

    /// Parse function arguments.
    fn parse_args(&mut self) -> Result<Vec<Expr>, String> {
        let mut args = Vec::new();

        // Handle empty argument list
        if matches!(self.current(), Some(Token::RParen)) {
            return Ok(args);
        }

        loop {
            args.push(self.parse_expr()?);
            match self.current() {
                Some(Token::Comma) => {
                    self.advance();
                }
                _ => break,
            }
        }

        Ok(args)
    }
}

/// Parse a string into an algebraic expression.
///
pub fn parse(input: &str) -> Result<Expr, String> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;

    if tokens.is_empty() {
        return Err("Empty expression".to_string());
    }

    let mut parser = Parser::new(tokens);
    let expr = parser.parse_expr()?;

    if parser.pos < parser.tokens.len() {
        return Err(format!(
            "Unexpected token after expression: {:?}",
            parser.tokens[parser.pos]
        ));
    }

    Ok(expr)
}


