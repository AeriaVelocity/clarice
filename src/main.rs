mod lexer;
mod parser;
mod symbol_table;
mod type_checker;
mod interpreter;

use linefeed::{Interface, ReadResult};

use lexer::Lexer;
use parser::Parser;
use interpreter::Environment;

fn clarice_eval(input: String) -> String {
    match input.as_str() {
        "exit" => {
            println!("Okay, shutting down the Clarice interactive mode.");
            std::process::exit(0);
        }
        "help" => {
            println!("You can enter Clarice commands into the interactive prompt.");
            println!("Type 'exit' to exit interactive mode.");
            return "=> help".to_string();
        }
        _ => (),
    }
    let lexer = Lexer::new(&input);
    let mut parser = Parser::new(lexer);

    let parsed_program = match parser.parse() {
        Ok(program) => program,
        Err(e) => {
            return format!("Error during parsing: {}", e);
        }
    };

    let mut environment = Environment::new();

    environment.interpret(parsed_program);

    // This is where I'd put the return value. IF I HAD ONE
    format!("=> {}", input)
}

fn clarice_welcome() {
    let cargo_version = std::env::var("CARGO_PKG_VERSION").unwrap();
    println!("Clarice v{}", cargo_version);
    println!("You are running the Clarice interactive mode. Please note Clarice is not yet fully functional.");
    println!("Type `help` for help or `exit` to leave interactive mode.");
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
    interactive();
}

