pub enum CliAction {
    ShowHelp,
    ShowVersion,
    RunFile(String),
    ShowTokens(String),
    ShowAst(String),
    SyntaxCheck(String),
    Invalid(String),
}

pub fn parse_args(args: &[String]) -> CliAction {
    if args.len() < 2 {
        return CliAction::ShowHelp;
    }

    match args[1].as_str() {
        "-v" | "--version" => CliAction::ShowVersion,
        "-h" | "--help" => CliAction::ShowHelp,
        "--token" if args.len() > 2 => CliAction::ShowTokens(args[2].clone()),
        "--ast"   if args.len() > 2 => CliAction::ShowAst(args[2].clone()),
        "--check" if args.len() > 2 => CliAction::SyntaxCheck(args[2].clone()),
        s if s.ends_with(".fx")     => CliAction::RunFile(s.to_string()),
        other => CliAction::Invalid(other.to_string()),
    }
}

pub fn show_help() {
    println!("FLYUX - Ultra minimal language runtime");
    println!("Usage:");
    println!("  flyux [options] <file.fx>");
    println!();
    println!("Options:");
    println!("  -v, --version       Show version");
    println!("  -h, --help          Show this help message");
    println!("  --token <file.fx>   Print token stream");
    println!("  --ast <file.fx>     Print abstract syntax tree");
    println!("  --check <file.fx>   Check syntax only");
}