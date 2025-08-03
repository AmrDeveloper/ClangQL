use gitql_cli::printer::OutputFormatKind;

/// Arguments for ClangQL
#[derive(Debug, PartialEq)]
pub struct Arguments {
    pub files: Vec<String>,
    pub analysis: bool,
    pub pagination: bool,
    pub page_size: usize,
    pub output_format: OutputFormatKind,
}

/// Create a new instance of Arguments with the default settings
impl Arguments {
    fn new() -> Arguments {
        Arguments {
            files: vec![],
            analysis: false,
            pagination: false,
            page_size: 10,
            output_format: OutputFormatKind::Table,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Command {
    ReplMode(Arguments),
    QueryMode(String, Arguments),
    ScriptMode(String, Arguments),
    Help,
    Version,
    Error(String),
}

pub fn parse_arguments(args: &[String]) -> Command {
    let args_len = args.len();

    if args.iter().any(|i| i == "--help" || i == "-h") {
        return Command::Help;
    }

    if args.iter().any(|i| i == "--version" || i == "-v") {
        return Command::Version;
    }

    let mut optional_query: Option<String> = None;
    let mut optional_script_file: Option<String> = None;
    let mut arguments = Arguments::new();

    let mut arg_index = 1;
    loop {
        if arg_index >= args_len {
            break;
        }

        let arg = &args[arg_index];

        if !arg.starts_with('-') {
            return Command::Error(format!("Unknown argument {arg}"));
        }

        match arg.as_ref() {
            "--files" | "-f" => {
                arg_index += 1;
                if arg_index >= args_len {
                    let message = format!("Argument {arg} must be followed by one or more path");
                    return Command::Error(message);
                }

                loop {
                    if arg_index >= args_len {
                        break;
                    }

                    let files = &args[arg_index];
                    if !files.starts_with('-') {
                        arguments.files.push(files.to_string());
                        arg_index += 1;
                        continue;
                    }

                    break;
                }

                if arguments.files.is_empty() {
                    return Command::Error("Must provide one or more C/C++ files".to_string());
                }
            }
            "--query" | "-q" => {
                arg_index += 1;
                if arg_index >= args_len {
                    let message = format!("Argument {arg} must be followed by the query");
                    return Command::Error(message);
                }

                optional_query = Some(args[arg_index].to_string());
                arg_index += 1;
            }
            "--script" | "-s" => {
                arg_index += 1;
                if arg_index >= args_len {
                    let message = format!("Argument {arg} must be followed by the file");
                    return Command::Error(message);
                }

                optional_script_file = Some(args[arg_index].to_string());
                arg_index += 1;
            }
            "--analysis" | "-a" => {
                arguments.analysis = true;
                arg_index += 1;
            }
            "--pagination" | "-p" => {
                arguments.pagination = true;
                arg_index += 1;
            }
            "--pagesize" | "-ps" => {
                arg_index += 1;
                if arg_index >= args_len {
                    let message = format!("Argument {arg} must be followed by the page size");
                    return Command::Error(message);
                }

                let page_size_result = args[arg_index].parse::<usize>();
                if page_size_result.is_err() {
                    return Command::Error("Invalid page size".to_string());
                }

                let page_size = page_size_result.ok().unwrap();
                arguments.page_size = page_size;
                arg_index += 1;
            }
            "--output" | "-o" => {
                arg_index += 1;
                if arg_index >= args_len {
                    let message = format!("Argument {arg} must be followed by output format");
                    return Command::Error(message);
                }

                let output_type = &args[arg_index].to_lowercase();
                if output_type == "csv" {
                    arguments.output_format = OutputFormatKind::CSV;
                } else if output_type == "json" {
                    arguments.output_format = OutputFormatKind::JSON;
                } else if output_type == "render" || output_type == "table" {
                    arguments.output_format = OutputFormatKind::Table;
                } else if output_type == "yaml" {
                    arguments.output_format = OutputFormatKind::YAML;
                } else {
                    return Command::Error("Invalid output format".to_string());
                }
                arg_index += 1;
            }
            _ => return Command::Error(format!("Unknown command {arg}")),
        }
    }

    if arguments.files.is_empty() {
        return Command::Error("Must provide one or more C/C++ files".to_string());
    }

    if let Some(script_file) = optional_script_file {
        Command::ScriptMode(script_file, arguments)
    } else if let Some(query) = optional_query {
        Command::QueryMode(query, arguments)
    } else {
        Command::ReplMode(arguments)
    }
}

pub fn print_help_list() {
    println!("ClangQL is a SQL like query language to run on local files");
    println!();
    println!("Usage: ClangQL [OPTIONS]");
    println!();
    println!("Options:");
    println!("-f,  --files <paths>        Path for local files to run query on");
    println!("-s,  --script <file>        Script file contains one or more query");
    println!("-q,  --query <GQL Query>    ClangQL query to run on selected repositories");
    println!("-p,  --pagination           Enable print result with pagination");
    println!("-ps, --pagesize             Set pagination page size [default: 10]");
    println!("-o,  --output               Set output format [render, json, csv]");
    println!("-a,  --analysis             Print Query analysis");
    println!("-h,  --help                 Print ClangQL help");
    println!("-v,  --version              Print ClangQL Current Version");
}
