use std::iter::FromIterator;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Let,
    Equal,
    In,
    Def,
    At,
    Arrow,
    Fn,
    If,
    Then,
    Else,
    Number(i64),
    Symbol(String),
}

fn word<Chars>(chars: &mut Chars, word: &str) -> Option<()>
where
    Chars: Iterator<Item = char>,
{
    let cword = word.chars();
    for c in cword {
        chars.next().filter(|x| *x == c)?;
    }
    Some(())
}

fn space<Chars>(chars: &mut Chars) -> Option<()>
where
    Chars: Iterator<Item = char>,
{
    chars.next().filter(|c| c.is_ascii_whitespace())?;
    Some(())
}

pub fn lex(input: &str) -> Option<Vec<Token>> {
    let mut chars = input.chars().peekable();
    let mut tokens: Vec<Token> = vec![];

    while let Some(c) = chars.peek() {
        if c.is_ascii_whitespace() {
            chars.next();
            continue;
        }
        match c {
            '=' => {
                word(&mut chars, "=")?;
                tokens.push(Token::Equal)
            }

            '@' => {
                word(&mut chars, "@")?;
                tokens.push(Token::At)
            }

            '-' => {
                word(&mut chars, "->")?;
                tokens.push(Token::Arrow)
            }

            'd' => {
                word(&mut chars, "def")?;
                space(&mut chars)?;
                tokens.push(Token::Def)
            }

            'e' => {
                word(&mut chars, "else")?;
                space(&mut chars)?;
                tokens.push(Token::Else)
            }

            'f' => {
                word(&mut chars, "fn")?;
                space(&mut chars)?;
                tokens.push(Token::Fn)
            }

            'i' => {
                chars.next()?;
                match chars.next()? {
                    'f' => {
                        space(&mut chars)?;
                        tokens.push(Token::If)
                    }
                    'n' => {
                        space(&mut chars)?;
                        tokens.push(Token::In)
                    }
                    _ => {}
                }
            }

            'l' => {
                word(&mut chars, "let")?;
                space(&mut chars)?;
                tokens.push(Token::Let)
            }

            't' => {
                word(&mut chars, "then")?;
                space(&mut chars)?;
                tokens.push(Token::Then)
            }

            c => {
                let mut tok = vec![];
                while let Some(cc) = chars.next() {
                    if !cc.is_ascii_whitespace() {
                        tok.push(cc);
                    } else {
                        break;
                    }
                }
                let sym = String::from_iter(tok);
                if let Ok(number) = sym.parse::<i64>() {
                    tokens.push(Token::Number(number))
                } else {
                    tokens.push(Token::Symbol(sym))
                }
            }
        }
    }
    Some(tokens)
}
