use crate::settings::Verbosity;

pub trait Logger{
    fn error(&self, message: &str);
    fn warning(&self, message: &str);
    fn debug(&self, message: &str);
}

pub struct UserLogger {
    verbosity: Verbosity,
}

impl UserLogger {
    pub fn with_verbosity(verbosity: &Verbosity) -> UserLogger {
        UserLogger { verbosity: verbosity.clone() }
    }
}

impl Logger for UserLogger {
    fn error(&self, message: &str) {
        match self.verbosity {
            Verbosity::Error | Verbosity::Warning | Verbosity::Debug => {
                eprint!("Error: ");
                eprintln!("{}", message);
            }
        }
    }

    fn warning(&self, message: &str) {
        match self.verbosity {
            Verbosity::Warning | Verbosity::Debug => {
                print!("Warning: ");
                println!("{}", message);
            }
            Verbosity::Error => {}
        }
    }

    fn debug(&self, message: &str) {
        match self.verbosity {
            Verbosity::Debug => {
                println!("{}", message);
            }
            Verbosity::Warning | Verbosity::Error => {}
        }
    }
}
