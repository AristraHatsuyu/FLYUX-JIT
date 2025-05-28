use std::fs;
use std::collections::HashMap;
use crate::lexer::tokenize;
use crate::parser::parse;
use crate::ast::{Expr, Stmt, Function};

pub enum ExecResult {
    None,
    Return(String),
}

pub fn execute_file(path: &str) {
    match fs::read_to_string(path) {
        Ok(content) => {
            let tokens = tokenize(&content);
            let ast = parse(&tokens);

            /*
            println!("Parsed functions:");
            for f in &ast {
                println!("- {}", f.name);
            }
            */

            let mut function_table: HashMap<String, &Function> = HashMap::new();
            for f in &ast {
                function_table.insert(f.name.clone(), f);
            }

            if let Some(main_fn) = function_table.get("main") {
                // println!("Executing main()...");
                let mut ctx = HashMap::new();
                exec_with_ctx(main_fn, &mut ctx, &function_table);
            }
        },
        Err(err) => {
            eprintln!("Failed to read file: {err}");
        }
    }
}

pub fn dump_tokens(path: &str) {
    match std::fs::read_to_string(path) {
        Ok(content) => {
            let tokens = crate::lexer::tokenize(&content);
            for token in tokens {
                println!("{:?}", token);
            }
        },
        Err(err) => eprintln!("Failed to read file: {err}"),
    }
}

pub fn dump_ast(path: &str) {
    match std::fs::read_to_string(path) {
        Ok(content) => {
            let tokens = crate::lexer::tokenize(&content);
            let ast = crate::parser::parse(&tokens);
            println!("{:#?}", ast);
        },
        Err(err) => eprintln!("Failed to read file: {err}"),
    }
}

pub fn syntax_check(path: &str) {
    match std::fs::read_to_string(path) {
        Ok(content) => {
            let tokens = crate::lexer::tokenize(&content);
            let _ = crate::parser::parse(&tokens);
            println!("Syntax OK.");
        },
        Err(err) => eprintln!("Failed to read file: {err}"),
    }
}

