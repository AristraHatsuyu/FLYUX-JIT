#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Fn,
    Return,
    Loop,
    Pipe,
    BindOne,
    BindTwo,
    Assign,
    ForceAssign,

    LParen, RParen,
    LBrace, RBrace,
    LBracket, RBracket,
    Comma, Eq,

    Colon,
    Dot,

    Ident(String),
    Number(f64),
    Str(String),

    Comment(String),
    Whitespace,
    Unknown(char),

    If,
    Elif,
    Else,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut line = 1;
    let mut col = 1;

    while let Some(&c) = chars.peek() {
        let token_line = line;
        let token_col = col;
        match c {
            // Symbols
            '(' => { tokens.push(Token { kind: TokenKind::LParen, line: token_line, col: token_col }); chars.next(); col += 1; },
            ')' => { tokens.push(Token { kind: TokenKind::RParen, line: token_line, col: token_col }); chars.next(); col += 1; },
            '{' => { tokens.push(Token { kind: TokenKind::LBrace, line: token_line, col: token_col }); chars.next(); col += 1; },
            '}' => { tokens.push(Token { kind: TokenKind::RBrace, line: token_line, col: token_col }); chars.next(); col += 1; },
            '[' => { tokens.push(Token { kind: TokenKind::LBracket, line: token_line, col: token_col }); chars.next(); col += 1; },
            ']' => { tokens.push(Token { kind: TokenKind::RBracket, line: token_line, col: token_col }); chars.next(); col += 1; },
            ',' => { tokens.push(Token { kind: TokenKind::Comma, line: token_line, col: token_col }); chars.next(); col += 1; },
            '=' => {
                chars.next();
                col += 1;
                match chars.peek() {
                    Some('>') => { chars.next(); tokens.push(Token { kind: TokenKind::BindOne, line: token_line, col: token_col }); col += 1; }
                    Some(':') => {
                        chars.next();
                        col += 1;
                        if chars.peek() == Some(&':') {
                            chars.next();
                            tokens.push(Token { kind: TokenKind::ForceAssign, line: token_line, col: token_col });
                            col += 1;
                        } else {
                            tokens.push(Token { kind: TokenKind::Eq, line: token_line, col: token_col });
                        }
                    }
                    _ => tokens.push(Token { kind: TokenKind::Eq, line: token_line, col: token_col }),
                }
            }
            ':' => {
                chars.next();
                col += 1;
                if chars.peek() == Some(&'=') {
                    chars.next();
                    col += 1;
                    tokens.push(Token { kind: TokenKind::Assign, line: token_line, col: token_col });
                } else {
                    tokens.push(Token { kind: TokenKind::Colon, line: token_line, col: token_col });
                }
            }
            '<' => {
                chars.next();
                col += 1;
                if chars.peek() == Some(&'=') {
                    chars.next();
                    col += 1;
                    if chars.peek() == Some(&'>') {
                        chars.next();
                        col += 1;
                        tokens.push(Token { kind: TokenKind::BindTwo, line: token_line, col: token_col });
                    } else {
                        tokens.push(Token { kind: TokenKind::Unknown('<'), line: token_line, col: token_col }); // treat <= as <
                    }
                } else {
                    tokens.push(Token { kind: TokenKind::Unknown('<'), line: token_line, col: token_col }); // handle lone <
                }
            }
            '.' => {
                chars.next();
                col += 1;
                if chars.peek() == Some(&'>') {
                    chars.next();
                    col += 1;
                    tokens.push(Token { kind: TokenKind::Pipe, line: token_line, col: token_col });
                } else {
                    tokens.push(Token { kind: TokenKind::Dot, line: token_line, col: token_col });
                }
            }
            '+' => { tokens.push(Token { kind: TokenKind::Unknown('+'), line: token_line, col: token_col }); chars.next(); col += 1; },
            '-' => { tokens.push(Token { kind: TokenKind::Unknown('-'), line: token_line, col: token_col }); chars.next(); col += 1; },
            '*' => { tokens.push(Token { kind: TokenKind::Unknown('*'), line: token_line, col: token_col }); chars.next(); col += 1; },
            '>' => { tokens.push(Token { kind: TokenKind::Unknown('>'), line: token_line, col: token_col }); chars.next(); col += 1; },
            '/' => {
                chars.next();
                col += 1;
                if chars.peek() == Some(&'/') {
                    chars.next();
                    col += 1;
                    let mut comment = String::new();
                    while let Some(&nc) = chars.peek() {
                        if nc == '\n' { break; }
                        comment.push(nc);
                        chars.next();
                        col += 1;
                    }
                    tokens.push(Token { kind: TokenKind::Comment(comment), line: token_line, col: token_col });
                } else {
                    tokens.push(Token { kind: TokenKind::Unknown('/'), line: token_line, col: token_col });
                }
            }
            '"' => {
                chars.next();
                col += 1;
                let mut value = String::new();
                while let Some(&nc) = chars.peek() {
                    if nc == '"' {
                        chars.next();
                        col += 1;
                        break;
                    }
                    value.push(nc);
                    chars.next();
                    if nc == '\n' {
                        line += 1;
                        col = 1;
                    } else {
                        col += 1;
                    }
                }
                tokens.push(Token { kind: TokenKind::Str(value), line: token_line, col: token_col });
            }
            c if c.is_whitespace() => {
                chars.next();
                if c == '\n' {
                    line += 1;
                    col = 1;
                } else {
                    col += 1;
                }
                tokens.push(Token { kind: TokenKind::Whitespace, line: token_line, col: token_col });
            }
            c if c.is_ascii_digit() => {
                let mut number = String::new();
                while let Some(&nc) = chars.peek() {
                    if nc.is_ascii_digit() || nc == '.' {
                        number.push(nc);
                        chars.next();
                        col += 1;
                    } else {
                        break;
                    }
                }
                tokens.push(Token { kind: TokenKind::Number(number.parse().unwrap()), line: token_line, col: token_col });
            }
            c if is_ident_start(c) => {
                let mut ident = String::new();
                ident.push(c);
                chars.next();
                col += 1;
                while let Some(&nc) = chars.peek() {
                    if is_ident_continue(nc) {
                        ident.push(nc);
                        chars.next();
                        col += 1;
                    } else {
                        break;
                    }
                }

                match ident.as_str() {
                    "F" => {
                        if chars.peek() == Some(&'>') { chars.next(); col += 1; tokens.push(Token { kind: TokenKind::Fn, line: token_line, col: token_col }); }
                        else { tokens.push(Token { kind: TokenKind::Ident(ident), line: token_line, col: token_col }); }
                    }
                    "R" => {
                        if chars.peek() == Some(&'>') { chars.next(); col += 1; tokens.push(Token { kind: TokenKind::Return, line: token_line, col: token_col }); }
                        else { tokens.push(Token { kind: TokenKind::Ident(ident), line: token_line, col: token_col }); }
                    }
                    "L" => {
                        if chars.peek() == Some(&'>') { chars.next(); col += 1; tokens.push(Token { kind: TokenKind::Loop, line: token_line, col: token_col }); }
                        else { tokens.push(Token { kind: TokenKind::Ident(ident), line: token_line, col: token_col }); }
                    }
                    "if" => tokens.push(Token { kind: TokenKind::If, line: token_line, col: token_col }),
                    "elif" => tokens.push(Token { kind: TokenKind::Elif, line: token_line, col: token_col }),
                    "else" => tokens.push(Token { kind: TokenKind::Else, line: token_line, col: token_col }),
                    _ => tokens.push(Token { kind: TokenKind::Ident(ident), line: token_line, col: token_col }),
                }
            }
            _ => {
                tokens.push(Token { kind: TokenKind::Unknown(c), line: token_line, col: token_col });
                chars.next();
                col += 1;
            }
        }
    }

    tokens.into_iter().filter(|t| t.kind != TokenKind::Whitespace).collect()
}

// ✅ 支持任意 Unicode 起始字符（中文、日文、emoji、国旗…）
fn is_ident_start(c: char) -> bool {
    // 不包括控制字符、符号、空格、标点等
    !(c.is_control() || c.is_whitespace() || is_reserved_symbol(c))
}

// ✅ 支持继续构建标识符
fn is_ident_continue(c: char) -> bool {
    !(c.is_control() || c.is_whitespace() || is_reserved_symbol(c) || matches!(c, '+' | '-' | '*' | '/' | '=' | '>' | '<' | ':' | ','))
}

// ✅ 屏蔽不能作为标识符的结构符号
fn is_reserved_symbol(c: char) -> bool {
    matches!(c,
        '(' | ')' | '{' | '}' | '[' | ']' |
        ',' | '=' | ':' | '<' | '>' | '.' | '/' | '"' | ';'
    )
}