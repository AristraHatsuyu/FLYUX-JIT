use crate::lexer::{Token, TokenKind};
use crate::ast::{Expr, Stmt, Function};

pub fn parse(tokens: &[Token]) -> Vec<Function> {
    let mut index = 0;
    let mut fns = Vec::new();

    while index < tokens.len() {
        if matches!(tokens.get(index), Some(Token { kind: TokenKind::Fn, .. })) {
            index += 1;

            let name = match tokens.get(index) {
                Some(Token { kind: TokenKind::Ident(id), .. }) => id.clone(),
                _ => panic!(
                    "Parse error at line {}, col {}: Expected function name",
                    tokens[index].line,
                    tokens[index].col
                ),
            };
            index += 1;

            let mut params = Vec::new();
            if matches!(tokens.get(index), Some(Token { kind: TokenKind::LParen, .. })) {
                index += 1;
                while !matches!(tokens.get(index), Some(Token { kind: TokenKind::RParen, .. })) {
                    let param_name = match tokens.get(index) {
                        Some(Token { kind: TokenKind::Ident(p), .. }) => p.clone(),
                        _ => panic!(
                            "Parse error at line {}, col {}: Expected parameter name",
                            tokens[index].line,
                            tokens[index].col
                        ),
                    };
                    index += 1;

                    let mut param_type = None;
                    if matches!(tokens.get(index), Some(Token { kind: TokenKind::LParen, .. })) {
                        index += 1;
                        param_type = match tokens.get(index) {
                            Some(Token { kind: TokenKind::Ident(t), .. }) => Some(t.clone()),
                            _ => panic!(
                                "Parse error at line {}, col {}: Expected type after (",
                                tokens[index].line,
                                tokens[index].col
                            ),
                        };
                        index += 1;
                        if !matches!(tokens.get(index), Some(Token { kind: TokenKind::RParen, .. })) {
                            panic!(
                                "Parse error at line {}, col {}: Expected ) after type",
                                tokens[index].line,
                                tokens[index].col
                            );
                        }
                        index += 1;
                    }

                    params.push((param_name, param_type));
                    if matches!(tokens.get(index), Some(Token { kind: TokenKind::Comma, .. })) {
                        index += 1;
                    }
                }
                index += 1; // skip RParen
            }

            while !matches!(tokens.get(index), Some(Token { kind: TokenKind::LBrace, .. })) {
                index += 1;
            }
            index += 1;

            let mut body = Vec::new();
            while !matches!(tokens.get(index), Some(Token { kind: TokenKind::RBrace, .. })) {
                body.push(parse_stmt(tokens, &mut index));
            }
            index += 1;

            fns.push(Function { name, params, body });
        } else {
            index += 1;
        }
    }

    fns
}

