use serde_json;
use prettytable::{Table, Row, Cell};


#[path = "./parser.rs"]
mod parser;
use parser::EnterpriseMatrixBreakdown;


#[path = "../utils/fshandler.rs"]
mod fshandler;
use fshandler::FileHandler;



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
    pub fn enterprise_by_name(&self, technique_name: &str)
    {
        //println!("Search Term: {}", technique_name);
        let mut _results = vec![];
        let _json: EnterpriseMatrixBreakdown = serde_json::from_slice(&self.content[..]).unwrap();
        for _item in _json.breakdown_techniques.platforms.iter() {
            if _item.technique.to_lowercase().as_str() == technique_name.to_lowercase().as_str() {
                _results.push(_item);
            }
        }
        println!("{}", serde_json::to_string_pretty(&_results).unwrap());
    }
}