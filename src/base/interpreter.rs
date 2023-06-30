use std::{
    io::{BufRead, BufReader, Write},
    process::{Child, ChildStderr, ChildStdout, Command, Stdio},
    sync::{
        mpsc::{channel, Sender},
        Mutex,
    },
    thread,
};

use super::InterpreterError;

pub struct Interpreter {
    interpreter: Child,
    input: Sender<String>,
    output: BufReader<ChildStdout>,
    error: BufReader<ChildStderr>,
    alive: bool,
    lock: Mutex<()>,
}

impl Interpreter {
    pub fn start() -> Interpreter {
        let mut wish = Command::new("wish8.6")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start interpreter");

        let mut wish_stdin = wish.stdin.take().expect("Failed to start interpreter");
        let wish_stdout = wish.stdout.take().expect("Failed to start interpreter");
        let wish_stderr = wish.stderr.take().expect("Failed to start interpreter");

        let (in_to_wish, out_from_wish) = channel::<String>();

        thread::Builder::spawn(thread::Builder::new(), move || loop {
            let got_command = out_from_wish.recv();
            if let Ok(cmd) = got_command {
                if let Err(_) = wish_stdin.write_all((cmd + "\n").as_bytes()) {
                    drop(out_from_wish);
                    break;
                }
            }
        })
        .expect("Failed to start interpreter.");

        Interpreter {
            interpreter: wish,
            input: in_to_wish,
            output: BufReader::new(wish_stdout),
            error: BufReader::new(wish_stderr),
            alive: true,
            lock: Mutex::new(()),
        }
    }

    pub fn run_command<S: Into<String>>(&mut self, msg: S) -> Result<(), InterpreterError> {
        let _guard = self
            .lock
            .lock()
            .expect("Should always be able to lock mutex");
        if self.alive {
            // Send the msg to the interpreter
            self.input.send(msg.into())?;
            // Put a newline out on stderr so there is always something to
            // read with read_line()
            self.input.send(r#"puts stderr "\n""#.to_string())?;

            // Read a line from the stderr into a string
            let mut error_buffer = String::new();
            self.error.read_line(&mut error_buffer)?;
            // If there was no error, the buffer contains just the newline ...
            error_buffer = error_buffer.trim_end().to_string();
            // ... and will be empty once trimmed
            if !error_buffer.is_empty() {
                return Err(InterpreterError::MalformedCommand);
            } else {
                // If the buffer was empty, we need to make sure to read another
                // line to discard the newline we put there
                // In essence clearing the buffer indirectly
                self.error.read_line(&mut error_buffer)?;
            }
            return Ok(());
        }
        Err(InterpreterError::DeadInterpreter)
    }

    pub fn run_command_with_response<S: Into<String>>(
        &mut self,
        qst: S,
    ) -> Result<String, InterpreterError> {
        let force_output_cmd = format!("puts [{}]; flush stdout", qst.into());
        self.run_command(force_output_cmd)?;
        let _guard = self
            .lock
            .lock()
            .expect("Should always be able to lock mutex");
        if self.alive {
            let mut output_buffer = String::new();
            self.output.read_line(&mut output_buffer)?;
            output_buffer = output_buffer.trim_end().to_string();
            return Ok(output_buffer);
        }
        Err(InterpreterError::DeadInterpreter)
    }
}

impl Drop for Interpreter {
    fn drop(&mut self) {
        // If this is an Err(), then the interpreter has already been killed which is fine
        self.alive = false;
        if let Err(_) = self.interpreter.kill() {
            // If the interpreter is already dead then that's fine
        }
    }
}
