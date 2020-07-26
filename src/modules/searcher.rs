use serde_json;
use prettytable::{Table, Row, Cell};


#[path = "./parser.rs"]
mod parser;
use parser::EnterpriseMatrixBreakdown;


#[path = "../structs/enterprise.rs"]
mod enterprise;
use enterprise::{EnterpriseTechnique, EnterpriseMatrixStatistics};


#[path = "../utils/fshandler.rs"]
mod fshandler;
use fshandler::FileHandler;


#[path = "../utils/regexes.rs"]
mod regexes;
use regexes::RegexPatternManager;


pub struct MatrixSearcher{
    matrix:     String,
    content:    Vec<u8> 
}
impl MatrixSearcher {
    pub fn new(matrix_type: &str) -> Self
    {
        let _input = matrix_type.to_lowercase();
        let mut _content: Vec<u8> = vec![];
        if _input == "enterprise".to_string() {
            _content = FileHandler::load_baseline("baselines", "baseline-enterprise.json");
        }
        MatrixSearcher {
            matrix:  _input,
            content: _content
        } 
    }
    pub fn search(&self, search_term: &str)
    {
        let mut _results: Vec<String> = vec![];
        let mut _valid: Vec<(&str, usize)> = vec![];
        let mut _wants_revoked: bool = false;
        let mut _wants_stats = false;
        let _scanner = RegexPatternManager::load_search_term_patterns();
        if search_term.to_lowercase().as_str() == "revoked" {
            _valid.push((search_term, 3usize));
            _wants_revoked = true;
        }
        else if search_term.to_lowercase().as_str() == "stats" {
            _valid.push((search_term, 4usize));
            _wants_stats = true;
        }
        else if !search_term.contains(",") {
            if _scanner.pattern.is_match(search_term) {
                let _idx: Vec<usize> = _scanner.pattern.matches(search_term).into_iter().collect();
                _valid.push((search_term, _idx[0]));
            }
        }
        else if search_term.contains(",") {
            // Split the terms
            let _terms: Vec<&str> = search_term.split(',').collect();
            //println!("{:#?}", _terms);
            // Validate the terms
            _valid = _terms.iter()
                        .filter(|_x| _scanner.pattern.is_match(_x))
                        .map(|_x| {
                            let _idx: Vec<_> = _scanner.pattern.matches(_x).into_iter().collect();
                            (*_x, _idx[0])
                        })
                        .collect();
            //println!("{:#?}", _valid);
        }
        if _valid.len() >= 1 {
            for (_term, _pattern) in _valid.iter() {
                if _pattern == &0usize {
                    _results.push(self.enterprise_by_id(_term));
                } else if _pattern == &1usize {
                    _results.push(self.enterprise_by_subtechnique_id(_term));
                } else if _pattern == &2usize {
                    _results.push(self.enterprise_by_name(_term));
                } else if _pattern == &3usize {
                    _results.push(self.enterprise_revoked());
                } else if _pattern == &4usize {
                    _results.push(self.enterprise_stats());
                }
            }
            _results.sort();
            if _wants_revoked {
                self.render_enterprise_revoked_table(&_results);
            } else if _wants_stats {
                self.render_enterprise_stats(&_results);
            } else {
                self.render_enterprise_table(&_results);
            }
            //println!("Results Length: {}\n\n{:#?}", _results.len(), _results);
        } else {
            println!(r#"[ "Results": {}, "SearchTerm": {} ]"#, "None Found", search_term);
        }
    }
    fn enterprise_by_name(&self, technique_name: &str) -> String
    {
        let mut _results = vec![];
        let _json: EnterpriseMatrixBreakdown = serde_json::from_slice(&self.content[..]).unwrap();
        for _item in _json.breakdown_techniques.platforms.iter() {
            if _item.technique.to_lowercase().as_str() == technique_name.to_lowercase().as_str() {
                _results.push(_item);
            }
        }
        serde_json::to_string_pretty(&_results).expect("(?) Error:  Unable To Deserialize Search Results By Technique Name")
    }
    fn enterprise_by_id(&self, technique_id: &str) -> String
    {
        let mut _results = vec![];
        let _json: EnterpriseMatrixBreakdown = serde_json::from_slice(&self.content[..]).unwrap();
        for _item in _json.breakdown_techniques.platforms.iter() {
            if _item.tid.to_lowercase().as_str() == technique_id.to_lowercase().as_str() {
                _results.push(_item);
            }
        }
        serde_json::to_string_pretty(&_results).expect("(?) Error:  Unable To Deserialize Search Results By Technique ID")
    }
    fn enterprise_by_subtechnique_id(&self, technique_id: &str) -> String
    {
        let mut _results = vec![];
        let _json: EnterpriseMatrixBreakdown = serde_json::from_slice(&self.content[..]).unwrap();
        for _item in _json.breakdown_subtechniques.platforms.iter() {
            if _item.tid.to_lowercase().as_str() == technique_id.to_lowercase().as_str() {
                _results.push(_item);
            }
        }
        serde_json::to_string_pretty(&_results).expect("(?) Error:  Unable To Deserialize Search Results By Subtechnique ID")
    }
    fn enterprise_revoked(&self) -> String
    {
        let mut _results = vec![];
        let _json: EnterpriseMatrixBreakdown = serde_json::from_slice(&self.content[..]).unwrap();
        for _item in _json.revoked_techniques.iter() {
            _results.push(_item);
        }
        serde_json::to_string_pretty(&_results).expect("(?) Error:  Unable To Deserialize Search Results By Revoked Techniques")
    }
    fn enterprise_stats(&self) -> String
    {
        //let mut _results = vec![];
        let _json: EnterpriseMatrixBreakdown = serde_json::from_slice(&self.content[..]).unwrap();
        //_results.push(_json.stats);
        serde_json::to_string_pretty(&_json.stats).expect("(?) Error:  Unable To Deserialize Search Results By Enterprise Stats")
    }
    fn render_enterprise_table(&self, results: &Vec<String>)
    {
        //let _dashes: String = "=".repeat(64);
        //println!("\n\n{}\n{}\n{}\n", _dashes, headline, _dashes);
        // Create a new table
        let mut _table = Table::new();
        // Add headers to the table
        _table.add_row(Row::new(vec![
            Cell::new("STATUS"),
            Cell::new("PLATFORM"),
            Cell::new("TACTIC"),
            Cell::new("TID").style_spec("FG"),
            Cell::new("TECHNIQUE"),
            Cell::new("DATA SOURCES")
        ]));
        for _item in results.iter() {
            let _json: Vec<EnterpriseTechnique> = serde_json::from_str(_item.as_str()).expect("(?) Error: Render Table Deserialization");
            for _row in _json.iter() {
                _table.add_row(
                    Row::new(vec![
                        Cell::new("Active"),
                        Cell::new(_row.platform.as_str()),
                        Cell::new(_row.tactic.as_str()),
                        Cell::new(_row.tid.as_str()).style_spec("FG"),
                        Cell::new(_row.technique.as_str()),
                        Cell::new(_row.datasources.as_str())
                    ])
                );
            }
        }
        _table.printstd();
    }
    fn render_enterprise_revoked_table(&self, results: &Vec<String>)
    {
        let mut _table = Table::new();
        _table.add_row(Row::new(vec![
            Cell::new("STATUS").style_spec("FR"),
            Cell::new("TID").style_spec("FR"),
            Cell::new("TECHNIQUE"),
        ]));
        for _item in results.iter() {
            let mut _json: Vec<(&str, &str)> = serde_json::from_str(_item.as_str()).expect("(?) Error:  Render Table Deserialization For Revoked");
            _json.sort();
            for (_tid, _technique) in _json.iter() {
                _table.add_row(
                    Row::new(vec![
                        Cell::new("Revoked"),
                        Cell::new(_tid),
                        Cell::new(_technique)
                    ])
                );
            }
        }
        _table.printstd();
    }
    fn render_enterprise_stats(&self, results: &Vec<String>)
    {
        let mut _table = Table::new();
        _table.add_row(Row::new(vec![
            Cell::new("CATEGORY"),
            Cell::new("TOTALS")
        ]));
        let _item = &results[0];
        let _json: EnterpriseMatrixStatistics = serde_json::from_str(_item.as_str()).expect("(?) Error:  Render Table Deserialization For Stats");
        _table.add_row(
            Row::new(vec![
                Cell::new("Active Techniques"),
                Cell::new(_json.count_active_techniques.to_string().as_str()),
            ])                                                                                                                                
        );
        _table.add_row(
            Row::new(vec![
                Cell::new("Active Subtechniques"),
                Cell::new(_json.count_active_subtechniques.to_string().as_str())
            ])
        );
        _table.add_row(
            Row::new(vec![
                Cell::new("Revoked Techniques"),
                Cell::new(_json.count_revoked_techniques.to_string().as_str()),
            ])
        );
        _table.add_row(
            Row::new(vec![
                Cell::new("Active Platforms"),
                Cell::new(_json.count_platforms.to_string().as_str()),
            ])
        );
        _table.add_row(
            Row::new(vec![
                Cell::new("Active Tactics"),
                Cell::new(_json.count_tactics.to_string().as_str()),
            ])
        );
        _table.add_row(
            Row::new(vec![
                Cell::new("Active Data Sources"),
                Cell::new(_json.count_datasources.to_string().as_str()),
            ])
        );
        _table.add_row(
            Row::new(vec![
                Cell::new("Records For Malware"),
                Cell::new(_json.count_malwares.to_string().as_str()),
            ])
        );
        _table.add_row(
            Row::new(vec![
                Cell::new("Records For Adversaries"),
                Cell::new(_json.count_adversaries.to_string().as_str())  
            ])
        ); 
        _table.add_row(
            Row::new(vec![
                Cell::new("Records For Tools"),
                Cell::new(_json.count_tools.to_string().as_str()),
            ])
        );
        println!("\n\n");        
        _table.printstd();
        println!("\n\n");    
    }
}