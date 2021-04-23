use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum BenCodeExpr {
    Int(i32),
    List(Vec<BenCodeExpr>),
}

pub type ParseError = String;
pub type ParseResult = Result<BenCodeExpr, ParseError>;

pub fn parse(input: &str) -> Result<BenCodeExpr, ParseError> {
    if input.len() == 0 {
        return Err("Empty string".to_string());
    }

    Parser::new(input).parse()
}

struct Parser<'a> {
    next_char: Option<char>,
    chars: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Parser {
        Parser {
            chars: input.chars().peekable(),
            next_char: None,
        }
    }

    fn parse(&mut self) -> ParseResult {
        self.next_char = self.chars.next();

        match self.next_char {
            Some(c) => match c {
                'i' => self.parse_int(),
                'l' => self.parse_list(),
                _ => Err(format!("Invalid Input found: {}, expecting: 'i or e'", c)),
            },
            None => return Err("Empty string".to_string()),
        }
    }

    fn parse_int(&mut self) -> ParseResult {
        let mut result = String::new();

        loop {
            self.next_char = self.chars.next();
            match self.next_char {
                Some(c) => {
                    match c {
                        '0'..='9' => {
                            result.push(c);
                        }
                        'e' => {
                            // unwraps are not recommend in normal code as they panic but because above we are
                            // making sure the string is only digits we are good to go
                            return Ok(BenCodeExpr::Int(result.as_str().parse::<i32>().unwrap()));
                        }
                        _ => {
                            return Err(format!(
                                "Invalid Input found: {}, expecting: '0..9 or e'",
                                c
                            ))
                        }
                    }
                }
                None => return Err("Unclosed Int".to_string()),
            }
        }
    }

    fn parse_list(&mut self) -> ParseResult {
        let mut result = Vec::new();
        loop {
            match self.chars.peek() {
                Some(c) => match c {
                    'e' => {
                        // Consume the 'e' since we peeked
                        self.next_char = self.chars.next();
                        return Ok(BenCodeExpr::List(result))
                    },
                    _ => result.push(self.parse()?),
                },
                None => return Err("Unclosed List".to_string()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_just_int() {
        assert_eq!(parse("i10e"), Ok(BenCodeExpr::Int(10)));
        assert_eq!(parse("i21e"), Ok(BenCodeExpr::Int(21)));
    }

    #[test]
    fn test_empty_list() {
        assert_eq!(parse("le"), Ok(BenCodeExpr::List(vec![])));
    }

    #[test]
    fn test_list_with_ints() {
        assert_eq!(parse("li10ee"), Ok(BenCodeExpr::List(vec![BenCodeExpr::Int(10)])));
        assert_eq!(parse("li10ei15ee"), Ok(BenCodeExpr::List(vec![BenCodeExpr::Int(10), BenCodeExpr::Int(15)])));
    }

    #[test]
    fn test_list_with_nested_lists() {
        assert_eq!(parse("lleli8eee"), Ok(BenCodeExpr::List(vec![BenCodeExpr::List(vec![]), BenCodeExpr::List(vec![BenCodeExpr::Int(8)])])));
    }
}
