use std::process::exit;

fn main() {
    let arg_count = std::env::args().len();
    if arg_count != 3 {
        println!("Exactly two arguments required, got '{}'", arg_count - 1);
        exit(exitcode::USAGE);
    }

    let who = std::env::args().nth(1).expect("missing 'who' argument in position 1");
    let lang = std::env::args().nth(2).expect("missing 'lang' argument in position 2");

    match lang.as_ref() {
        "en" => {
            println!("Hello, {}!", who);
        }
        "de" => {
            println!("Hallo, {}!", who);
        }
        "fr" => {
            println!("Bonjour, {}!", who);
        }
        _ => {
            println!("Unknown language '{}'", lang);
            exit(exitcode::USAGE);
        }
    }
}