pub fn exec_with_ctx(
    func: &Function,
    ctx: &mut HashMap<String, (String, Option<String>, bool)>, // value, type, is_const
    fns: &HashMap<String, &Function>
) -> ExecResult {
    for stmt in &func.body {
        match stmt {
            Stmt::ConstDecl(name, typ, expr) => {
                let mut val = eval_expr(expr, ctx, fns);
                let inferred_type = typ.clone().or_else(|| infer_type(&val));
                let expected_type = typ.clone().unwrap_or_else(|| inferred_type.clone().unwrap_or("string".into()));
                if let Some(t) = &typ {
                    if !["int", "float", "bool", "string", "obj"].contains(&t.as_str()) {
                        panic!("Unknown type '{}'", t);
                    }
                }
                if expected_type == "bool" {
                    let normalized = val.trim_matches('"').to_lowercase();
                    val = match normalized.as_str() {
                        "true" | "1" => "1".to_string(),
                        "false" | "0" => "0".to_string(),
                        _ => panic!("Invalid boolean literal: '{}'", val),
                    };
                } else if expected_type == "int" {
                    val.parse::<i64>().unwrap_or_else(|_| panic!("Invalid int literal: '{}'", val));
                } else if expected_type == "float" {
                    val.parse::<f64>().unwrap_or_else(|_| panic!("Invalid float literal: '{}'", val));
                } else if expected_type == "string" {
                    // 如果原始表达式为 Expr::Str 则直接使用，无需二次解析
                    if val.starts_with('"') && val.ends_with('"') {
                        val = val[1..val.len()-1].to_string(); // 去除引号
                    } else if val.parse::<i64>().is_ok() || val.parse::<f64>().is_ok() {
                        val = val.to_string();
                    } else if val == "true" || val == "false" {
                        val = val.clone(); // 保持字符串 "true"/"false"
                    } else if ["<null>", "<undef>"].contains(&val.to_lowercase().as_str()) {
                        val = val.to_string();
                    } else if val.starts_with('[') && val.ends_with(']') {
                        val = val.to_string(); // 保留数组格式
                    } else {
                        // 其他必须加引号
                        panic!("String literal must be quoted: '{}'", val);
                    }
                }

                if let Some((_, _, true)) = ctx.get(name) {
                    panic!("Cannot redefine constant '{}'", name);
                }

                ctx.insert(name.clone(), (val, Some(expected_type), typ.is_some()));
            }

            Stmt::VarDecl(name, typ, expr) => {
                let mut val = eval_expr(expr, ctx, fns);
                let expected_type = typ.clone().unwrap_or_else(|| infer_type(&val).unwrap_or("string".into()));
                if expected_type == "bool" {
                    let normalized = val.trim_matches('"').to_lowercase();
                    val = match normalized.as_str() {
                        "true" | "1" => "1".to_string(),
                        "false" | "0" => "0".to_string(),
                        _ => panic!("Invalid boolean literal: '{}'", val),
                    };
                } else if expected_type == "int" {
                    val.parse::<i64>().unwrap_or_else(|_| panic!("Invalid int literal: '{}'", val));
                } else if expected_type == "float" {
                    val.parse::<f64>().unwrap_or_else(|_| panic!("Invalid float literal: '{}'", val));
                } else if expected_type == "string" {
                    if val.starts_with('"') && val.ends_with('"') {
                        val = val[1..val.len()-1].to_string(); // strip quotes
                    } else if val.parse::<i64>().is_ok() || val.parse::<f64>().is_ok() {
                        val = val.to_string();
                    } else if val == "true" || val == "false" {
                        val = val.clone();
                    } else if ["<null>", "<undef>"].contains(&val.to_lowercase().as_str()) {
                        val = val.to_string();
                    } else if val.starts_with('[') && val.ends_with(']') {
                        val = val.to_string(); // 保留数组格式
                    } else {
                        panic!("String literal must be quoted: '{}'", val);
                    }
                }
                ctx.insert(name.clone(), (val, Some(expected_type), false));
            }

            Stmt::Expr(Expr::Call(fname, args)) => {
                if fname == "print" {
                    let output: Vec<String> = args.iter().map(|e| {
                        match e {
                            Expr::Str(s) => s.clone(),
                            _ => eval_expr(e, ctx, fns)
                        }
                    }).collect();
                    println!("{}", output.join(" "));
                } else if let Some(f) = fns.get(fname) {
                    let mut local_ctx = HashMap::new();
                    for (i, (pname, _ptype)) in f.params.iter().enumerate() {
                        let arg_val = args.get(i).map(|e| eval_expr(e, ctx, fns)).unwrap_or_default();
                        local_ctx.insert(pname.clone(), (arg_val, None, false));
                    }
                    let result = exec_with_ctx(f, &mut local_ctx, fns);
                    if let ExecResult::Return(v) = result {
                        return ExecResult::Return(v);
                    }
                }
            }

            Stmt::Return(expr) => {
                let val = eval_expr(expr, ctx, fns);
                return ExecResult::Return(val);
            }

            Stmt::Assign(name, expr) => {
                let value = eval_expr(expr, ctx, fns);
                if let Some((_, typ, is_const)) = ctx.get(name) {
                    println!("Assign attempt to '{}', context entry: {:?}", name, ctx.get(name)); // 调试输出
                    if *is_const {
                        panic!("Cannot assign to constant '{}'", name);
                    }
                    let enforced = if let Some(t) = typ {
                        if t == "int" {
                            value.parse::<i64>().unwrap_or_else(|_| panic!("Type mismatch: expected int, got '{}'", value)).to_string()
                        } else if t == "float" {
                            value.parse::<f64>().unwrap_or_else(|_| panic!("Type mismatch: expected float, got '{}'", value)).to_string()
                        } else if t == "bool" {
                            let v = value.to_lowercase();
                            match v.as_str() {
                                "true" | "1" => "1".to_string(),
                                "false" | "0" => "0".to_string(),
                                _ => panic!("Type mismatch: expected bool, got '{}'", value)
                            }
                        } else if t == "string" {
                            value
                        } else {
                            panic!("Unsupported type '{}'", t);
                        }
                    } else {
                        value
                    };
                    ctx.insert(name.clone(), (enforced, typ.clone(), false));
                } else {
                    panic!("Undefined variable '{}'", name);
                }
            }

            Stmt::Loop(var, expr, body) => {
                let count_str = eval_expr(expr, ctx, fns);
                let count = count_str.parse::<usize>().unwrap_or(0);
                for i in 0..count {
                    ctx.insert(var.clone(), (i.to_string(), Some("int".to_string()), false));
                    for stmt in body {
                        match stmt {
                            Stmt::ConstDecl(name, typ, expr) => {
                                let mut val = eval_expr(expr, ctx, fns);
                                let inferred_type = typ.clone().or_else(|| infer_type(&val));
                                let expected_type = typ.clone().unwrap_or_else(|| inferred_type.clone().unwrap_or("string".into()));
                                if let Some(t) = &typ {
                                    if !["int", "float", "bool", "string", "obj"].contains(&t.as_str()) {
                                        panic!("Unknown type '{}'", t);
                                    }
                                }
                                if expected_type == "bool" {
                                    let normalized = val.trim_matches('"').to_lowercase();
                                    val = match normalized.as_str() {
                                        "true" | "1" => "1".to_string(),
                                        "false" | "0" => "0".to_string(),
                                        _ => panic!("Invalid boolean literal: '{}'", val),
                                    };
                                } else if expected_type == "int" {
                                    val.parse::<i64>().unwrap_or_else(|_| panic!("Invalid int literal: '{}'", val));
                                } else if expected_type == "float" {
                                    val.parse::<f64>().unwrap_or_else(|_| panic!("Invalid float literal: '{}'", val));
                                } else if expected_type == "string" {
                                    // 如果原始表达式为 Expr::Str 则直接使用，无需二次解析
                                    if val.starts_with('"') && val.ends_with('"') {
                                        val = val[1..val.len()-1].to_string(); // 去除引号
                                    } else if val.parse::<i64>().is_ok() || val.parse::<f64>().is_ok() {
                                        val = val.to_string();
                                    } else if val == "true" || val == "false" {
                                        val = val.clone(); // 保持字符串 "true"/"false"
                                    } else if ["<null>", "<undef>"].contains(&val.to_lowercase().as_str()) {
                                        val = val.to_string();
                                    } else if val.starts_with('[') && val.ends_with(']') {
                                        val = val.to_string(); // 保留数组格式
                                    } else {
                                        // 其他必须加引号
                                        panic!("String literal must be quoted: '{}'", val);
                                    }
                                }
                                if let Some((_, _, true)) = ctx.get(name) {
                                    panic!("Cannot redefine constant '{}'", name);
                                }
                                ctx.insert(name.clone(), (val, Some(expected_type), typ.is_some()));
                            }
                            _ => {}
                        }
                    }
                }
            }

            // Stmt::IfElse removed

            Stmt::MultiIf(branches) => {
                let mut executed = false;
                for (cond, body) in branches {
                    if executed {
                        break;
                    }
                    let passed = match cond {
                        Some(expr) => {
                            let v = eval_expr(expr, ctx, fns);
                            v != "0" && v != "false"
                        }
                        None => true
                    };
                    if passed {
                        for stmt in body {
                            exec_with_ctx(&Function {
                                name: "<if>".into(),
                                params: vec![],
                                body: vec![stmt.clone()],
                            }, ctx, fns);
                        }
                        executed = true;
                    }
                }
            }
            _ => {}
        }
    }

    ExecResult::None
}

