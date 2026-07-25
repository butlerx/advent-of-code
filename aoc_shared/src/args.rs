use std::env;
use std::process;

pub struct Args {
    pub day: u32,
    pub part: Option<u32>,
}

enum Token {
    Day(u32),
    Part(u32),
    Help,
}

impl Args {
    #[must_use]
    pub fn parse(program_name: &str) -> Self {
        let tokens: Vec<Token> = env::args()
            .skip(1)
            .collect::<Vec<_>>()
            .chunk_by(|a, _| a == "--part" || a == "-p")
            .flat_map(|chunk| Self::parse_chunk(chunk, program_name))
            .collect();

        if tokens.iter().any(|t| matches!(t, Token::Help)) {
            Self::print_usage(program_name);
            process::exit(0);
        }

        let part = tokens.iter().find_map(|t| match t {
            Token::Part(p) => Some(*p),
            _ => None,
        });

        let day = tokens
            .iter()
            .find_map(|t| match t {
                Token::Day(d) => Some(*d),
                _ => None,
            })
            .unwrap_or_else(|| {
                eprintln!("Error: missing required argument <day>");
                Self::print_usage(program_name);
                process::exit(1);
            });

        Self { day, part }
    }

    fn parse_chunk(chunk: &[String], program_name: &str) -> Vec<Token> {
        match chunk {
            [flag, val] if flag == "--part" || flag == "-p" => {
                let p = val.parse::<u32>().unwrap_or_else(|_| {
                    eprintln!("Error: --part must be 1 or 2");
                    process::exit(1);
                });
                if p != 1 && p != 2 {
                    eprintln!("Error: --part must be 1 or 2");
                    process::exit(1);
                }
                vec![Token::Part(p)]
            }
            [flag] if flag == "--part" || flag == "-p" => {
                eprintln!("Error: --part requires a value (1 or 2)");
                process::exit(1);
            }
            _ => chunk
                .iter()
                .map(|a| Self::parse_positional(a, program_name))
                .collect(),
        }
    }

    fn parse_positional(arg: &str, program_name: &str) -> Token {
        match arg {
            "--help" | "-h" => Token::Help,
            _ => Token::Day(arg.parse::<u32>().unwrap_or_else(|_| {
                eprintln!("Error: day must be a number, got '{arg}'");
                Self::print_usage(program_name);
                process::exit(1);
            })),
        }
    }

    fn print_usage(program_name: &str) {
        eprintln!("Usage: {program_name} <day> [--part <1|2>]");
        eprintln!();
        eprintln!("Arguments:");
        eprintln!("  <day>            Day/mission number to run");
        eprintln!();
        eprintln!("Options:");
        eprintln!("  -p, --part <N>   Run only part 1 or 2");
        eprintln!("  -h, --help       Print this help");
    }
}
