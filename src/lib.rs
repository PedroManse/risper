#[derive(thiserror::Error, Debug)]
pub enum RispError {
    #[error("No such escape char {0}")]
    NotEscapeChar(char),
    #[error("headless list")]
    Headless,
    #[error("Closing unopened param")]
    CloseNothing,
    #[error("List not closing")]
    UnclosedList,
}

enum Parser {
    OnSymbol,
    OnString { on_special: bool },
}

macro_rules! pushif {
    ($v:ident, $b:ident) => {
        if !$b.is_empty() {
            let f = $b.parse::<f64>();
            let x = if let Some(f) = f.ok() {
                RispToken::Number(f)
            } else {
                match $b {
                    b if b.starts_with("\"") => RispToken::Text(b),
                    b => RispToken::Symbol(b),
                }
            };
            $v.push(x);
        }
    };
}

#[derive(Debug)]
pub enum RispToken {
    Symbol(String),
    Text(String),
    Number(f64),
    LOpen,
    LClose,
}

pub enum RispExp {
    Symbol(String),
    Text(String),
    Number(f64),
    List(Box<RispExp>, Vec<RispExp>),
}

impl std::fmt::Debug for RispExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use RispExp as Re;
        match self {
            Re::List(head, tail) => {
                write!(f, "List:{head:?}")?;
                f.debug_list().entries(tail).finish()
            }
            Re::Symbol(name) => {
                write!(f, "Sym {name}")
            }
            Re::Number(value) => {
                write!(f, "Num {value}")
            }
            Re::Text(value) => {
                write!(f, "Txt {value}")
            }
        }
    }
}

pub fn convert_token(
    token: RispToken,
    rest: &mut impl Iterator<Item = RispToken>,
) -> Result<RispExp, RispError> {
    use RispExp as Re;
    use RispToken as Rt;
    println!("{token:?}");
    Ok(match token {
        Rt::Symbol(x) => Re::Symbol(x),
        Rt::Number(x) => Re::Number(x),
        Rt::Text(x) => Re::Text(x),
        Rt::LOpen => get_list(rest)?,
        Rt::LClose => Err(RispError::CloseNothing)?,
    })
}

pub fn get_list(tokens: &mut impl Iterator<Item = RispToken>) -> Result<RispExp, RispError> {
    let head = convert_token(tokens.next().ok_or(RispError::Headless)?, tokens)?;
    let mut acc: Vec<RispExp> = Vec::new();
    use RispExp as Re;
    use RispToken as Rt;
    loop {
        //while let Some(token) = tokens.next() {
        let token = tokens.next().ok_or(RispError::UnclosedList)?;
        let n = match token {
            Rt::Symbol(x) => Re::Symbol(x),
            Rt::Number(x) => Re::Number(x),
            Rt::Text(x) => Re::Text(x),
            Rt::LOpen => get_list(tokens)?,
            Rt::LClose => break,
        };
        acc.push(n);
    }
    Ok(RispExp::List(Box::new(head), acc))
}

pub fn into_expr(tokens: Vec<RispToken>) -> Result<Vec<RispExp>, RispError> {
    let mut tokens = tokens.into_iter().peekable();
    let mut out = Vec::new();
    while let Some(token) = tokens.next() {
        out.push(convert_token(token, &mut tokens)?);
    }
    Ok(out)
}

pub fn into_tokens(text: &str) -> Result<Vec<RispToken>, RispError> {
    // vec of tokens
    let mut return_buffer = Vec::new();
    // buffer for next token
    let mut buffer = String::new();
    // current parser context
    let mut context = Parser::OnSymbol;
    for chr in text.chars() {
        match context {
            Parser::OnSymbol => match chr {
                '(' => {
                    pushif!(return_buffer, buffer);
                    buffer = String::new();
                    return_buffer.push(RispToken::LOpen);
                }
                ')' => {
                    pushif!(return_buffer, buffer);
                    buffer = String::new();
                    return_buffer.push(RispToken::LClose);
                }
                ' ' | '\n' | '\t' => {
                    pushif!(return_buffer, buffer);
                    buffer = String::new();
                }
                '"' => {
                    pushif!(return_buffer, buffer);
                    buffer = String::new();
                    context = Parser::OnString { on_special: false };
                }
                other => {
                    if !other.is_whitespace() {
                        buffer.push(other);
                    }
                }
            },
            Parser::OnString { on_special: false } => match chr {
                '\"' => {
                    pushif!(return_buffer, buffer);
                    buffer = String::new();
                    context = Parser::OnSymbol;
                }
                '\\' => context = Parser::OnString { on_special: true },
                other => {
                    buffer.push(other);
                }
            },
            Parser::OnString { on_special: true } => {
                let c: char = match chr {
                    '"' => '"',
                    '\\' => '\\',
                    'n' => '\n',
                    other => Err(RispError::NotEscapeChar(other))?,
                };
                buffer.push(c);
                context = Parser::OnString { on_special: false };
            }
        }
    }
    Ok(return_buffer)
}

struct Reader {
    text: String,
}
