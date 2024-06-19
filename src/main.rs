mod lexer;
mod parser;
mod symbol_table;
mod type_checker;

use linefeed::{Interface, ReadResult};

use lexer::Lexer;
use parser::Parser;

fn clarice_eval(input: String) -> String {
    if input.as_str() == "exit" {
        println!("Okay, shutting down the Clarice interactive mode.");
        std::process::exit(0);
    }
    else {
        format!("There is no evaluation! Function definition intentionally left blank.\nYou typed: {}", input)
    }
}

fn clarice_welcome() {
    let cargo_version = std::env::var("CARGO_PKG_VERSION").unwrap();
    println!("Clarice v{}", cargo_version);
    println!("YOU ARE RUNNING THE UNIMPLEMENTED CLARICE INTERACTIVE MODE. THIS IS NOT SUPPOSED TO WORK.");
}

fn interactive() {
    // Welcome
    clarice_welcome();

    // Linefeed
    let interface = Interface::new("Clarice").unwrap();
    let prompt = String::from("Clarice> ");
    interface.set_prompt(&prompt).unwrap();

    loop {
        // Read
        while let ReadResult::Input(line) = interface.read_line().unwrap() {
            let command = line.trim_end().to_string();
            interface.add_history_unique(command.clone());

            // Eval
            let eval_result = clarice_eval(command);
            
            // Print
            println!("{}", eval_result);
        }
        // Loop
        // ...well, it will loop by itself.
    }
}

fn main() {
    let program = "set x to 1 then set y to x then print y";
    let lexer = Lexer::new(program);
    let mut parser = Parser::new(lexer);

    match parser.parse() {
        Ok(program) => {
            println!("Parsed successfully! {:#?}", program);
        }
        Err(e) => {
            println!("Error during parsing: {}", e);
        }
    }
}
