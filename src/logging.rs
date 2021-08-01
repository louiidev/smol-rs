use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::VecDeque, sync::Mutex};

use crate::render::{BLACK, BLUE, Color, GREEN, RED, WHITE};

pub type LogItemValue<'a> = (&'a str, Color);

#[derive(Default, Debug)]
pub struct Log<'a> {
    pub items: Vec<Vec<LogItemValue<'a>>>,
}

lazy_static! {
    static ref LOG: Mutex<Log<'static>> = Mutex::new(Log::default());
}

pub fn get_log() -> std::sync::MutexGuard<'static, Log<'static>> {
    LOG.lock().unwrap()
}

pub fn log_new_message(text: &'static str) {
    let mut log = get_log();
    log.items.push(parse_tokens(lex(text)));
}

lazy_static! {
    static ref PATTERN_WORD: Regex = Regex::new(r"[a-zA-Z0-9,!?\.'][a-zA-Z0-9,!?\.'\s]+").unwrap();
    static ref PATTERN_COLOR: Regex = Regex::new(r"^(RED|GREEN|BLUE|BLACK)\b").unwrap();
    static ref PATTERN_OPEN_BRACKET: Regex = Regex::new(r"^\[").unwrap();
    static ref PATTERN_CLOSE_BRACKET: Regex = Regex::new(r"^\]").unwrap();
}

#[derive(Debug, PartialEq)]
pub enum LogColors {
    Red,
    Green,
    Blue,
    Black,
    White,
}

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Color(LogColors),
    Text(&'a str),
    OpenParen,
    CloseParen,
}

fn match_with_pattern<'a, F>(
    source: &'a str,
    pattern: &Regex,
    tokenizer: F,
) -> Option<(&'a str, Token<'a>)>
where
    F: Fn(&'a str) -> Token,
{
    if let Some(range) = pattern.find(source) {
        let matched = &source[range.start()..range.end()];
        let rest = &source[range.end()..];

        Some((rest, tokenizer(matched)))
    } else {
        None
    }
}

fn match_color<'a>(source: &'a str) -> Option<(&'a str, Token)> {
    // Our numeric pattern shouldn't ever fail to parse as an f64
    match_with_pattern(source, &PATTERN_COLOR, |v| {
        let colour: LogColors = match v {
            "RED" => LogColors::Red,
            "GREEN" => LogColors::Green,
            "BLUE" => LogColors::Blue,
            _ => LogColors::White,
        };

        Token::Color(colour)
    })
}

fn match_bracket<'a>(source: &'a str) -> Option<(&'a str, Token)> {
    match_with_pattern(source, &PATTERN_OPEN_BRACKET, |_| Token::OpenParen)
        .or_else(|| match_with_pattern(source, &PATTERN_CLOSE_BRACKET, |_| Token::CloseParen))
}

fn match_word<'a>(source: &'a str) -> Option<(&'a str, Token)> {
    match_with_pattern(source, &PATTERN_WORD, |v| Token::Text(v))
}

fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::default();
    let mut current_text = input;

    loop {
        let result = match_color(current_text)
            .or_else(|| match_bracket(current_text))
            .or_else(|| match_word(current_text));

        match result {
            Some((next, token)) => {
                tokens.push(token);
                current_text = next;
            }
            None => break,
        }
    }

    if !current_text.is_empty() {
        eprintln!("Found unexpected tokens in lexer: {}", current_text);
    }

    tokens
}

fn log_color_to_color(log_color: LogColors) -> Color {
    match log_color {
        LogColors::Red => RED,
        LogColors::Blue => BLUE,
        LogColors::Green => GREEN,
        LogColors::Black => BLACK,
        _ => WHITE,
    }
}

fn parse_tokens(tokens: Vec<Token>) -> Vec<LogItemValue> {
    let mut log = Vec::default();
    let mut current_tokens: VecDeque<Token> = tokens.into();

    while let Some(token) = current_tokens.pop_front() {
        match token {
            Token::OpenParen => {
                // expect next token to be color
                let color_token = current_tokens.pop_front().unwrap();
                // expect next token to be text
                let text_token = current_tokens.pop_front().unwrap();
                let color = match color_token {
                    Token::Color(color) => color,
                    _ => panic!("TOKEN IS NOT A COLOR"),
                };
                let text = match text_token {
                    Token::Text(text) => text,
                    _ => panic!("TOKEN IS NOT TEXT"),
                };
                log.push((text, log_color_to_color(color)))
            }
            Token::CloseParen => {}
            Token::Text(text) => {
                log.push((text, WHITE));
            }
            Token::Color(_) => {}
        }
    }

    log
}

#[test]
fn parsing() {
    let text = "[BLUE Hello ]World, My [RED name is louis!]";

    let tokens = lex(text);

    let expect_tokens = vec![
        Token::OpenParen,
        Token::Color(LogColors::Blue),
        Token::Text("Hello "),
        Token::CloseParen,
        Token::Text("World, My "),
        Token::OpenParen,
        Token::Color(LogColors::Red),
        Token::Text("name is louis!"),
        Token::CloseParen,
    ];

    assert_eq!(tokens, expect_tokens);

    let log_message = parse_tokens(tokens);

    let expect_log = vec![
        ("Hello ", BLUE),
        ("World, My ", WHITE),
        ("name is louis!", RED),
    ];

    assert_eq!(log_message, expect_log)
}

#[test]
fn parsing_normal_text() {
    let text = "Hello world my name is louis!";

    let tokens = lex(text);

    let expect_tokens = vec![Token::Text("Hello world my name is louis!")];

    assert_eq!(tokens, expect_tokens);

    let log_message = parse_tokens(tokens);

    let expect_log = vec![("Hello world my name is louis!", WHITE)];

    assert_eq!(log_message, expect_log)
}

#[test]
fn test() {
    let text = "You hit a [GREEN Tree!]. [RED The Tree isn't happy]";

    let tokens = lex(text);

    let expect_tokens = vec![
        Token::Text("You hit a "),
        Token::OpenParen,
        Token::Color(LogColors::Green),
        Token::Text("Tree!"),
        Token::CloseParen,
        Token::Text(". "),
        Token::OpenParen,
        Token::Color(LogColors::Red),
        Token::Text("The Tree isn't happy"),
        Token::CloseParen,
    ];
    assert_eq!(tokens, expect_tokens);
}
