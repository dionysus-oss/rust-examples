use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};

use crate::parser::HashMapAction;
use anyhow::Result;
use nom::error::{convert_error, VerboseError};

fn main() -> Result<()> {
    let mut content = String::new();
    BufReader::new(File::open("./key_value_store.log")?).read_to_string(&mut content)?;

    let actions = match parser::parse_log::<VerboseError<&str>>(content.as_str()) {
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
            panic!("Could not parse - {}", convert_error(content.as_str(), e));
        }
        Ok((_, actions)) => actions,
        k => {
            panic!("unexpected parse error {:?}", k);
        }
    };

    let mut state: HashMap<String, String> = HashMap::new();
    for action in actions {
        match action {
            HashMapAction::InsertKeyValue(k, v) => state.insert(k, v),
            HashMapAction::DeleteKey(k) => state.remove(k.as_str()),
        };
    }

    println!("{:?}", state);

    Ok(())
}

mod parser {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, alphanumeric1, char, line_ending, space1};
    use nom::combinator::map;
    use nom::error::{context, ContextError, ParseError};
    use nom::sequence::Tuple;
    use nom::{IResult, Parser};

    #[derive(Debug)]
    pub enum HashMapAction {
        InsertKeyValue(String, String),
        DeleteKey(String),
    }

    pub fn parse_log<'a, E>(content: &'a str) -> IResult<&'a str, Vec<HashMapAction>, E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        let mut output: Vec<HashMapAction> = vec![];

        let mut current_content = content;
        while !current_content.is_empty() {
            let (content, (v, _)) = context("line", |x| {
                (
                    alt((
                        context("delete-command line", delete_command_parser_map),
                        context("key-value line", key_value_assignment_parser_map),
                    )),
                    line_ending,
                )
                    .parse(x)
            })
            .parse(current_content)?;

            output.push(v);
            current_content = content;
        }

        Ok((content, output))
    }

    fn key_value_assignment_parser_map<'a, E>(
        content: &'a str,
    ) -> IResult<&'a str, HashMapAction, E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        map(
            context("key-value", key_value_assignment_parser),
            |(key, _, value)| HashMapAction::InsertKeyValue(key.to_string(), value.to_string()),
        )
        .parse(content)
    }

    fn key_value_assignment_parser<'a, E>(
        content: &'a str,
    ) -> IResult<&'a str, (&str, char, &str), E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        (
            context("key", alpha1),
            char('='),
            context("value", alphanumeric1),
        )
            .parse(content)
    }

    fn delete_command_parser_map<'a, E>(content: &'a str) -> IResult<&'a str, HashMapAction, E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        map(
            context("delete command", delete_command_parser),
            |(_, _, key)| HashMapAction::DeleteKey(key.to_string()),
        )
        .parse(content)
    }

    fn delete_command_parser<'a, E>(content: &'a str) -> IResult<&'a str, (&str, &str, &str), E>
    where
        E: ParseError<&'a str>,
    {
        (tag(":del"), space1, alpha1).parse(content)
    }
}
