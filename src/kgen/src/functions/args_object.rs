use nom::IResult;
use nom::sequence::tuple;
use nom::combinator::opt;
use nom::combinator::map;
use nom::multi::many0;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::character::complete::alphanumeric1;

pub fn args_parser(text: &str) -> IResult<&str, Vec<&str>> {
    let texts_parser = map(
        tuple((
            alphanumeric1,
            map(
                many0(
                    tuple((
                        space0,
                        tag(","),
                        space0,
                        alphanumeric1
                    ))
                ),
                |val| {
                    val.into_iter()
                        .map(|with_comma| {
                            with_comma.3
                        })
                        .collect::<Vec<&str>>()
                }
            )
        )),
        |val| {
            let (name, names) = val;
            let mut args: Vec<&str> = names.clone();
            args.insert(0, name);
            args
        }
    );

    map(
        opt(texts_parser),
        |val| {
            match val {
                Some(vec) => vec,
                None => Vec::new()
            }
        }
    )(text)
}
