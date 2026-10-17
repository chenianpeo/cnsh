use crate::error::Error::{self};

pub fn lex(source: &str) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::<Token>::new();
    let mut word = String::new();

    for char in source.chars() {
        match char {
            ' ' => {
                if !word.is_empty() {
                    tokens.push(Token::Word(std::mem::take(&mut word)));
                } else {
                    continue;
                }
            }

            '|' => {
                if !word.is_empty() {
                    tokens.push(Token::Word(std::mem::take(&mut word)));
                }
                tokens.push(Token::Pipe)
            }
            '<' => {
                if !word.is_empty() {
                    tokens.push(Token::Word(std::mem::take(&mut word)));
                }
                tokens.push(Token::RedirectIn)
            }
            '>' => {
                if !word.is_empty() {
                    tokens.push(Token::Word(std::mem::take(&mut word)));
                }
                tokens.push(Token::RedirectOut)
            }

            _ => word.push(char),
        }
    }

    if !word.is_empty() {
        tokens.push(Token::Word(word));
    }

    Ok(tokens)
}

#[derive(Debug, Clone)]
pub enum Token {
    Word(String),
    Pipe,
    RedirectIn,
    RedirectOut,
}
