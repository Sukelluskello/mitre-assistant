use std::thread;
use std::sync::{Arc};
use std::sync::mpsc;
use std::collections::HashSet;
//use std::borrow::Cow;


use serde_json;
use serde_derive::{Deserialize, Serialize};


#[path = "./webclient.rs"]
mod webclient;
use webclient::WebClient;


#[path = "../utils/fshandler.rs"]
mod fshandler;
use fshandler::FileHandler;


#[path = "../utils/regexes.rs"]
mod regexes;
use regexes::RegexPatternManager;


#[derive(Debug,Deserialize, Serialize)]
pub struct EnterpriseMatrixStatistics {
    pub count_revoked_techniques:   usize,
    pub count_active_techniques:    usize,
    pub count_active_subtechniques: usize,
    pub count_malware_records:      usize,
    pub count_adversary_records:    usize,
    pub count_tools_records:        usize,
    pub count_platforms_records:    usize,
    pub count_tactics_records:      usize,
    pub count_datasources_records:  usize
}
impl EnterpriseMatrixStatistics {
    pub fn new() -> Self
    {
        EnterpriseMatrixStatistics {
            count_revoked_techniques:   0,
            count_active_techniques:    0,
            count_active_subtechniques: 0,
            count_malware_records:      0,
            count_adversary_records:    0,
            count_tools_records:        0,
            count_platforms_records:    0,
            count_tactics_records:      0,
            count_datasources_records:  0
        }
    }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct EnterpriseMatrixParser {
    pub techniques:         HashSet<(String, String, String)>,
    pub tactics:            HashSet<String>,
    pub subtechniques:      HashSet<(String, String, String)>,
    pub platforms:          HashSet<String>,
    pub datasources:        HashSet<String>,
    pub revoked_techniques: HashSet<(String, String)>,
    pub stats:              EnterpriseMatrixStatistics
}
impl EnterpriseMatrixParser {
    pub fn new() -> EnterpriseMatrixParser
    {
        EnterpriseMatrixParser {
            techniques:     HashSet::new(),
            tactics:        HashSet::new(),
            subtechniques:  HashSet::new(),
            platforms:      HashSet::new(),
            datasources:    HashSet::new(),
            revoked_techniques: HashSet::new(),
            stats:      EnterpriseMatrixStatistics::new()
        }
    }
    pub fn baseline(&mut self, matrix_type: &str) -> Result<(), Box<dyn std::error::Error>>
    {
        if FileHandler::check_for_config_folder().unwrap() {
            match matrix_type {
                "enterprise" => self.baseline_enterprise()?,
                _ => ()
            }
        }
        Ok(())
    }
    fn baseline_enterprise(&mut self) -> Result<(), Box<dyn std::error::Error>>
    {
        let _bufr = FileHandler::load_resource("matrixes", "enterprise.json");
        let _json: serde_json::Value = serde_json::from_reader(_bufr).unwrap();
        let _scanner = RegexPatternManager::load_subtechnique();
        // Iterate through JSON Objects
        for _t in _json["objects"].as_array().unwrap().iter() {
            let _s = _t["type"].as_str().unwrap();
            let _x = serde_json::to_string(_t).unwrap();
            //
            //
            if _s == "attack-pattern" && _x.contains("revoked") {
                self.extract_revoked_techniques(_t);
            } else if _s == "attack-pattern" && !_x.contains("revoked")  {
                if _scanner.pattern.is_match(&_x) {
                    self.extract_techniques_and_tactics(_t, true);
                } else {
                    self.extract_techniques_and_tactics(_t, false);
                }
                self.extract_tactics_killchain(_t);
                if _x.contains("x_mitre_data_sources") {
                    self.extract_datasources(_t);
                } else if _x.contains("x_mitre_platforms") {
                    self.extract_platforms(_t);
                }
            } else if _s == "malware" {
                self.stats.count_malware_records += 1;
            } else if _s == "intrusion-set" {
                self.stats.count_adversary_records += 1;
            } else if _s == "tool" {
                self.stats.count_tools_records += 1;
            }
        }
        /*
            identity
            intrusion-set
            malware
            marking-definition
            relationship
            Revoked Techniques: 0
            tool
            x-mitre-matrix
            x-mitre-tactic
        */
        //self.stats = _ems;
        //println!("\n{:?}", _ems);
        Ok(())
    }
    fn extract_revoked_techniques(&mut self, items: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>>
    {
        let _tid = items["external_references"].as_array().expect("Problem With External References");
        let _tid = _tid[0]["external_id"].as_str().expect("Problem With External ID");
        let _tname = items["name"].as_str().expect("Problem With Technique Name");
        self.revoked_techniques.insert((_tid.to_string(), _tname.to_string()));
        self.stats.count_revoked_techniques = self.revoked_techniques.len();
        Ok(())
    }
    fn extract_datasources(&mut self, items: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>>
    {
        //println!("{:?}", items["x_mitre_data_sources"]);
        for _item in items["x_mitre_data_sources"].as_array().unwrap().iter() {
            self.datasources.insert(_item.as_str().unwrap().to_string());
        }
        self.stats.count_datasources_records = self.datasources.len();
        Ok(())
    }
    fn extract_platforms(&mut self, items: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>>
    {
        //println!("{:?}", items["x_mitre_data_sources"]);
        for _item in items["x_mitre_platforms"].as_array().unwrap().iter() {
            self.platforms.insert(_item.as_str().unwrap().to_string());
        }
        self.stats.count_platforms_records  = self.platforms.len();
        Ok(())
    }
    fn extract_tactics_killchain(&mut self, items: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>>
    {
        for _item in items["kill_chain_phases"].as_array().unwrap().iter() {
            self.tactics.insert(_item["phase_name"].as_str().unwrap().to_string());
        }
        self.stats.count_tactics_records = self.tactics.len();
        Ok(())
    }
    fn extract_techniques_and_tactics(&mut self, items: &serde_json::Value, is_subtechnique: bool)
        -> Result<(), Box<dyn std::error::Error>>
    {
        //println!("{:?}", items);
        let _msg: &str = "(?) Enterprise Parser Error: Unable to Extract Techniques & Tactics";
        // Obtain the technique id
        let _tid = items["external_references"].as_array().expect("Problem With External References");
        let _tid = _tid[0]["external_id"].as_str().expect("Problem With External ID");
        // Obtain the technique name
        let _tname = items["name"].as_str().expect("Problem With Technique Name");
        // Obtain the Tactic Kill Chains
        for _item in items["kill_chain_phases"].as_array().unwrap().iter() {
            let _tactic = &_item["phase_name"].as_str().expect("Problem With Killchain Phase");
            if is_subtechnique {
                self.subtechniques.insert(
                    (_tid.to_string(), _tname.to_string(), _tactic.to_string())
                );
            } else {
                self.techniques.insert(
                    (_tid.to_string(), _tname.to_string(), _tactic.to_string())
                );
            }
        }
        self.stats.count_active_techniques = self.techniques.len();
        self.stats.count_active_subtechniques = self.subtechniques.len();
        Ok(())
    }
    pub fn to_string(&self) -> String
    {
        serde_json::to_string_pretty(self).unwrap()
    }
    pub fn save_baseline(&self)
    {
        FileHandler::write_baseline("baseline-enterprise.json", &self.to_string());
    }
}