fn parse_stmt(tokens: &[Token], index: &mut usize) -> Stmt {
    // 跳过注释和空白 token
    loop {
        match tokens.get(*index) {
            Some(Token { kind: TokenKind::Comment(_), .. }) |
            Some(Token { kind: TokenKind::Whitespace, .. }) => {
                *index += 1;
            }
            _ => break,
        }
    }
    // Prefix increment/decrement: ++a or --a
    if matches!(tokens.get(*index), Some(Token { kind: TokenKind::Unknown('+'), .. }))
        && matches!(tokens.get(*index + 1), Some(Token { kind: TokenKind::Unknown('+'), .. }))
        && matches!(tokens.get(*index + 2), Some(Token { kind: TokenKind::Ident(_), .. })) {
        // consume '++'
        *index += 2;
        // capture variable name
        if let Token { kind: TokenKind::Ident(name), .. } = tokens.get(*index).unwrap() {
            let var = name.clone();
            *index += 1;
            return Stmt::Increment(var);
        }
    }
    if matches!(tokens.get(*index), Some(Token { kind: TokenKind::Unknown('-'), .. }))
        && matches!(tokens.get(*index + 1), Some(Token { kind: TokenKind::Unknown('-'), .. }))
        && matches!(tokens.get(*index + 2), Some(Token { kind: TokenKind::Ident(_), .. })) {
        *index += 2;
        if let Token { kind: TokenKind::Ident(name), .. } = tokens.get(*index).unwrap() {
            let var = name.clone();
            *index += 1;
            return Stmt::Decrement(var);
        }
    }
    if let Some(Token { kind: TokenKind::Return, .. }) = tokens.get(*index) {
        *index += 1;
        let expr = parse_binary_expr(tokens, index);
        // println!("Return 表达式结构：{:?}", expr);
        return Stmt::Return(expr);
    }

    if let Some(Token { kind: TokenKind::Loop, .. }) = tokens.get(*index) {
        return parse_loop_stmt(tokens, index);
    }

    if let Some(Token { kind: TokenKind::If, .. }) = tokens.get(*index) {
        *index += 1;

        let mut branches = Vec::new();

        // 处理 if 主分支
        if matches!(tokens.get(*index), Some(Token { kind: TokenKind::LParen, .. })) {
            *index += 1;
            let cond = parse_binary_expr(tokens, index);
            if !matches!(tokens.get(*index), Some(Token { kind: TokenKind::RParen, .. })) {
                panic!(
                    "Parse error at line {}, col {}: Expected ')' after if condition",
                    tokens[*index].line,
                    tokens[*index].col
                );
            }
            *index += 1;

            if !matches!(tokens.get(*index), Some(Token { kind: TokenKind::LBrace, .. })) {
                panic!(
                    "Parse error at line {}, col {}: Expected '{{' after if condition",
                    tokens[*index].line,
                    tokens[*index].col
                );
            }
            *index += 1;

            let mut body = Vec::new();
            while !matches!(tokens.get(*index), Some(Token { kind: TokenKind::RBrace, .. })) {
                body.push(parse_stmt(tokens, index));
            }
            *index += 1;

            branches.push((Some(cond), body));
        } else {
            panic!(
                "Parse error at line {}, col {}: Expected '(' after if",
                tokens[*index].line,
                tokens[*index].col
            );
        }

        // 处理 elif 和 else 分支
        loop {
            if let Some(Token { kind: TokenKind::Elif, .. }) = tokens.get(*index) {
                *index += 1;
                if !matches!(tokens.get(*index), Some(Token { kind: TokenKind::LParen, .. })) {
                    panic!(
                        "Parse error at line {}, col {}: Expected '(' after elif",
                        tokens[*index].line,
                        tokens[*index].col
                    );
                }
                *index += 1;
                let cond = parse_binary_expr(tokens, index);
                if !matches!(tokens.get(*index), Some(Token { kind: TokenKind::RParen, .. })) {
                    panic!(
                        "Parse error at line {}, col {}: Expected ')' after elif condition",
                        tokens[*index].line,
                        tokens[*index].col
                    );
                }
                *index += 1;

                if !matches!(tokens.get(*index), Some(Token { kind: TokenKind::LBrace, .. })) {
                    panic!(
                        "Parse error at line {}, col {}: Expected '{{' after elif condition",
                        tokens[*index].line,
                        tokens[*index].col
                    );
                }
                *index += 1;

                let mut body = Vec::new();
                while !matches!(tokens.get(*index), Some(Token { kind: TokenKind::RBrace, .. })) {
                    body.push(parse_stmt(tokens, index));
                }
                *index += 1;

                branches.push((Some(cond), body));
            } else if let Some(Token { kind: TokenKind::Else, .. }) = tokens.get(*index) {
                *index += 1;
                if !matches!(tokens.get(*index), Some(Token { kind: TokenKind::LBrace, .. })) {
                    panic!(
                        "Parse error at line {}, col {}: Expected '{{' after else",
                        tokens[*index].line,
                        tokens[*index].col
                    );
                }
                *index += 1;

                let mut body = Vec::new();
                while !matches!(tokens.get(*index), Some(Token { kind: TokenKind::RBrace, .. })) {
                    body.push(parse_stmt(tokens, index));
                }
                *index += 1;

                branches.push((None, body));
                break;
            } else if matches!(tokens.get(*index), Some(Token { kind: TokenKind::LParen, .. })) {
                // 原始简洁写法
                *index += 1;
                let cond = parse_binary_expr(tokens, index);
                if !matches!(tokens.get(*index), Some(Token { kind: TokenKind::RParen, .. })) {
                    panic!(
                        "Parse error at line {}, col {}: Expected ')' after condition",
                        tokens[*index].line,
                        tokens[*index].col
                    );
                }
                *index += 1;

                if !matches!(tokens.get(*index), Some(Token { kind: TokenKind::LBrace, .. })) {
                    panic!(
                        "Parse error at line {}, col {}: Expected '{{' after condition",
                        tokens[*index].line,
                        tokens[*index].col
                    );
                }
                *index += 1;

                let mut body = Vec::new();
                while !matches!(tokens.get(*index), Some(Token { kind: TokenKind::RBrace, .. })) {
                    body.push(parse_stmt(tokens, index));
                }
                *index += 1;

                branches.push((Some(cond), body));
            } else if matches!(tokens.get(*index), Some(Token { kind: TokenKind::LBrace, .. })) {
                *index += 1;
                let mut body = Vec::new();
                while !matches!(tokens.get(*index), Some(Token { kind: TokenKind::RBrace, .. })) {
                    body.push(parse_stmt(tokens, index));
                }
                *index += 1;
                branches.push((None, body));
                break;
            } else {
                break;
            }
        }

        return Stmt::MultiIf(branches);
    }

    // ✅ 优先识别函数调用语句
    if let Some(Token { kind: TokenKind::Ident(id), .. }) = tokens.get(*index) {
        if matches!(tokens.get(*index + 1), Some(Token { kind: TokenKind::LParen, .. })) {
            let id = id.clone();
            *index += 1;
            let args = parse_call_args(tokens, index);
            return Stmt::Expr(Expr::Call(id, args));
        }
    }

    // Postfix increment/decrement: a++ or a--
    if let Some(Token { kind: TokenKind::Ident(name), .. }) = tokens.get(*index) {
        if matches!(tokens.get(*index + 1), Some(Token { kind: TokenKind::Unknown('+'), .. }))
            && matches!(tokens.get(*index + 2), Some(Token { kind: TokenKind::Unknown('+'), .. })) {
            let var = name.clone();
            *index += 3;
            return Stmt::Increment(var);
        }
        if matches!(tokens.get(*index + 1), Some(Token { kind: TokenKind::Unknown('-'), .. }))
            && matches!(tokens.get(*index + 2), Some(Token { kind: TokenKind::Unknown('-'), .. })) {
            let var = name.clone();
            *index += 3;
            return Stmt::Decrement(var);
        }
    }

    // ─── 对象属性 / 索引赋值 ─────────────────────────────────────────────
    if matches!(tokens.get(*index), Some(Token { kind: TokenKind::Ident(_), .. })) {
        let backup = *index;
        let lhs_expr = parse_expr(tokens, index);

        // 只有当 lhs_expr 为访问属性或数组索引时，才认为是属性赋值
        if (matches!(lhs_expr, Expr::Access(_, _) | Expr::Index(_, _)))
            && (matches!(tokens.get(*index), Some(Token { kind: TokenKind::Assign, .. }))
                || matches!(tokens.get(*index), Some(Token { kind: TokenKind::Eq, .. })))
        {
            *index += 1; // 跳过 := 或 =
            let rhs = parse_binary_expr(tokens, index);
            return Stmt::PropAssign(Box::new(lhs_expr), rhs);
        } else {
            // 不是属性赋值，回退到解析前
            *index = backup;
        }
    }

    // ✅ 变量/常量定义语句和普通变量赋值
    if let Some(Token { kind: TokenKind::Ident(name), .. }) = tokens.get(*index) {
        let name = name.clone();
        *index += 1;

        let mut var_type = None;

        if matches!(tokens.get(*index), Some(Token { kind: TokenKind::LParen, .. })) {
            *index += 1;
            var_type = match tokens.get(*index) {
                Some(Token { kind: TokenKind::Ident(t), .. }) => Some(t.clone()),
                _ => panic!(
                    "Parse error at line {}, col {}: Expected type inside ()",
                    tokens[*index].line,
                    tokens[*index].col
                ),
            };
            *index += 1;
            if !matches!(tokens.get(*index), Some(Token { kind: TokenKind::RParen, .. })) {
                panic!(
                    "Parse error at line {}, col {}: Expected )",
                    tokens[*index].line,
                    tokens[*index].col
                );
            }
            *index += 1;
        } else if matches!(tokens.get(*index), Some(Token { kind: TokenKind::LBracket, .. })) {
            *index += 1;
            var_type = match tokens.get(*index) {
                Some(Token { kind: TokenKind::Ident(t), .. }) => Some(t.clone()),
                _ => panic!(
                    "Parse error at line {}, col {}: Expected type inside []",
                    tokens[*index].line,
                    tokens[*index].col
                ),
            };
            *index += 1;
            if !matches!(tokens.get(*index), Some(Token { kind: TokenKind::RBracket, .. })) {
                panic!(
                    "Parse error at line {}, col {}: Expected ]",
                    tokens[*index].line,
                    tokens[*index].col
                );
            }
            *index += 1;
        }

        if matches!(tokens.get(*index), Some(Token { kind: TokenKind::Assign, .. })) {
            *index += 1;
            let expr = parse_binary_expr(tokens, index);
            return Stmt::ConstDecl(name, var_type, expr);
        } else if matches!(tokens.get(*index), Some(Token { kind: TokenKind::Colon, .. })) {
            *index += 1;

            match tokens.get(*index) {
                Some(Token { kind: TokenKind::LParen, .. }) => {
                    // 常量: (type) = value
                    *index += 1;
                    let const_type = match tokens.get(*index) {
                        Some(Token { kind: TokenKind::Ident(t), .. }) => Some(t.clone()),
                        _ => panic!(
                            "Parse error at line {}, col {}: Expected type after :(",
                            tokens[*index].line,
                            tokens[*index].col
                        ),
                    };
                    *index += 1;
                    if !matches!(tokens.get(*index), Some(Token { kind: TokenKind::RParen, .. })) {
                        panic!(
                            "Parse error at line {}, col {}: Expected ) after constant type",
                            tokens[*index].line,
                            tokens[*index].col
                        );
                    }
                    *index += 1;
                    if !matches!(tokens.get(*index), Some(Token { kind: TokenKind::Eq, .. })) {
                        panic!(
                            "Parse error at line {}, col {}: Expected = after constant type",
                            tokens[*index].line,
                            tokens[*index].col
                        );
                    }
                    *index += 1;
                    let expr = parse_binary_expr(tokens, index);
                    return Stmt::ConstDecl(name, const_type, expr);
                }
                Some(Token { kind: TokenKind::LBracket, .. }) => {
                    // 变量: [type] = value
                    *index += 1;
                    let var_type = match tokens.get(*index) {
                        Some(Token { kind: TokenKind::Ident(t), .. }) => Some(t.clone()),
                        _ => panic!(
                            "Parse error at line {}, col {}: Expected type after :[",
                            tokens[*index].line,
                            tokens[*index].col
                        ),
                    };
                    *index += 1;
                    if !matches!(tokens.get(*index), Some(Token { kind: TokenKind::RBracket, .. })) {
                        panic!(
                            "Parse error at line {}, col {}: Expected ] after variable type",
                            tokens[*index].line,
                            tokens[*index].col
                        );
                    }
                    *index += 1;
                    if !matches!(tokens.get(*index), Some(Token { kind: TokenKind::Eq, .. })) {
                        panic!(
                            "Parse error at line {}, col {}: Expected = after variable type",
                            tokens[*index].line,
                            tokens[*index].col
                        );
                    }
                    *index += 1;
                    let expr = parse_binary_expr(tokens, index);
                    return Stmt::VarDecl(name, var_type, expr);
                }
                _ => {
                    panic!(
                        "Parse error at line {}, col {}: Expected :() or :[] for type declaration",
                        tokens[*index - 1].line,
                        tokens[*index - 1].col
                    );
                }
            }
        } else if matches!(tokens.get(*index), Some(Token { kind: TokenKind::Eq, .. })) {
            // 普通变量赋值（允许 a = 加法🧮(a, b)）
            *index += 1;
            let expr = parse_binary_expr(tokens, index);
            return Stmt::Assign(name, expr);
        } else {
            println!("当前 Token：{:?}", tokens.get(*index));
            panic!(
                "Parse error at line {}, col {}: Expected := after variable name",
                tokens[*index].line,
                tokens[*index].col
            );
        }
    }


    panic!(
        "Parse error at line {}, col {}: Unknown statement",
        tokens[*index].line,
        tokens[*index].col
    )
}

