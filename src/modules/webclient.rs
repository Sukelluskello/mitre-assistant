use reqwest;
use serde_json;


#[path = "../../utils/fshandler.rs"]
mod fshandler;
use fshandler::FileHandler;


/// # Mitre-Assistant: WebClient Module
/// This WebClient allows for the access to internet resources.
/// It has several methods that are either blocking or non-blocking
/// when connecting to the internet.
pub struct WebClient {
    pub source_urls: &'static [(&'static str, &'static str)]
}
impl WebClient {
    /// # Mitre-Assistant: Constructor
    /// Instantiates an instance of a webclient.
    /// 
    /// ## Example
    /// ```rust
    /// let _wc = WebClient::new();
    /// ```
    pub fn new() -> WebClient
    {
        WebClient {
            source_urls: &[
                ("enterprise-attack", "https://raw.githubusercontent.com/mitre/cti/master/enterprise-attack/enterprise-attack.json"),
                ("mobile-attack", "https://raw.githubusercontent.com/mitre/cti/master/mobile-attack/mobile-attack.json"),
                ("pre-attack", "https://raw.githubusercontent.com/mitre/cti/master/pre-attack/pre-attack.json")
            ]
        }
    }
    /// # Mitre-Assistant: Load Matrix (Blocking)
    /// This method is a blocking (synchronous) method.
    /// 
    /// ## Example
    /// ```rust
    /// let _wc = WebClient::new();                     // Create a new webclient
    /// 
    /// let _mx = _wc.download("enterprise-attack");   // load the enterprise matrix
    /// ```
    pub fn download(&self, matrix_type: &str)  -> Result<(), Box<dyn std::error::Error>>
    {
        let _url = match matrix_type {
            "enterprise" => self.source_urls[0].1,
            "mobile" => self.source_urls[1].1,
            "pre-attack" => self.source_urls[2].1,
            _ => "None"
        };
        let _json = reqwest::blocking::get(_url)?.text()?;
        if !_json.status.is_success() {
            Ok(Err())
        } else {
            if FileHandler::check_for_config_folder()? {
                let matrix_type = format!("{}.json", matrix_type);
                if FileHandler::write_download(matrix_type.as_str(), &_json)? {
                    // ...here...
                    println!("Downloaded: {}", matrix_type);
                }
            }
        }
    }
}