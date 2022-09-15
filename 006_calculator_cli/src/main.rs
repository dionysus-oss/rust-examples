use clap::Parser;

#[derive(Parser)]
#[clap(author, version)]
struct Args {
    #[clap(short, long, value_parser)]
    op: String,

    #[clap(value_parser)]
    first: f32,

    #[clap(value_parser)]
    second: f32,
}

fn main() {
    let cli = Args::parse();

    let result = match cli.op.as_ref() {
        "+" | "add" | "plus" => cli.first + cli.second,
        "-" | "sub" | "subtract" => cli.first - cli.second,
        "*" | "mult" | "multiply" => cli.first * cli.second,
        "/" | "div" | "divide" => cli.first / cli.second,
        _ => panic!("unknown operator - {}", cli.op)
    };

    println!("{}", result);
}
