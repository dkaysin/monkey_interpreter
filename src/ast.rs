use std::fmt::Display;

#[derive(Debug)]
pub enum Statement {
    Let(Identifier, Expression),
    If(Expression, Box<Statement>, Box<Statement>),
    Return(Expression),
    Expression(Expression),
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Let(i, e) => format!("let {} = {}", i, e),
                Self::If(e, s1, s2) => format!("if {} {{\n{}\n}} else {{\n{}\n}}", e, s1, s2),
                Self::Return(e) => format!("return {}", e),
                Self::Expression(e) => format!("e{{{}}}", e),
            }
        )
    }
}

#[derive(Debug)]
pub enum Expression {
    Variable(String),
    IntLiteral(i32),
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
    Grouped(Box<Expression>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Variable(s) => format!("VAR({s})"),
                Self::IntLiteral(n) => format!("INT({n})"),
                Self::Binary(o, e1, e2) => format!("{e1} {o} {e2}"),
                Self::Grouped(e) => format!("({e})"),
            }
        )
    }
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Add => "+",
                Self::Subtract => "-",
            }
        )
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub name: String,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IDENT({})", self.name)
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let strings = self
            .statements
            .iter()
            .map(|stmt| -> String { stmt.to_string() });
        write!(f, "Program [\n")?;
        for line in strings {
            write!(f, "{}\n", line)?;
        }
        write!(f, "]")
    }
}
