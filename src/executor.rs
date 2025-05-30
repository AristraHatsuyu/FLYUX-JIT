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

            let mut function_table: HashMap<String, &Function> = HashMap::new();
            for f in &ast {
                function_table.insert(f.name.clone(), f);
            }

            if let Some(main_fn) = function_table.get("main") {
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
                        "true" | "1" => "true".to_string(),
                        "false" | "0" => "false".to_string(),
                        _ => panic!("Invalid boolean literal: '{}'", val),
                    };
                } else if expected_type == "int" {
                    val.parse::<i64>().unwrap_or_else(|_| panic!("Invalid int literal: '{}'", val));
                } else if expected_type == "float" {
                    val.parse::<f64>().unwrap_or_else(|_| panic!("Invalid float literal: '{}'", val));
                } else if expected_type == "string" {
                    if val.starts_with('"') && val.ends_with('"') {
                        val = val[1..val.len()-1].to_string(); // 去除引号
                    } else {
                        val = val.to_string(); // 放宽要求，允许非引号包裹的字符串（如对象、数组或字面量）
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
                        val = val[1..val.len()-1].to_string(); // 去除引号
                    } else {
                        val = val.to_string(); // 放宽要求，允许非引号包裹的字符串（如对象、数组或字面量）
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
                                "true" | "1"  => "true".to_string(),
                                "false" | "0" => "false".to_string(),
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

            Stmt::Loop(kind, body) => {
                match kind.clone() {
                    crate::ast::LoopKind::Times(expr) => {
                        let count = eval_expr(&expr, ctx, fns)
                            .parse::<usize>()
                            .unwrap_or_else(|_| panic!("Invalid loop count: {:?}", expr));
                        for i in 0..count {
                            ctx.insert("_".to_string(), (i.to_string(), Some("int".to_string()), false));
                            for stmt in body {
                                exec_with_ctx(&Function {
                                    name: "<loop>".into(),
                                    params: vec![],
                                    body: vec![stmt.clone()],
                                }, ctx, fns);
                            }
                        }
                    }
                    crate::ast::LoopKind::While(expr) => {
                        while {
                            let cond = eval_expr(&expr, ctx, fns);
                            cond != "0" && cond.to_lowercase() != "false"
                        } {
                            for stmt in body {
                                exec_with_ctx(&Function {
                                    name: "<while>".into(),
                                    params: vec![],
                                    body: vec![stmt.clone()],
                                }, ctx, fns);
                            }
                        }
                    }
                    crate::ast::LoopKind::ForEach(var, expr) => {
                        let list_val = eval_expr(&expr, ctx, fns);
                        if list_val.starts_with('[') && list_val.ends_with(']') {
                            let trimmed = &list_val[1..list_val.len()-1];
                            let mut elements = vec![];
                            let mut current = String::new();
                            let mut depth = 0;

                            for c in trimmed.chars() {
                                match c {
                                    '[' | '{' => {
                                        depth += 1;
                                        current.push(c);
                                    }
                                    ']' | '}' => {
                                        depth -= 1;
                                        current.push(c);
                                    }
                                    ',' if depth == 0 => {
                                        elements.push(current.trim().to_string());
                                        current.clear();
                                    }
                                    _ => current.push(c),
                                }
                            }
                            if !current.trim().is_empty() {
                                elements.push(current.trim().to_string());
                            }

                            for el in elements {
                                ctx.insert(var.clone(), (el.to_string(), Some("string".to_string()), false));
                                for stmt in body {
                                    exec_with_ctx(&Function {
                                        name: "<foreach>".into(),
                                        params: vec![],
                                        body: vec![stmt.clone()],
                                    }, ctx, fns);
                                }
                            }
                        } else {
                            panic!("For-each target is not an array: {}", list_val);
                        }
                    }
                    crate::ast::LoopKind::For(init, cond, step) => {
                        exec_with_ctx(&Function {
                            name: "<for-init>".into(),
                            params: vec![],
                            body: vec![*init.clone()],
                        }, ctx, fns);

                        while {
                            let cond_val = eval_expr(&cond, ctx, fns);
                            cond_val != "0" && cond_val.to_lowercase() != "false"
                        } {
                            for stmt in body {
                                exec_with_ctx(&Function {
                                    name: "<for-body>".into(),
                                    params: vec![],
                                    body: vec![stmt.clone()],
                                }, ctx, fns);
                            }

                            // Execute the step statement
                            exec_with_ctx(&Function {
                                name: "<for-step>".into(),
                                params: vec![],
                                body: vec![*step.clone()],
                            }, ctx, fns);
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
            Stmt::Increment(var) => {
                // println!("DEBUG: Incrementing '{}' from {:?}", var, ctx.get(var).unwrap());
                let (current_str, typ, is_const) = ctx.get(var)
                    .expect(&format!("Variable '{}' not found for increment", var))
                    .clone();
                let new_str = match typ.as_deref() {
                    Some("int") => {
                        let n = current_str.parse::<i64>()
                            .unwrap_or_else(|_| panic!("Invalid int for increment: '{}'", current_str));
                        (n + 1).to_string()
                    }
                    Some("float") => {
                        let f = current_str.parse::<f64>()
                            .unwrap_or_else(|_| panic!("Invalid float for increment: '{}'", current_str));
                        (f + 1.0).to_string()
                    }
                    _ => panic!("Unsupported type '{:?}' for increment", typ),
                };
                ctx.insert(var.clone(), (new_str, typ.clone(), is_const));
            }
            Stmt::Decrement(var) => {
                let (current_str, typ, is_const) = ctx.get(var)
                    .expect(&format!("Variable '{}' not found for decrement", var))
                    .clone();
                let new_str = match typ.as_deref() {
                    Some("int") => {
                        let n = current_str.parse::<i64>()
                            .unwrap_or_else(|_| panic!("Invalid int for decrement: '{}'", current_str));
                        (n - 1).to_string()
                    }
                    Some("float") => {
                        let f = current_str.parse::<f64>()
                            .unwrap_or_else(|_| panic!("Invalid float for decrement: '{}'", current_str));
                        (f - 1.0).to_string()
                    }
                    _ => panic!("Unsupported type '{:?}' for decrement", typ),
                };
                ctx.insert(var.clone(), (new_str, typ.clone(), is_const));
            }
            _ => {}
        }
    }

    ExecResult::None
}

fn eval_expr(
    expr: &Expr,
    ctx: &mut HashMap<String, (String, Option<String>, bool)>,
    fns: &HashMap<String, &Function>
) -> String {
    match expr {
        Expr::PostfixIncrement(var) => {
            // Evaluate and apply postfix increment: return new value after increment
            let (current_str, typ, _is_const) = ctx.get(var)
                .expect(&format!("Undefined identifier in postfix ++: '{}'", var))
                .clone();
            let old = current_str.clone();
            let new_str = match typ.as_deref() {
                Some("int") => {
                    let n = old.parse::<i64>()
                        .unwrap_or_else(|_| panic!("Invalid int in postfix ++: '{}'", old));
                    (n + 1).to_string()
                }
                Some("float") => {
                    let f = old.parse::<f64>()
                        .unwrap_or_else(|_| panic!("Invalid float in postfix ++: '{}'", old));
                    (f + 1.0).to_string()
                }
                _ => panic!("Unsupported type '{:?}' for postfix ++ on '{}'", typ, var),
            };
            ctx.insert(var.clone(), (new_str.clone(), typ.clone(), _is_const));
            new_str
        }
        Expr::PostfixDecrement(var) => {
            // Evaluate and apply postfix decrement: return new value after decrement
            let (current_str, typ, _is_const) = ctx.get(var)
                .expect(&format!("Undefined identifier in postfix --: '{}'", var))
                .clone();
            let old = current_str.clone();
            let new_str = match typ.as_deref() {
                Some("int") => {
                    let n = old.parse::<i64>()
                        .unwrap_or_else(|_| panic!("Invalid int in postfix --: '{}'", old));
                    (n - 1).to_string()
                }
                Some("float") => {
                    let f = old.parse::<f64>()
                        .unwrap_or_else(|_| panic!("Invalid float in postfix --: '{}'", old));
                    (f - 1.0).to_string()
                }
                _ => panic!("Unsupported type '{:?}' for postfix -- on '{}'", typ, var),
            };
            ctx.insert(var.clone(), (new_str.clone(), typ.clone(), _is_const));
            new_str
        }
        Expr::Number(n) => n.to_string(),
        Expr::Str(s) => s.clone(),
        Expr::Ident(id) => {
            match id.as_str() {
                "true" => "1".to_string(),
                "false" => "0".to_string(),
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
            let l_str = eval_expr(lhs, ctx, fns);
            let r_str = eval_expr(rhs, ctx, fns);
            let lnum = l_str.parse::<f64>().unwrap_or(0.0);
            let rnum = r_str.parse::<f64>().unwrap_or(0.0);
            let result = match op.as_str() {
                "+"  => format!("{}", lnum + rnum),
                "-"  => format!("{}", lnum - rnum),
                "*"  => format!("{}", lnum * rnum),
                "/"  => format!("{}", if rnum != 0.0 { lnum / rnum } else { 0.0 }),
                ">"  => (lnum >  rnum).to_string(),
                "<"  => (lnum <  rnum).to_string(),
                "="  => (l_str == r_str).to_string(),        // 如果单等号当作等于
                "==" => (l_str == r_str).to_string(),
                "<=" => (lnum <= rnum).to_string(),
                ">=" => (lnum >= rnum).to_string(),
                "&&" => {
                    // 非零 / 非 "false" 视为真
                    let lbool = !(l_str == "0" || l_str.eq_ignore_ascii_case("false"));
                    let rbool = !(r_str == "0" || r_str.eq_ignore_ascii_case("false"));
                    if lbool && rbool { "true".into() } else { "false".into() }
                }
                "||" => {
                    let lbool = !(l_str == "0" || l_str.eq_ignore_ascii_case("false"));
                    let rbool = !(r_str == "0" || r_str.eq_ignore_ascii_case("false"));
                    if lbool || rbool { "true".into() } else { "false".into() }
                }
                _other => {
                    "<bad-op>".to_string()
                }
            };
            result
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
        Expr::Array(elements) => {
            let values: Vec<String> = elements.iter().map(|e| eval_expr(e, ctx, fns)).collect();
            format!("[{}]", values.join(","))
        }
        Expr::Index(array_expr, index_expr) => {
            let target_str = eval_expr(array_expr, ctx, fns);
            let key = eval_expr(index_expr, ctx, fns).trim_matches('"').to_string();

            if target_str.starts_with('{') && target_str.ends_with('}') {
                // Handle object-style index
                let inner = &target_str[1..target_str.len()-1];
                let mut map = HashMap::new();
                let mut depth = 0;
                let mut current = String::new();
                let mut entries = vec![];

                for c in inner.chars() {
                    match c {
                        '{' | '[' => {
                            depth += 1;
                            current.push(c);
                        }
                        '}' | ']' => {
                            depth -= 1;
                            current.push(c);
                        }
                        ',' if depth == 0 => {
                            entries.push(current.trim().to_string());
                            current.clear();
                        }
                        _ => current.push(c),
                    }
                }
                if !current.trim().is_empty() {
                    entries.push(current.trim().to_string());
                }

                for kv in entries {
                    if let Some((k, v)) = kv.split_once(':') {
                        let parsed_key = k.trim().trim_matches('"').to_string();
                        let val = v.trim().to_string();
                        map.insert(parsed_key, val);
                    }
                }

                if let Some(value) = map.get(&key) {
                    value.clone()
                } else {
                    // fallback: return "" instead of panic
                    // println!("Key '{}' not found in object: {}", key, target_str);
                    "".to_string()
                }
            } else if target_str.starts_with('[') && target_str.ends_with(']') {
                // Handle array-style index with nested structure parsing
                let idx = key.parse::<usize>().unwrap_or_else(|_| panic!("Invalid index: '{}'", key));
                let trimmed = target_str.trim_matches(&['[', ']'][..]);

                let mut elements = vec![];
                let mut current = String::new();
                let mut depth = 0;

                for c in trimmed.chars() {
                    match c {
                        '{' | '[' => {
                            depth += 1;
                            current.push(c);
                        }
                        '}' | ']' => {
                            depth -= 1;
                            current.push(c);
                        }
                        ',' if depth == 0 => {
                            elements.push(current.trim().to_string());
                            current.clear();
                        }
                        _ => current.push(c),
                    }
                }
                if !current.trim().is_empty() {
                    elements.push(current.trim().to_string());
                }

                if idx >= elements.len() {
                    // fallback: return "" instead of panic
                    // panic!("Array index out of bounds: {}", idx);
                    "".to_string()
                } else {
                    elements[idx].to_string()
                }
            } else {
                // fallback: return "" instead of panic
                // panic!("Unsupported index target: {}", target_str)
                "".to_string()
            }
        }
        Expr::Access(obj_expr, prop) => {
            let obj_str = eval_expr(obj_expr, ctx, fns);
            if obj_str.starts_with('{') && obj_str.ends_with('}') {
                let inner = &obj_str[1..obj_str.len()-1];
                let mut pairs = HashMap::new();
                let mut depth = 0;
                let mut current = String::new();
                let mut entries = vec![];

                for c in inner.chars() {
                    match c {
                        '{' | '[' => {
                            depth += 1;
                            current.push(c);
                        }
                        '}' | ']' => {
                            depth -= 1;
                            current.push(c);
                        }
                        ',' if depth == 0 => {
                            entries.push(current.trim().to_string());
                            current.clear();
                        }
                        _ => current.push(c),
                    }
                }
                if !current.trim().is_empty() {
                    entries.push(current.trim().to_string());
                }

                for kv in entries {
                    if let Some((k, v)) = kv.split_once(':') {
                        let key = k.trim().trim_matches('"').to_string();
                        let val = v.trim().to_string();
                        pairs.insert(key, val);
                    }
                }

                if let Some(value) = pairs.get(prop) {
                    value.clone()
                } else {
                    println!("Accessing property '{}' from object string: {}", prop, obj_str);
                    println!("Parsed key-value pairs: {:?}", pairs);
                    panic!("Property '{}' not found in object", prop)
                }
            } else {
                panic!("Not an object: {}", obj_str)
            }
        }
        Expr::Object(pairs) => {
            let kvs: Vec<String> = pairs.iter()
                .map(|(k, v)| format!("\"{}\":{}", k, eval_expr(v, ctx, fns)))
                .collect();
            format!("{{{}}}", kvs.join(","))
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
    } else if val.trim().starts_with('{') && val.trim().ends_with('}') {
        Some("obj".to_string())
    } else {
        Some("string".to_string())
    }
}