fn parse_binary_expr(tokens: &[Token], index: &mut usize) -> Expr {
    // 支持多重比较和多重等式判断 a > b > c, a = b = c, a < b < c
    let mut exprs = Vec::new();
    let mut ops = Vec::new();

    // 首先解析左侧表达式
    exprs.push(parse_expr(tokens, index));

    // 不断尝试读取运算符和右侧表达式
    while let Some(op_token) = tokens.get(*index) {

        // 检测两字符运算符 <=, >=, ==, &&, ||
        let op_str = if matches!(op_token, Token { kind: TokenKind::Unknown('<'), .. })
            && (matches!(tokens.get(*index + 1), Some(Token { kind: TokenKind::Unknown('='), .. }))
                 || matches!(tokens.get(*index + 1), Some(Token { kind: TokenKind::Eq, .. })))
        {
            *index += 2;
            "<=".to_string()
        } else if matches!(op_token, Token { kind: TokenKind::Unknown('>'), .. })
            && (matches!(tokens.get(*index + 1), Some(Token { kind: TokenKind::Unknown('='), .. }))
                 || matches!(tokens.get(*index + 1), Some(Token { kind: TokenKind::Eq, .. })))
        {
            *index += 2;
            ">=".to_string()
        } else if matches!(op_token, Token { kind: TokenKind::Eq, .. })
            && matches!(tokens.get(*index + 1), Some(Token { kind: TokenKind::Eq, .. }))
        {
            *index += 2;
            "==".to_string()
        } else if matches!(op_token, Token { kind: TokenKind::Unknown('&'), .. })
            && matches!(tokens.get(*index + 1), Some(Token { kind: TokenKind::Unknown('&'), .. }))
        {
            *index += 2;
            "&&".to_string()
        } else if matches!(op_token, Token { kind: TokenKind::Unknown('|'), .. })
            && matches!(tokens.get(*index + 1), Some(Token { kind: TokenKind::Unknown('|'), .. }))
        {
            *index += 2;
            "||".to_string()
        } else {
            // 单字符运算符
            let single = match op_token.kind {
                TokenKind::Unknown('+') => "+",
                TokenKind::Unknown('-') => "-",
                TokenKind::Unknown('*') => "*",
                TokenKind::Unknown('/') => "/",
                TokenKind::Unknown('>') => ">",
                TokenKind::Unknown('<') => "<",
                TokenKind::Unknown('&') => "&",
                TokenKind::Unknown('|') => "|",
                TokenKind::Eq         => "=",
                _ => {
                    break;
                }
            };
            *index += 1;
            single.to_string()
        };

        // 解析右侧表达式
        let next_expr = parse_expr(tokens, index);

        // 判断是否为比较或逻辑运算符，以决定是否链式
        if ["<", ">", "=", "<=", ">=", "==", "&&", "||"].contains(&op_str.as_str()) {
            exprs.push(next_expr);
            ops.push(op_str);
        } else {
            // 普通算术：立刻构建节点
            let left = exprs.pop().unwrap();
            exprs.push(Expr::Binary(Box::new(left), op_str, Box::new(next_expr)));
        }
    }

    // 如果没有链式比较，直接返回最后一个表达式
    if ops.is_empty() {
        return exprs.pop().unwrap();
    }

    // 构造链式比较 ((a>b)&&(b>c)&&…)
    let mut result = Expr::Binary(
        Box::new(exprs[0].clone()),
        ops[0].clone(),
        Box::new(exprs[1].clone()),
    );
    for i in 1..ops.len() {
        let cmp = Expr::Binary(
            Box::new(exprs[i].clone()),
            ops[i].clone(),
            Box::new(exprs[i + 1].clone()),
        );
        result = Expr::Logical("&&".to_string(), Box::new(result), Box::new(cmp));
    }
    result
}

