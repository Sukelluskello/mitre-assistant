use std::path::{ self, Path };
use std::fs::{ self, File, Metadata };
use std::io::prelude::*;
use std::io::{ self, BufRead, BufReader, BufWriter, Read, Write };
use std::collections::{ HashMap };

// 3rd Party
use fs2::{ self, FileExt };
use walkdir::{ DirEntry, WalkDir };


/// # Custom Errors - Exit Process
/// A convenient method for lightweight usage to exit the program
/// due to a condition.
/// 
/// # Example
/// ```ignore
/// exit_process("Info", "Foo Sucks");      // Logs an informational level
/// exit_process("Warn", "Low Memory");     // Logs a warning level message
/// exit_process("Error", "Foo Failed");    // Logs an error
/// ```
pub fn exit_process(log_level: &str, msg: &str) {
    let _dashes = "-".repeat(msg.len());
    let _user_message = format!(r#"
    (?) {} | Process Exiting Due To:
    {}
    {}
    "#, log_level, _dashes, msg);

    println!("{}", _user_message);
    std::process::exit(0x0100);
}


/// # FileHandler - RepoFileListing
/// This object hold a listing of directory entries by mime extension.
/// 
/// Each extension type is mapped to a Vec<DirEntry>.
/// 
#[derive(Debug)]
pub struct RepoFileListing {
    pub files: HashMap<String, Vec<DirEntry>>
}
/// # FileSystem Handler Utility
/// The filesystem handler utility provides convenience helper functions
/// with the filesystem.  It is intended to have this source file provide
/// a central location to interact with the filesystem.
///
/// # Examples
/// Walk Directories
/// Open files, Write Files, etc.
///
/// # FileHandler Object
/// This object allows for the helper methods concerned with the filesystem.
/// ```ignore
/// let _fh = FileHandler::new();
/// ```
#[derive(Debug)]
pub struct FileHandler {
    pub handle: File,
    pub name: String,
    pub path: String,
    pub meta: Metadata,
    pub size: u64,
}
impl FileHandler {
    /// # FileHandler - Open
    /// Returns a new instance of a custom file object and its handle.
    /// 
    /// When the file is opened for Write Mode, we acquire an exclusive lock
    /// to ensure the sink is likely guaranteed.
    /// 
    /// When the file is opened for Read mode, we inspect for file being already opened,
    /// if it is opened, we duplicate the handle to acquire a shared handle to the file
    /// and not disrupt a foreign application using it.
    /// 
    /// # Example
    /// ```
    /// let _f = FileHandler::open("foo.txt", "r");        // read mode
    ///
    /// let _f = FileHandler::open("foo.txt", "w");        // write mode
    ///
    /// let _f = FileHandler::open("foo.txt", "rw");       // read/write mode
    ///
    /// let _f = FileHandler::open("foo.txt", "cra");      // append mode
    ///
    /// let _f = FileHandler::open("foo.txt", "crt");      // truncate mode
    ///
    /// let _f = FileHandler::open("foo.txt", "crw");      // create new with write mode
    /// ```
    pub fn open(fp: &str, mode: &str) -> Self {
        let _path_string = FileHandler::strip_input(fp);
        let _filepath = Path::new(&_path_string);

        match mode {
            "r" | "rw" | "cra" | "crt" => {
                if _filepath.is_dir() {
                    exit_process("info", "Desired Target is a Folder/Directory. Require a file");
                }
                if !_filepath.exists() {
                    exit_process("info", "Desired Target Does Not Exists.  Require an existent file");
                }
            }
            "crw" => {
                println!("\t|__(+) New File To Be Created: {}", fp);
            }
            _ => exit_process("info", "Desired File Mode Not Suppported, Process Exiting..."),
        }

        let mut _read = false;
        let mut _write = false;
        let mut _create = false;
        let mut _append = false;
        let mut _truncate = false;

        match mode {
            "r" => {
                _read = true;
            },
            "rw" => {
                _read = true;
                _write = true;
            },
            "crw" => {
                _write = true;
                _create = true;
            },
            "cra" => {
                _write = true;
                _append = true;
            },
            "crt" => {
                _write = true;
                _truncate = true;
            },
            _ => exit_process("Info", "Desired File Mode Not Suppported, Process Exiting..."),
        }

        let _file = fs::OpenOptions::new()
                                    .read(_read)
                                    .write(_write)
                                    .create(_create)
                                    .append(_append)
                                    .write(_write)
                                    .open(_filepath)
                                    .unwrap();

        let _name = _filepath.file_name().unwrap();
        let _name = _name.to_str().unwrap();
        let _name = String::from(_name);
        let _meta = _filepath.metadata().unwrap();
        let _size = _meta.len();

        FileHandler {
            handle: _file,
            name: _name,
            path: _filepath.display().to_string(),
            meta: _meta,
            size: _size,
        }
    }
    /// # FileHandler - Recurse
    /// Walk the angular project directory and indexes its contents for further processing.
    /// This creates an initial data structure that summarizes the profile of the project
    /// and is used by analysis methods to provide the details of each category.
    /// 
    /// ```ignore
    /// let _dir_listing = FileHandler::recurse("angular/foo"); // at the root of the project
    /// ```
    /*
    pub fn recurse(dp: &str) -> Result<RepoFileListing, Box<dyn std::error::Error>>
    {
        let _path_string = FileHandler::strip_input(dp);
        let _dirpath = Path::new(&_path_string);
        if _dirpath.is_file() {
            exit_process("Info", "User Error:  Requires a Project Folder Path, not a file");
        }
        if !_dirpath.exists() {
            exit_process("Info", "User Error:  Desired Path Does Not Exist");
        }
        
        let mut _repo: HashMap<String, Vec<DirEntry>> = HashMap::new();
        let _mimes: [&str; 3] = [ ".ps1", ".psd1", ".psm1"];

        for _mime in _mimes.iter() {
            let mut _entries = Vec::new();
            for _entry in WalkDir::new(&_path_string).max_depth(50).into_iter().filter_map(|e| e.ok()) {
                let _e = _entry.path().to_str().unwrap();
                if _e.contains("node_modules") || _e.contains("spec.ts") {
                    continue
                } else {
                    if _e.ends_with(_mime) { _entries.push(_entry); }
                }
            }
            _repo.insert(_mime.to_string(), _entries);
        }
        let _repo_listing = RepoFileListing { files: _repo };
        Ok(_repo_listing)
    }
    */
    /// # FileHandler - Strip Input (Private Method)
    /// This method performs simple char replacement of input strings that have "\r", "\r\n", or "\n"
    /// characters when provided to the program.  For file_paths, this is very important to protect against
    /// to ensure program crashes are avoided and inputs are normalized as much as possible.
    /// 
    /// # Example
    /// ```
    /// let _input = FileHandler::strip_input("foo\r");
    /// assert_eq!("foo", _input);
    /// ```
    fn strip_input(input: &str) -> String
    {
        let mut _s = String::from(input);

        if input.ends_with(r"\r\n") {       // inspect string and strip trailing chars
            _s = _s.replace("\r\n", "");
        }

        if input.ends_with(r"\r") {
            _s = _s.replace(r"\r", "");
        }

        if input.ends_with(r"\n") {
            _s = _s.replace(r"\n", "");
        }
        _s
    }
     /// # FileHandler Write Method
     /// 
     /// This method writes content as bytes to the file whose previous call to open
     /// produced a mutable handle to the file.
     ///```
     /// let mut _f = FileHandler::open("foo.txt", "crw");
     ///
     /// let _s = String::from("baz");
     ///
     /// _f.write(&_s)?;
     /// ```
     pub fn write(&mut self, _content: &String) -> Result<(), Box<dyn std::error::Error>>
     {
        self.handle.lock_exclusive()?;
        self.handle.write_all(_content.as_bytes())?;
        self.handle.flush()?;
        self.handle.unlock()?;
        Ok(())
     }
     /// # FileHandler - ReadAsVecBytes
     /// This method allows reading into a buffer made of a vector of bytes.
     /// Note this method should be used to read a file into memory as it reads the
     /// entire content of the file into a Vec<u8>.
     /// ```
     /// let _f = FileHandler::open("foo.exe", "r");
     ///
     ///     _f.read_as_vecbytes()?;
     ///
     ///     println!("{:#?}", f.content);
     /// ```
     pub fn read_as_vecbytes(&self, n_bytes: u64) -> Result<Vec<u8>, Box<dyn std::error::Error>>
     {
        let mut _bytes: Vec<u8> = Vec::with_capacity(n_bytes as usize);
        let mut _bufr = BufReader::new(&self.handle);
                _bufr.read_to_end(&mut _bytes)?;
        Ok(_bytes)
     }
     pub fn read_as_bytesarray(&self, n_bytes: &mut [u8]) -> Result<(), Box<dyn std::error::Error>>
     {
        let mut _bufr = BufReader::new(&self.handle);
                _bufr.read_exact(n_bytes)?;
        Ok(())
     }
     pub fn check_for_config_folder() -> Result<bool, Box<dyn std::error::Error>>
     {
         let _home = std::env::home_dir().unwrap();
         let _home = format!("{}/{}", _home.display().to_string(), ".mitre-assistant");
         let _home = Path::new(_home.as_str());
         match _home.exists() {
             true => Ok(true),
             false => { std::fs::create_dir(_home); Ok(true) }
         }
     }
     pub fn write_download(filename: &str, content: &String) -> Result<bool, Box<dyn std::error::Error>>
     {
        let _home = std::env::home_dir().unwrap():
        let _home = format!("{}/{}/{}", _home.display().to_string(), ".mitre-assistant", "matrixes");
        let _path = Path::new(_home.as_str());
        let _dst  = match _home.exists() {
            true => true,
            false => { std::fs::create_dir(_home); true }
        };
        // Now write the file
        let _ofile = format!("{}/{}", _home, filename);
        let _path = Path::new(_ofile.as_str());
        let _dst = match _path.exists() {
            true => true,
            false => {
                let mut _f = FileHandler::open(_ofile, "crw");
                _f.write(content);
                true
            }
        }
        Ok(_dst)
     }
}