extern crate rustyline;
use rustyline::line_buffer::LineBuffer;

struct ChacalcCompleter {

}


fn main() {
    // Head line
    println!("Chacalc calculator. Type type a command, an expression or 'help'.");

    prompt();

}

fn prompt() {
    let mut rl = rustyline::Editor::<()>::new();

    let completer : ChacalcCompleter;

    rl.set_completer(Some(completer));
    // Prompt while not exiting
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(command_line) => return println!("line: {}", command_line),
            Err(_)   => break,
        }
    }
}




impl rustyline::completion::Completer for ChacalcCompleter {
    fn complete(&self, line: &str, pos: usize) -> rustyline::Result<(usize, Vec<String>)> {
    }

    fn update(&self, line: &mut LineBuffer, start: usize, elected: &str) {
    }
}