fn parse_expr(tokens: &[Token], index: &mut usize) -> Expr {
    match tokens.get(*index) {
        // Parentheses grouping: (expr)
        Some(Token { kind: TokenKind::LParen, .. }) => {
            // Consume '('
            *index += 1;
            // Parse inner expression
            let inner = parse_binary_expr(tokens, index);
            // Expect ')'
            if !matches!(tokens.get(*index), Some(Token { kind: TokenKind::RParen, .. })) {
                panic!(
                    "Parse error at line {}, col {}: Expected ')' to close grouping",
                    tokens[*index].line,
                    tokens[*index].col
                );
            }
            *index += 1;
            // Return grouped expr
            return inner;
        }
        Some(Token { kind: TokenKind::Number(n), .. }) => {
            *index += 1;
            Expr::Number(*n)
        }
        Some(Token { kind: TokenKind::Str(s), .. }) => {
            *index += 1;
            Expr::Str(s.clone())
        }
        Some(Token { kind: TokenKind::LBracket, .. }) => {
            // 解析数组字面量
            *index += 1;
            let mut elements = Vec::new();
            while !matches!(tokens.get(*index), Some(Token { kind: TokenKind::RBracket, .. })) {
                elements.push(parse_expr(tokens, index));
                if matches!(tokens.get(*index), Some(Token { kind: TokenKind::Comma, .. })) {
                    *index += 1;
                }
            }
            *index += 1;
            // 支持 [1,2,3][1][2] 这样的链式索引
            let mut expr = Expr::Array(elements);
            while matches!(tokens.get(*index), Some(Token { kind: TokenKind::LBracket, .. })) {
                *index += 1;
                let idx = parse_binary_expr(tokens, index);
                if !matches!(tokens.get(*index), Some(Token { kind: TokenKind::RBracket, .. })) {
                    panic!("Expected closing bracket ] for array index");
                }
                *index += 1;
                expr = Expr::Index(Box::new(expr), Box::new(idx));
            }
            return expr;
        }
        Some(Token { kind: TokenKind::LBrace, .. }) => {
            // Parse object literal: { key: value, ... }
            *index += 1;
            let mut props = Vec::new();
            while !matches!(tokens.get(*index), Some(Token { kind: TokenKind::RBrace, .. })) {
                let key = match tokens.get(*index) {
                    Some(Token { kind: TokenKind::Ident(k), .. }) => k.clone(),
                    Some(Token { kind: TokenKind::Str(s), .. }) => s.clone(),
                    _ => panic!("Expected key in object literal"),
                };
                *index += 1;
                if !matches!(tokens.get(*index), Some(Token { kind: TokenKind::Colon, .. })) {
                    panic!("Expected ':' after object key");
                }
                *index += 1;
                let value = parse_expr(tokens, index);
                props.push((key, Box::new(value)));
                if matches!(tokens.get(*index), Some(Token { kind: TokenKind::Comma, .. })) {
                    *index += 1;
                }
            }
            *index += 1;
            Expr::Object(props)
        }
        Some(Token { kind: TokenKind::Ident(id), .. }) => {
            let id = id.clone();
            *index += 1;
            // Input expression: I>[prompt?, type?, limit?]
            if id == "I"
                && matches!(tokens.get(*index), Some(Token { kind: TokenKind::Unknown('>'), .. }))
                && matches!(tokens.get(*index + 1), Some(Token { kind: TokenKind::LBracket, .. }))
            {
                // consume '>' and '['
                *index += 2;
                let mut args = Vec::new();
                let mut need_default = true;

                while !matches!(tokens.get(*index), Some(Token { kind: TokenKind::RBracket, .. })) {
                    // handle commas indicating omitted args
                    if matches!(tokens.get(*index), Some(Token { kind: TokenKind::Comma, .. })) {
                        if need_default {
                            args.push(Expr::Str("".into()));
                        }
                        *index += 1;
                        need_default = true;
                        continue;
                    }
                    // parse provided argument
                    args.push(parse_binary_expr(tokens, index));
                    need_default = false;
                    // skip optional comma
                    if matches!(tokens.get(*index), Some(Token { kind: TokenKind::Comma, .. })) {
                        *index += 1;
                        need_default = true;
                    }
                }
                // pad defaults up to three args
                while args.len() < 3 {
                    args.push(Expr::Str("".into()));
                }
                // consume ']'
                *index += 1;
                return Expr::Input(args);
            }
            // Handle function call
            if matches!(tokens.get(*index), Some(Token { kind: TokenKind::LParen, .. })) {
                let args = parse_call_args(tokens, index);
                return Expr::Call(id, args);
            }
            // 支持 a[1][2].b[3] 这样的链式索引和属性访问
            let mut expr = Expr::Ident(id);
            loop {
                if matches!(tokens.get(*index), Some(Token { kind: TokenKind::Dot, .. })) {
                    *index += 1;
                    let prop = match tokens.get(*index) {
                        Some(Token { kind: TokenKind::Ident(p), .. }) => p.clone(),
                        _ => panic!("Expected property name after '.'"),
                    };
                    *index += 1;
                    expr = Expr::Access(Box::new(expr), prop);
                } else if matches!(tokens.get(*index), Some(Token { kind: TokenKind::LBracket, .. })) {
                    *index += 1;
                    let idx = parse_binary_expr(tokens, index);
                    if !matches!(tokens.get(*index), Some(Token { kind: TokenKind::RBracket, .. })) {
                        panic!("Expected closing bracket ] for array index");
                    }
                    *index += 1;
                    expr = Expr::Index(Box::new(expr), Box::new(idx));
                } else {
                    break;
                }
            }
            // support postfix ++/--
            if let Expr::Ident(name) = &expr {
                if matches!(tokens.get(*index), Some(Token { kind: TokenKind::Unknown('+'), .. }))
                    && matches!(tokens.get(*index + 1), Some(Token { kind: TokenKind::Unknown('+'), .. })) {
                    *index += 2;
                    return Expr::PostfixIncrement(name.clone());
                }
                if matches!(tokens.get(*index), Some(Token { kind: TokenKind::Unknown('-'), .. }))
                    && matches!(tokens.get(*index + 1), Some(Token { kind: TokenKind::Unknown('-'), .. })) {
                    *index += 2;
                    return Expr::PostfixDecrement(name.clone());
                }
            }
            return expr;
        }
        _ => panic!("Unsupported expression"),
    }
}



