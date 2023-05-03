use nom::{
    character::complete::{char, digit1, multispace0},
    combinator::{map, map_res},
    sequence::{delimited, preceded, separated_pair, terminated},
    branch::alt,
    bytes::complete::tag,
    multi::{separated_list0, separated_list1},
    IResult,
};

#[derive(Debug)]
enum Expr {
    Number(i32),
    FunctionCall(Box<Expr>, Box<Expr>),
    Multiply(Vec<Expr>),
    Add(Vec<Expr>),
}

fn number(input: &str) -> IResult<&str, Expr> {
    map_res(digit1, |s: &str| s.parse::<i32>().map(Expr::Number))(input)
}

fn function_call(input: &str) -> IResult<&str, Expr> {
    let inner_expr = preceded(multispace0, expr);
    let args = separated_list1(char(','), inner_expr);

    map(
        separated_pair(
            preceded(multispace0, tag("add")),
            delimited(multispace0, char('('), multispace0),
            terminated(args, preceded(multispace0, char(')'))),
        ),
        |(_, arguments)| Expr::Add(arguments),
    )(input)
}

fn multiply(input: &str) -> IResult<&str, Expr> {
    let inner_expr = preceded(multispace0, expr);
    let expr_list = separated_list1(char(','), inner_expr);

    map(
        preceded(
            multispace0,
            delimited(multispace0, char('('), multispace0),
        ),
        |exprs| Expr::Multiply(exprs),
    )(input)
}

fn expr(input: &str) -> IResult<&str, Expr> {
    alt((number, function_call, multiply))(input)
}

pub fn parse_expr(input: &str) -> Result<Expr, nom::Err<&str>> {
    expr(input).map(|(_, res)| res)
}

