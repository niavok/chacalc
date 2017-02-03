extern crate termios;
extern crate rustyline;
use std::io;
use std::io::Read;
use std::io::Write;
//use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};

fn main() {
    println!("Chacalc calculator. Type type a command, an expression or 'help'.");
    

    loop {
        let prompt_result = prompt();
        match prompt_result {
            Ok(command_line) => println!("line: {}", command_line),
            Err(_)           => return
        }
    }

    /*loop {
        print!("> ");
        io::stdout().flush().ok().expect("Could not flush stdout");
        
        io::stdin().read_line(&mut command_line).expect("Failed to read line");
        println!("ok: {}", command_line);
    }*/
}
/*
fn prompt() -> String {
    let stdin = 0; // couldn't get std::os::unix::io::FromRawFd to work 
                   // on /dev/stdin or /dev/tty
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();  // make a mutable copy of termios 
                                            // that we will modify
    new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0;1];  // read exactly one byte

    // display the prompt
    print!("> ");
    stdout.lock().flush().unwrap();

    
    reader.read_exact(&mut buffer).unwrap();
    //println!("You have hit: {:?}", buffer);
    


    tcsetattr(stdin, TCSANOW, & termios).unwrap();  // reset the stdin to 
                                                    // original termios data

    String::new()
}*/

fn prompt() -> Result<String, ()> {
    let mut rl = rustyline::Editor::<()>::new();
    let readline = rl.readline("> ");
    match readline {
        Ok(line) => return Ok(line),
        Err(_)   => return Err(()),
    }
}
