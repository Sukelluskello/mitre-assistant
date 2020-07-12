use std::thread;
use std::sync::{Arc};
use std::sync::mpsc;
use std::collections::HashSet;
//use std::borrow::Cow;


use serde_json;


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
    pub fn baseline(&self, matrix_type: &str) -> Result<(), Box<dyn std::error::Error>>
    {
        if FileHandler::check_for_config_folder().unwrap() {
            match matrix_type {
                "enterprise" => self.baseline_enterprise()?,
                _ => ()
            }
        }
        Ok(())
    }
    fn baseline_enterprise(&self) -> Result<(), Box<dyn std::error::Error>>
    {
        let _bufr = FileHandler::load_resource("matrixes", "enterprise.json");
        let _json: serde_json::Value = serde_json::from_reader(_bufr).unwrap();
        //println!("{:?}", _json);
        // Setup Safe Threading Infrastructure
        // step 1 - Arc::new() SmartPointer againts JSON - 10MBs on the heap
        let _arc = Arc::new(_json);
        // step 2 - mps::channel() Multiple Produce Single Consumer Channel
        let (_tx, _rx) = mpsc::channel();
        // step 3 - storage of thread handles
        //let mut _thandles = vec![];
        // step 4 - Setup threads and cloned arcs
        // Start with the thread that gets all techniques
        let _a1 = Arc::clone(&_arc);
        let _t1 = _tx.clone();
        let _th = thread::spawn(move || {
            let mut _v_techniques: Vec<String> = vec![];
            for _item in _a1["objects"].as_array().unwrap().iter() {
                if _item["type"].as_str() == Some("attack-pattern") {
                    _v_techniques.push(serde_json::to_string(_item).unwrap());
                }
            }
            println!("Thread Worker: Techniques - Done | {} Items", _v_techniques.len());
            _t1.send(_v_techniques).unwrap();

        });
        let _values = _rx.recv().unwrap();
        println!("{:#?}", _values);
        /*
        _thandles.push(_th);
        // Next the thread with all the tactics
        let _a2 = Arc::clone(&_arc);
        let _th = std::thread::spawn(move || {
            
        });
        _thandles.push(_th);
        // Next the thread with all the platforms
        let _a3 = Arc::clone(&_arc);
        let _th = std::thread::spawn(move || {
            
        });
        _thandles.push(_th);
        
        // Step 4 - Signal Main Thread to Wait For Joins
        for _handle in _thandles {
            _handle.join().unwrap();
        }
        */
        Ok(())
    }
}