use std::iter::Peekable;
use std::str::Chars;

use crate::error::Error;
use crate::json::Json;
use crate::number::Number;

pub fn number(slice: &mut Peekable<&mut Chars>) -> Result<Json, Error>
{
    let mut token = String::new();

    #[derive(Debug, PartialEq)]
    enum Stages {
        Sign,
        AfterSign,
        AfterZero,
        Integer,
        AfterDot,
        Fraction,
        AfterExp,
        AfterExpSign,
        Exponent,
        End,
    }

    let mut stage       = Stages::Sign;
    let mut is_unsigned = true;

    'tokenizer: loop {
        let current = match slice.peek() {
            Some(chr) => *chr,
            None      => { break 'tokenizer },
        };

        match stage {
            // Waiting for sign or number.
            Stages::Sign => match current {
                '-'       => { stage = Stages::AfterSign; is_unsigned = false; token.push(current); slice.next(); },
                '0'       => { stage = Stages::AfterZero; token.push(current); slice.next(); },
                '1'...'9' => { stage = Stages::Integer;   token.push(current); slice.next(); },

                // Waiting for a number.
                _ => {
                    return Err(Error::InvalidCharacter(current.to_string()));
                },
            },
            Stages::AfterSign => match current {
                '0'       => { stage = Stages::AfterZero; token.push(current); slice.next(); },
                '1'...'9' => { stage = Stages::Integer;   token.push(current); slice.next(); },
                '.'       => { stage = Stages::AfterDot;  token.push(current); slice.next(); },
                'e' | 'E' => { stage = Stages::AfterExp;  token.push(current); slice.next(); },

                // Waiting for a number.
                _ => {
                    return Err(Error::InvalidCharacter(current.to_string()));
                },
            },
            Stages::AfterZero => match current {
                '.'       => { stage = Stages::AfterDot; token.push(current); slice.next(); },
                'e' | 'E' => { stage = Stages::AfterExp; token.push(current); slice.next(); },
                _         => { break 'tokenizer; },
            },
            Stages::Integer => match current {
                '0'...'9' => { stage = Stages::Integer;   token.push(current); slice.next(); },
                '.'       => { stage = Stages::AfterDot;  token.push(current); slice.next(); },
                'e' | 'E' => { stage = Stages::AfterExp;  token.push(current); slice.next(); },
                _         => { break 'tokenizer; },
            },
            Stages::AfterDot => match current {
                '0'...'9' => { stage = Stages::Fraction; token.push(current); slice.next(); },
                'e' | 'E' => { stage = Stages::AfterExp; token.push(current); slice.next(); },

                // Waiting for a number.
                _ => {
                    return Err(Error::InvalidCharacter(current.to_string()));
                },
            },
            Stages::Fraction => match current {
                '0'...'9' => { stage = Stages::Fraction; token.push(current); slice.next(); },
                'e' | 'E' => { stage = Stages::AfterExp; token.push(current); slice.next(); },
                _         => { break 'tokenizer; },
            },
            Stages::AfterExp => match current {
                '+' | '-' => { stage = Stages::AfterExpSign; token.push(current); slice.next(); },
                '0'       => { stage = Stages::End;          token.push(current); slice.next(); },
                '1'...'9' => { stage = Stages::Exponent;     token.push(current); slice.next(); },

                // Waiting for a number.
                _ => {
                    return Err(Error::InvalidCharacter(current.to_string()));
                },
            },
            Stages::AfterExpSign => match current {
                '1'...'9' => { stage = Stages::Exponent; token.push(current); slice.next(); },

                // Waiting for a number.
                _ => {
                    return Err(Error::InvalidCharacter(current.to_string()));
                },
            },
            Stages::Exponent => match current {
                '0'...'9' => { token.push(current); slice.next(); },
                _         => { break 'tokenizer; },
            },
            Stages::End => match current {
                _         => { break 'tokenizer; },
            },
        }
    }

    Ok(Json::Number(
        match stage {
            Stages::Integer | Stages::AfterZero => {
                if is_unsigned {
                    Number::Unsigned(token.parse::<u64>().unwrap())
                } else {
                    Number::Integer(token.parse::<i64>().unwrap())
                }
            },
            _ => {
                Number::Float(token.parse::<f64>().unwrap())
            },
        }
    ))
}

