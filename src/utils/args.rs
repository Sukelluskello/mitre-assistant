use clap::{ App, Arg, ArgMatches, SubCommand };


#[path = "../modules/parser.rs"]
mod parser;
use parser::EnterpriseMatrixParser;


#[path = "../modules/webclient.rs"]
mod webclient;
use webclient::WebClient;




/// # Globals
/// Represent global variables used throughout this source file.
static _VERSION: &str = "v.0.0.1"; 
static _AUTHOR: &str = "carlos diaz | @dfirence\n\n";
static _ABOUT: &str = "Mitre Attack Assistant";


/// # ArgumentsParser
/// This object wraps the cli params into an object for convenience.
/// It provides methods to parse the cli user inputs from here and 
/// keep the main source file clean for developers contributing to `nginja`
/// 
/// # Example
/// ```ignore
/// let _args = ArgumentsParser::new();
/// ```
#[derive(Debug)]
pub struct ArgumentsParser<'a> {
    pub inputs: ArgMatches<'a>
}
impl ArgumentsParser<'_> {
    /// # ArgumentsParser Constructor
    /// Creates a new instance  of a cli arguments object
    pub fn new() -> Self
    {
        ArgumentsParser {
            inputs: App::new("\n\n\nmitre-assistant")
                        .author(_AUTHOR)
                        .version(_VERSION)
                        .about(_ABOUT)
                        .subcommand(
                            SubCommand::with_name("download")
                                       .author(_AUTHOR)
                                       .version(_VERSION)
                                       .about("A more useful utility for the ATT&CK Matrix")
                                       .arg(
                                            Arg::with_name("matrix")
                                                .short("m")
                                                .long("matrix")
                                                .value_name("matrix_name")
                                                .takes_value(true)
                                                .help("Load a Matrix From ATT&CK: (Enterprise|Mobile|Pre-Attatck)")
                                        )
                        )
                        .subcommand(
                            SubCommand::with_name("baseline")
                            .author(_AUTHOR)
                            .version(_VERSION)
                            .about("Parse a Matrix into comprehensive insights")
                            .arg(
                                 Arg::with_name("matrix")
                                     .short("m")
                                     .long("matrix")
                                     .value_name("matrix_name")
                                     .takes_value(true)
                                     .help("Load a Matrix From ATT&CK: (Enterprise|Mobile|Pre-Attatck)")
                             )
                        )
                        .get_matches()
        }
    }
    /// # ArgumentsParser - Parse
    /// This method collects user inputs and should be used as the entry point by the developer
    /// to parse the inputs, execute logic matching the usage of the argument and returning values
    /// to a renderer.
    ///
    /// # Example
    /// ```ignore
    /// let _args = ArgumentsParser::new();
    /// 
    /// let _results = _args.parse();   // Returns results from function invoked
    ///                                 // The function invoked is relevant to the args provided by the user
    /// ```
    pub fn parse(&self) -> Result<(), Box<dyn std::error::Error>>
    {
        if self.inputs.is_present("download") {
            self.download()?;
        } else if self.inputs.is_present("baseline") {
            self.baseline()?;
        }
        Ok(())
    }
    pub fn download(&self) -> Result<(), Box<dyn std::error::Error>>
    {
        let _subcommand = self.inputs.subcommand_matches("download").unwrap();
        let _matrix = match _subcommand.is_present("matrix") {
            true => _subcommand.value_of("matrix").unwrap(),
            false => "None"
        };
        if _matrix != "None" {
            let _wc = WebClient::new();
            let _mx = match _matrix {
                "enterprise" => _wc.download("enterprise")?,
                "pre-attack" => _wc.download("pre-attack")?,
                "mobile" => _wc.download("mobile")?,
                _ => "None".to_string()
            };
        }
        Ok(())
    }
    pub fn baseline(&self) -> Result<(), Box<dyn std::error::Error>>
    {
        let _subcommand = self.inputs.subcommand_matches("baseline").unwrap();
        let _matrix = match _subcommand.is_present("matrix") {
            true => _subcommand.value_of("matrix").unwrap(),
            false => "None"
        };
        if _matrix != "None" {
            let mut _emp = EnterpriseMatrixParser::new();
            _emp.baseline(_matrix)?;
            _emp.save_baseline();
            println!("{}", _emp.to_string());
        }
        Ok(())
    }
}