fn eval_expr(
    expr: &Expr,
    ctx: &HashMap<String, (String, Option<String>, bool)>,
    fns: &HashMap<String, &Function>
) -> String {
    match expr {
        Expr::Number(n) => n.to_string(),
        Expr::Str(s) => format!("\"{}\"", s),
        Expr::Ident(id) => {
            match id.as_str() {
                "true" => "true".to_string(),
                "false" => "false".to_string(),
                _ => {
                    if let Some((val, _, _)) = ctx.get(id) {
                        val.clone()
                    } else if id.parse::<i64>().is_ok() || id.parse::<f64>().is_ok() || ["<null>", "<undef>"].contains(&id.to_lowercase().as_str()) {
                        id.to_string()
                    } else {
                        panic!("Undefined identifier: '{}'", id)
                    }
                }
            }
        },
        Expr::Call(name, args) => {
            if let Some(f) = fns.get(name) {
                let mut local_ctx = HashMap::new();
                for (i, (pname, _ptype)) in f.params.iter().enumerate() {
                    let arg_val = args.get(i).map(|e| eval_expr(e, ctx, fns)).unwrap_or_default();
                    local_ctx.insert(pname.clone(), (arg_val, None, false));
                }
                match exec_with_ctx(f, &mut local_ctx, fns) {
                    ExecResult::Return(v) => v,
                    _ => "<void>".to_string()
                }
            } else {
                "<unknown-fn>".to_string()
            }
        }
        Expr::Binary(lhs, op, rhs) => {
            let l = eval_expr(lhs, ctx, fns);
            let r = eval_expr(rhs, ctx, fns);
            let lnum = l.parse::<f64>().unwrap_or(0.0);
            let rnum = r.parse::<f64>().unwrap_or(0.0);
            match op.as_str() {
                "+" => format!("{}", lnum + rnum),
                "-" => format!("{}", lnum - rnum),
                "*" => format!("{}", lnum * rnum),
                "/" => format!("{}", if rnum != 0.0 { lnum / rnum } else { 0.0 }),
                ">" => (lnum > rnum).to_string(),
                "<" => (lnum < rnum).to_string(),
                "=" => (l == r).to_string(),
                _ => "<bad-op>".to_string()
            }
        }
        Expr::Logical(op, left, right) => {
            let l = eval_expr(left, ctx, fns) == "true";
            let r = eval_expr(right, ctx, fns) == "true";
            match op.as_str() {
                "&&" => (l && r).to_string(),
                "||" => (l || r).to_string(),
                _ => panic!("Unsupported logical operator: {}", op),
            }
        }
        // ---- 新增数组表达式支持 ----
        Expr::Array(elements) => {
            let values: Vec<String> = elements.iter().map(|e| eval_expr(e, ctx, fns)).collect();
            format!("[{}]", values.join(","))
        }
        Expr::Index(array_expr, index_expr) => {
            let arr_str = eval_expr(array_expr, ctx, fns);
            let idx_str = eval_expr(index_expr, ctx, fns);
            let idx = idx_str.parse::<usize>().unwrap_or_else(|_| panic!("Invalid index: '{}'", idx_str));
            let trimmed = arr_str.trim_matches(&['[', ']'][..]);
            let elements: Vec<&str> = trimmed.split(',').map(|s| s.trim()).collect();
            if idx >= elements.len() {
                panic!("Array index out of bounds: {}", idx);
            }
            elements[idx].to_string()
        }
    }
}

fn infer_type(val: &str) -> Option<String> {
    if val.parse::<i64>().is_ok() {
        Some("int".to_string())
    } else if val.parse::<f64>().is_ok() {
        Some("float".to_string())
    } else if val.to_lowercase() == "true" || val.to_lowercase() == "false" {
        Some("bool".to_string())
    } else if val.trim().starts_with('[') && val.trim().ends_with(']') {
        Some("obj".to_string())
    } else {
        Some("string".to_string())
    }
}