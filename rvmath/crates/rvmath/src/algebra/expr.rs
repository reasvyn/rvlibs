//! Expression AST for symbolic algebra.
/// Expression AST (Abstract Syntax Tree) for algebraic expressions.
/// Represents symbolic algebraic expressions as a tree structure, supporting:
/// - Numbers and variables
/// - Binary operations: +, -, *, /, ^
/// - Unary operations: negation
/// - Functions: sin, cos, tan, sqrt, ln, etc.
/// This enum is the core data structure for symbolic algebra operations.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Numeric constant: 3, 3.14, -5
    Const(f64),
    /// Variable: x, y, z
    Var(String),
    /// Addition: a + b
    Add(Box<Expr>, Box<Expr>),
    /// Subtraction: a - b
    Sub(Box<Expr>, Box<Expr>),
    /// Multiplication: a * b
    Mul(Box<Expr>, Box<Expr>),
    /// Division: a / b
    Div(Box<Expr>, Box<Expr>),
    /// Power: a ^ b
    Pow(Box<Expr>, Box<Expr>),
    /// Negation: -a
    Neg(Box<Expr>),
    /// Function call: sin(x), cos(x), sqrt(x), etc.
    Func(String, Vec<Expr>),
}

impl Expr {
    /// Check if expression is a constant (numeric value).
    /// Returns true if and only if the expression is a numeric constant.
    pub fn is_const(&self) -> bool {
        matches!(self, Expr::Const(_))
    }

    /// Get the numeric value if this expression is a constant.
    /// Returns `Some(value)` if the expression is a constant, `None` otherwise.
    pub fn as_const(&self) -> Option<f64> {
        if let Expr::Const(n) = self {
            Some(*n)
        } else {
            None
        }
    }

    /// Check if expression is a single variable with a specific name.
    /// Returns true only if the expression is exactly the variable with the given name.
    pub fn is_var(&self, name: &str) -> bool {
        matches!(self, Expr::Var(v) if v == name)
    }

    /// Check if expression contains a variable with a specific name anywhere in the tree.
    /// Recursively searches all sub-expressions for the given variable.
    pub fn contains_var(&self, name: &str) -> bool {
        match self {
            Expr::Const(_) => false,
            Expr::Var(v) => v == name,
            Expr::Add(a, b)
            | Expr::Sub(a, b)
            | Expr::Mul(a, b)
            | Expr::Div(a, b)
            | Expr::Pow(a, b) => a.contains_var(name) || b.contains_var(name),
            Expr::Neg(e) => e.contains_var(name),
            Expr::Func(_, args) => args.iter().any(|arg| arg.contains_var(name)),
        }
    }

    /// Get all variables used in the expression as a sorted, deduplicated vector.
    pub fn variables(&self) -> Vec<String> {
        let mut vars = Vec::new();
        self.collect_vars(&mut vars);
        vars.sort();
        vars.dedup();
        vars
    }

    fn collect_vars(&self, vars: &mut Vec<String>) {
        match self {
            Expr::Const(_) => {}
            Expr::Var(v) => vars.push(v.clone()),
            Expr::Add(a, b)
            | Expr::Sub(a, b)
            | Expr::Mul(a, b)
            | Expr::Div(a, b)
            | Expr::Pow(a, b) => {
                a.collect_vars(vars);
                b.collect_vars(vars);
            }
            Expr::Neg(e) => e.collect_vars(vars),
            Expr::Func(_, args) => {
                for arg in args {
                    arg.collect_vars(vars);
                }
            }
        }
    }

    /// Create a constant expression.
    pub fn const_val(n: f64) -> Self {
        Expr::Const(n)
    }

    /// Create a variable expression.
    pub fn var(name: impl Into<String>) -> Self {
        Expr::Var(name.into())
    }

    /// Add two expressions.
    #[allow(clippy::should_implement_trait)]
    pub fn add(self, other: Expr) -> Self {
        Expr::Add(Box::new(self), Box::new(other))
    }

    /// Subtract two expressions.
    #[allow(clippy::should_implement_trait)]
    pub fn sub(self, other: Expr) -> Self {
        Expr::Sub(Box::new(self), Box::new(other))
    }

    /// Multiply two expressions.
    #[allow(clippy::should_implement_trait)]
    pub fn mul(self, other: Expr) -> Self {
        Expr::Mul(Box::new(self), Box::new(other))
    }

    /// Divide two expressions.
    #[allow(clippy::should_implement_trait)]
    pub fn div(self, other: Expr) -> Self {
        Expr::Div(Box::new(self), Box::new(other))
    }

    /// Raise to a power.
    pub fn pow(self, exp: Expr) -> Self {
        Expr::Pow(Box::new(self), Box::new(exp))
    }

    /// Negate the expression.
    #[allow(clippy::should_implement_trait)]
    pub fn neg(self) -> Self {
        Expr::Neg(Box::new(self))
    }

    /// Apply a function to this expression.
    pub fn func(self, fname: impl Into<String>) -> Self {
        Expr::Func(fname.into(), vec![self])
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Const(n) => {
                // Format numbers nicely
                if n.fract() == 0.0 && !n.is_infinite() {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            Expr::Var(v) => write!(f, "{}", v),
            Expr::Add(a, b) => {
                write!(f, "({}+{})", a, b)
            }
            Expr::Sub(a, b) => {
                write!(f, "({}-{})", a, b)
            }
            Expr::Mul(a, b) => {
                write!(f, "{}*{}", a, b)
            }
            Expr::Div(a, b) => {
                write!(f, "{}/{}", a, b)
            }
            Expr::Pow(a, b) => {
                write!(f, "({}^{})", a, b)
            }
            Expr::Neg(e) => {
                write!(f, "-({})", e)
            }
            Expr::Func(name, args) => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
        }
    }
}


