use std::env;
use std::io;
use std::io::Write;

// http://craftinginterpreters.com/
// lox language, but in rust
// rox? like Sisyphus with the rock

fn main() {
    println!("Hello, world!");

    // Read file from path
    let arg_count = env::args().len();
    println!("args# {}", arg_count);
    if arg_count > 1 {
        for (i, argument) in env::args().enumerate() {
            if i == 0 {
                continue; // skip first
            }
            println!("{i}\t{argument}");
            let source = argument;

            let result = run_file(source);
            println!("result {:?}", result);
        }
        return;
    } else {
        // or drop into prompt
        match run_prompt() {
            Ok(_) => {}
            Err(e) => {
                println!("Error running prompt\n{:?}", e);
                return;
            }
        }
    }
}

// 1. runFile(path) Read a file and run it
fn run_file(path: String) -> Result<(), ()> {
    println!("run_file {}", path);

    // if file doesn't exist, panic
    let warning = format!("File Not Found {}", path);
    let contents = std::fs::read_to_string(path).expect(warning.as_str());

    run(contents)
}

// 2. runPrompt() Interactively ask for input
fn run_prompt() -> Result<(), ()> {
    let prompt = "> ";

    let mut buffer = String::new();
    let stdin = io::stdin();

    loop {
        // prompt
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        // read line, handle error
        match stdin.read_line(&mut buffer) {
            Ok(_) => {}
            Err(_) => {
                println!("Error reading line");
                return Err(());
            }
        }

        // user done?
        let trimmed = buffer.trim();
        if trimmed == "" || trimmed == "quit" || trimmed == "exit" {
            break;
        }

        // TODO: do something with the line

        // print and reset buffer
        println!("[user] {}", buffer);
        buffer.clear();
    }

    Ok(())
}

// 1.1 run(source) Run the contents of a file
fn run(source: String) -> Result<(), ()> {
    println!("run {}", source);

    Ok(())
}
