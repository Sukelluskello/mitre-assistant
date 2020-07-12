use std::fmt::{self, Formatter, Display, Debug};

pub struct CustomPathError;

impl fmt::Display for CustomPathError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        write!(f, "Custom Path Error: Cannot Find Resource in Config Folder")
    }
}

impl fmt::Debug for CustomPathError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}