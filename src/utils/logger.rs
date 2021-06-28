use std::fs::File;
use std::io::{Error, Write};
use std::sync::Mutex;
use std::thread::current;

#[derive(Debug)]
pub struct Logger {
    file: Mutex<std::fs::File>,
}

impl Logger {
    pub fn new(filename: &str) -> Result<Self, Error> {
        let log_f = File::create(filename)?;
        Ok(Logger {
            file: Mutex::new(log_f),
        })
    }
    fn write(&self, message: String) -> Result<(), Error> {
        let message = message + "\n";
        match self.file.lock() {
            Ok(mut file) => {
                (*file).write_all(message.as_bytes())?;
            }
            Err(e) => {
                // Si el lock está envenenado hacemos panic!()
                panic!("{}", e)
            }
        };
        Ok(())
    }

    pub fn info(&self, msg: String) {
        let me = current();
        match self.write(format!(
            "[INFO][{}] - {}",
            me.name().get_or_insert("??"),
            msg
        )) {
            Ok(_) => {}
            Err(_) => {
                println!("Unable to write to logging file. Logging messages won't be saved.");
            }
        }
    }
}
