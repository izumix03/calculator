use nom::character::complete::{char, one_of};
use nom::{IResult};
use nom::branch::alt;
use nom::error::ErrorKind;
use nom::multi::{many0, many1};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
enum Expr {
    Num(u64),
    // 数値
    Add(Box<Expr>, Box<Expr>),
    // 加算
    Mul(Box<Expr>, Box<Expr>), // 乗算
}

fn parse_num(c: &str) -> IResult<&str, Expr> {
    let (c1, v) = many1(one_of("0123456789"))(c)?;
    let var: String = v.into_iter().collect();

    if let Ok(n) = var.parse::<u64>() {
        Ok((c1, Expr::Num(n)))
    } else {
        let err = nom::error::Error::new(c, ErrorKind::Fail);
        Err(nom::Err::Error(err))
    }
}

fn parse_op(c: &str) -> IResult<&str, Expr> {
    let (c, op) = one_of("+*")(c)?;
    let (c, e1) = parse_expr(c)?; // 1つ目の式をパース
    let (c, e2) = parse_expr(c)?; // 2つ目の式をパース

    if op == '+' {
        Ok((c, Expr::Add(Box::new(e1), Box::new(e2))))
    } else {
        Ok((c, Expr::Mul(Box::new(e1), Box::new(e2))))
    }
}

fn parse_expr(c: &str) -> IResult<&str, Expr> {
    // 0個以上の空白文字を読み飛ばす
    let (c, _) = many0(char(' '))(c)?;

    let result = alt((parse_num, parse_op))(c)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use nom::error::ErrorKind;
    use crate::{Expr, parse_num, parse_op};

    #[test]
    fn test_parse_num() {
        assert_eq!(parse_num("123"), Ok(("", Expr::Num(123))));
        assert_eq!(parse_num("abc"), Err(nom::Err::Error(nom::error::Error::new("abc", ErrorKind::OneOf))));
    }

    #[test]
    fn test_parse_op() {
        assert_eq!(parse_op("+ 1 2"), Ok(("", Expr::Add(Box::new(Expr::Num(1)), Box::new(Expr::Num(2))))));
        assert_eq!(parse_op("* 1 2"), Ok(("", Expr::Mul(Box::new(Expr::Num(1)), Box::new(Expr::Num(2))))));
        assert_eq!(parse_op("* + 1 3 + 2 5"), Ok(("",
                                                  Expr::Mul(
                                                      Box::new(Expr::Add(
                                                          Box::new(Expr::Num(1)),
                                                          Box::new(Expr::Num(3)))
                                                      ),
                                                      Box::new(Expr::Add(
                                                          Box::new(Expr::Num(2)),
                                                          Box::new(Expr::Num(5)))))
        )));
    }
}