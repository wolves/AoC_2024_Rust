use nom::{
    branch::alt,
    bytes::complete::is_not,
    character::complete::{self, char},
    combinator::{map, opt, recognize},
    multi::separated_list0,
    sequence::{delimited, pair, separated_pair},
    IResult,
};

pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    let (_input, json_val) = parse_val(input).map_err(|e| miette::miette!("Parsing failed {e}"))?;

    Ok(json_val.sum().to_string())
}

#[derive(Debug, PartialEq)]
enum JsonVal {
    Number(i64),
    Array(Vec<JsonVal>),
    Object(Vec<(String, JsonVal)>),
    String(String),
}

impl JsonVal {
    fn sum(&self) -> i64 {
        match self {
            JsonVal::Number(n) => *n,
            JsonVal::Array(arr) => arr.iter().map(|val| val.sum()).sum(),
            JsonVal::Object(obj) => obj.iter().map(|(_, val)| val.sum()).sum(),
            JsonVal::String(_) => 0,
        }
    }
}

fn parse_val(input: &str) -> IResult<&str, JsonVal> {
    alt((parse_num, parse_array, parse_obj, parse_string))(input)
}

fn parse_num(input: &str) -> IResult<&str, JsonVal> {
    map(
        recognize(pair(opt(char('-')), complete::digit1)),
        |num_str: &str| JsonVal::Number(num_str.parse().unwrap()),
    )(input)
}

fn parse_string(input: &str) -> IResult<&str, JsonVal> {
    map(
        delimited(complete::char('"'), is_not("\""), complete::char('"')),
        |s: &str| JsonVal::String(s.to_string()),
    )(input)
}

fn parse_array(input: &str) -> IResult<&str, JsonVal> {
    map(
        delimited(
            complete::char('['),
            separated_list0(complete::char(','), parse_val),
            complete::char(']'),
        ),
        JsonVal::Array,
    )(input)
}

fn parse_obj(input: &str) -> IResult<&str, JsonVal> {
    map(
        delimited(
            complete::char('{'),
            separated_list0(
                complete::char(','),
                separated_pair(parse_string, complete::char(':'), parse_val),
            ),
            complete::char('}'),
        ),
        |pairs| {
            JsonVal::Object(
                pairs
                    .into_iter()
                    .map(|(key, val)| {
                        if let JsonVal::String(k) = key {
                            (k, val)
                        } else {
                            panic!("Object key must be a string")
                        }
                    })
                    .collect(),
            )
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("[1,2,3]", "6")]
    #[case(r#"{"a":2,"b":4}"#, "6")]
    #[case("[[[3]]]", "3")]
    #[case(r#"{"a":{"b":4},"c":-1}"#, "3")]
    #[case(r#"{"a":[-1,1]}"#, "0")]
    #[case(r#"[-1,{"a":1}]"#, "0")]
    #[case("[]", "0")]
    #[case("{}", "0")]
    fn test_process(#[case] input: &str, #[case] result: &str) -> miette::Result<()> {
        assert_eq!(result, process(input)?);
        Ok(())
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn test_num_parser() {
        assert_eq!(parse_num("123"), Ok(("", JsonVal::Number(123))));
        assert_eq!(parse_num("-123"), Ok(("", JsonVal::Number(-123))));
    }

    #[test]
    fn test_string_parser() {
        assert_eq!(
            parse_string(r#""hello""#),
            Ok(("", JsonVal::String("hello".to_string())))
        );
    }

    #[test]
    fn test_array_parser() {
        assert_eq!(
            parse_array("[1,2,3]"),
            Ok((
                "",
                JsonVal::Array(vec![
                    JsonVal::Number(1),
                    JsonVal::Number(2),
                    JsonVal::Number(3),
                ])
            ))
        );
    }

    #[test]
    fn test_obj_parser() {
        assert_eq!(
            parse_obj(r#"{"a":1}"#),
            Ok((
                "",
                JsonVal::Object(vec![("a".to_string(), JsonVal::Number(1))])
            ))
        );
    }
}
