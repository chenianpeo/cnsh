use crate::{
    error::Error,
    lexer::Token::{self},
};

#[derive(Debug)]
pub struct Command {
    procedure: String,
    arguments: Vec<String>,
}

#[derive(Debug)]
pub struct Pipeline {
    commands: Vec<Command>,
}

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn parse_command(&self) -> Result<Pipeline, Error> {
        let mut commands = Vec::<Command>::new();
        let mut procedure = String::new();
        let mut arguments = Vec::<String>::new();

        for token in self.tokens.clone() {
            match token {
                Token::Word(word) => {
                    if procedure.is_empty() {
                        procedure = word;
                    } else {
                        arguments.push(word);
                    }
                }
                Token::Pipe => {
                    commands.push(Command {
                        procedure: procedure.clone(),
                        arguments: arguments.clone(),
                    });
                    procedure.clear();
                    arguments.clear();
                }

                _ => return Err(Error::Unfinished),
            }
        }
        if !procedure.is_empty() {
            commands.push(Command {
                procedure,
                arguments,
            });
        }

        Ok(Pipeline { commands })
    }
}
