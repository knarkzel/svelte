use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::multispace0,
    combinator::map,
    multi::many0,
    sequence::{delimited, terminated},
    IResult,
};

#[derive(Debug)]
pub struct Element<'a> {
    tag: &'a str,
    body: Box<Body<'a>>,
}

#[derive(Debug)]
enum Body<'a> {
    Text(&'a str),
    Element(Element<'a>),
}

fn parse_body(input: &str) -> IResult<&str, Body> {
    let parse_text = map(take_until("<"), |text: &str| Body::Text(text));
    let parse_element = map(parse_tag, |element| Body::Element(element));
    alt((parse_element, parse_text))(input)
}

pub fn parse_tag(input: &str) -> IResult<&str, Element> {
    let (rest, opening) = delimited(tag("<"), take_until(">"), tag(">"))(input)?;
    let closing_tag = delimited(tag("</"), tag(opening), tag(">"));
    let (rest, body) = terminated(parse_body, closing_tag)(rest)?;
    Ok((
        rest,
        Element {
            tag: opening,
            body: Box::new(body),
        },
    ))
}

pub fn parse(input: &str) -> IResult<&str, Vec<Element>> {
    many0(delimited(multispace0, parse_tag, multispace0))(input)
}