fn parse_call_args(tokens: &[Token], index: &mut usize) -> Vec<Expr> {
    let mut args = Vec::new();
    if matches!(tokens.get(*index), Some(Token { kind: TokenKind::LParen, .. })) {
        *index += 1;
        while !matches!(tokens.get(*index), Some(Token { kind: TokenKind::RParen, .. })) {
            args.push(parse_binary_expr(tokens, index));
            if matches!(tokens.get(*index), Some(Token { kind: TokenKind::Comma, .. })) {
                *index += 1;
            }
        }
        *index += 1;
    }
    args
}
// 新的循环语句解析函数，支持多种循环格式
fn parse_loop_stmt(tokens: &[Token], index: &mut usize) -> Stmt {
    use crate::ast::LoopKind;

    if !matches!(tokens.get(*index), Some(Token { kind: TokenKind::Loop, .. })) {
        panic!("Expected Loop");
    }
    *index += 1;

    let loop_kind = match tokens.get(*index) {
        Some(Token { kind: TokenKind::LBracket, .. }) => {
            *index += 1;
            let expr = parse_binary_expr(tokens, index);
            // Accept either ] or { directly after the expr, for L>[10]{...}
            if !matches!(tokens.get(*index), Some(Token { kind: TokenKind::RBracket, .. }))
                && !matches!(tokens.get(*index), Some(Token { kind: TokenKind::LBrace, .. }))
            {
                panic!("Expected ']' or '{{' after loop expression");
            }
            if matches!(tokens.get(*index), Some(Token { kind: TokenKind::RBracket, .. })) {
                *index += 1;
            }
            LoopKind::Times(expr)
        }
        Some(Token { kind: TokenKind::Ident(data), .. }) => {
            let data = data.clone();
            *index += 1;
            if let Some(Token { kind: TokenKind::Colon, .. }) = tokens.get(*index) {
                *index += 1;
                let item = match tokens.get(*index) {
                    Some(Token { kind: TokenKind::Ident(var), .. }) => var.clone(),
                    _ => panic!("Expected variable name after ':'"),
                };
                *index += 1;
                LoopKind::ForEach(item, Expr::Ident(data))
            } else {
                panic!("Expected ':' after iterable identifier");
            }
        }
        Some(Token { kind: TokenKind::LParen, .. }) => {
            // Decide While vs For by counting semicolons up to the matching RParen
            let mut semi_count = 0;
            let mut depth = 1;
            for look in (*index + 1)..tokens.len() {
                match tokens.get(look) {
                    Some(Token { kind: TokenKind::LParen, .. }) => depth += 1,
                    Some(Token { kind: TokenKind::RParen, .. }) => {
                        depth -= 1;
                        if depth == 0 { break; }
                    }
                    Some(Token { kind: TokenKind::Semicolon, .. }) 
                    | Some(Token { kind: TokenKind::Unknown(';'), .. }) if depth == 1 => semi_count += 1,
                    _ => {}
                }
            }
            let is_for = semi_count == 2;


            *index += 1;
            let mut parts = Vec::new();
            if !is_for {
                // While-style: parse a single expression as condition
                while !matches!(tokens.get(*index), Some(Token { kind: TokenKind::RParen, .. }))
                    && !matches!(tokens.get(*index), Some(Token { kind: TokenKind::LBrace, .. }))
                {
                    parts.push(parse_binary_expr(tokens, index));
                    // Only advance on semicolon if present; do not require it
                    if matches!(tokens.get(*index), Some(Token { kind: TokenKind::Semicolon, .. })) 
                        || matches!(tokens.get(*index), Some(Token { kind: TokenKind::Unknown(';'), .. })) {
                        *index += 1;
                    } else if !matches!(tokens.get(*index), Some(Token { kind: TokenKind::RParen, .. }))
                        && !matches!(tokens.get(*index), Some(Token { kind: TokenKind::LBrace, .. }))
                    {
                        // Enhance debug info before panicking
                        println!("DEBUG parse_loop_stmt unexpected token at index {}: {:?}", index, tokens.get(*index));
                        if let Some(tok) = tokens.get(*index) {
                            println!("  Token detail - kind: {:?}, line: {}, col: {}", tok.kind, tok.line, tok.col);
                        }
                        println!("  Next tokens: {:?}", tokens.get(*index+1));
                        panic!("Expected ';', ')', or '{{' in loop header");
                    }
                }
                // Accept closing ) if present
                if matches!(tokens.get(*index), Some(Token { kind: TokenKind::RParen, .. })) {
                    *index += 1;
                }
                if parts.len() != 1 {
                    panic!("Invalid while-loop header: expected 1 condition expression");
                }
                LoopKind::While(parts.remove(0))
            } else {
                // For-style: parse init; cond; step
                // Parse init statement
                let init = {
                    let stmt = parse_stmt(tokens, index);
                    Box::new(stmt)
                };
                // require semicolon
                if !(matches!(tokens.get(*index), Some(Token { kind: TokenKind::Semicolon, .. })) 
                    || matches!(tokens.get(*index), Some(Token { kind: TokenKind::Unknown(';'), .. }))) {
                    panic!("Expected ';' after init in for-loop header");
                }
                *index += 1;
                // parse condition
                let cond = parse_binary_expr(tokens, index);
                if !(matches!(tokens.get(*index), Some(Token { kind: TokenKind::Semicolon, .. })) 
                    || matches!(tokens.get(*index), Some(Token { kind: TokenKind::Unknown(';'), .. }))) {
                    panic!("Expected ';' after condition in for-loop header");
                }
                *index += 1;
                // parse step statement
                let step = {
                    let stmt = parse_stmt(tokens, index);
                    Box::new(stmt)
                };
                // Accept closing ) if present
                if matches!(tokens.get(*index), Some(Token { kind: TokenKind::RParen, .. })) {
                    *index += 1;
                }
                LoopKind::For(init, cond, step)
            }
        }
        _ => panic!("Unknown loop format"),
    };

    // Accept either '{' after loop header, or error
    if !matches!(tokens.get(*index), Some(Token { kind: TokenKind::LBrace, .. })) {
        panic!("Expected '{{' after loop header");
    }
    *index += 1;

    let mut body = Vec::new();
    while !matches!(tokens.get(*index), Some(Token { kind: TokenKind::RBrace, .. })) {
        body.push(parse_stmt(tokens, index));
    }
    *index += 1;

    Stmt::Loop(loop_kind, body)
}