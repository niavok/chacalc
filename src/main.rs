extern crate rustyline;

fn main() {
    // Head line
    println!("Chacalc calculator. Type type a command, an expression or 'help'.");


    // Prompt while not exiting
    loop {
        let prompt_result = prompt();
        match prompt_result {
            Ok(command_line) => println!("line: {}", command_line),
            Err(_)           => return
        }
    }
}

fn prompt() -> Result<String, ()> {
    let mut rl = rustyline::Editor::<()>::new();
    let readline = rl.readline("> ");
    match readline {
        Ok(line) => return Ok(line),
        Err(_)   => return Err(()),
    }
}
