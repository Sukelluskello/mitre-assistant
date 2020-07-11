use std::thread;
use std::sync::mpsc::channel;
use std::collections::HashSet;
//use std::borrow::Cow;
//use std::sync::{Arc};


#[path = "./webclient.rs"]
mod webclient;
use webclient::WebClient;


#[path = "../utils/fshandler.rs"]
mod fshandler;
use fshandler::FileHandler;


pub struct EnterpriseMatrixParser {
    pub techniques:     HashSet<(&'static str, &'static str)>,
    pub tactics:        HashSet<&'static str>,
    pub platforms:      HashSet<&'static str>,
}
impl EnterpriseMatrixParser {
    pub fn new() -> EnterpriseMatrixParser
    {
        EnterpriseMatrixParser {
            techniques: HashSet::new(),
            tactics:    HashSet::new(),
            platforms:  HashSet::new()
        }
    }
    pub fn load(&self)
    {
        let _wc = WebClient::new();
        if FileHandler::check_for_config_folder().unwrap() {
            //let _json = _wc.load("enterprise").unwrap();
            //pick up from here
        }
    }
}