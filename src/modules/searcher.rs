use serde_json;
use prettytable::{Table, Row, Cell};


#[path = "./parser.rs"]
mod parser;
use parser::EnterpriseMatrixBreakdown;


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
            matrix:     _input,
            content:    _content
        } 
    }
    pub fn search(&self, search_term: &str)
    {
        let mut _results: Vec<String> = vec![];
        let mut _valid: Vec<(&str, usize)> = vec![];
        let _scanner = RegexPatternManager::load_search_term_patterns();
        if !search_term.contains(",") {
            if _scanner.pattern.is_match(search_term) {
                let _idx: Vec<usize> = _scanner.pattern.matches(search_term).into_iter().collect();
                _valid.push((search_term, _idx[0]));
            }
            //println!("{:#?}", _valid);
        }
        else if search_term.contains(",") {
            // Split the terms
            let _terms: Vec<&str> = search_term.split(',').collect();
            println!("{:#?}", _terms);
            // Validate the terms
            _valid = _terms.iter()
                        .filter(|_x| _scanner.pattern.is_match(_x))
                        .map(|_x| {
                            let _idx: Vec<_> = _scanner.pattern.matches(_x).into_iter().collect();
                            (*_x, _idx[0])
                        })
                        .collect();
            println!("{:#?}", _valid);
        }
        if _valid.len() >= 1 {
            for (_term, _pattern) in _valid.iter() {
                if _pattern == &0usize {
                        _results.push(self.enterprise_by_id(*_term));
                } else if _pattern == &1usize {
                        _results.push(self.enterprise_by_subtechnique_id(*_term));
                } else if _pattern == &2usize {
                        _results.push(self.enterprise_by_name(*_term));
                }
                println!("{:#?}", _results)
            }
        } else {
            println!(r#"[ "Results": {}, "SearchTerm": {} ]"#, "None Found", search_term);
        }

    }
    pub fn enterprise_by_name(&self, technique_name: &str) -> String
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
    pub fn enterprise_by_id(&self, technique_id: &str) -> String
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
    pub fn enterprise_by_subtechnique_id(&self, technique_id: &str) -> String
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
}