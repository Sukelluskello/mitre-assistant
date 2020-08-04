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


pub struct EnterpriseMatrixSearcher{
    matrix:     String,
    content:    Vec<u8> 
}
impl EnterpriseMatrixSearcher {
    pub fn new(matrix_type: &str) -> Self
    {
        let _input = matrix_type.to_lowercase();
        let mut _content: Vec<u8> = vec![];
        if _input == "enterprise".to_string() {
            _content = FileHandler::load_baseline("baselines", "baseline-enterprise.json");
        }
        EnterpriseMatrixSearcher {
            matrix:  _input,
            content: _content
        } 
    }
    pub fn search(&self, search_term: &str, _wants_subtechniques: bool)
    {
        let mut _results: Vec<String> = vec![];
        let mut _valid: Vec<(&str, usize)> = vec![];
        let _scanner = RegexPatternManager::load_search_term_patterns();
        // Special Flags
        //      Easier to search this way without flooding the user with parameters
        //      These flags are commonly placed in both the query and render functions 
        let mut _wants_stats: bool = false;             // Returns The Stats Key
        let mut _wants_nosub: bool = false;             // Returns Techniques That Don't Have Subtechniques
        let mut _wants_revoked: bool = false;           // Returns Techniques Revoked By Mitre
        let mut _wants_platforms: bool = false;         // Returns The Platforms Key
        let mut _wants_datasources: bool = false;       // Returns The Data Sources Key
        // Parse the search term explicitly
        //      We are not using partial matches on search term keywords
        //      We keep a simple incrementing usize by search term
        if search_term.to_lowercase().as_str() == "revoked" {
            _valid.push((search_term, 3usize));
            _wants_revoked = true;
        }
        else if search_term.to_lowercase().as_str() == "stats" {
            _valid.push((search_term, 4usize));
            _wants_stats = true;
        }
        else if search_term.to_lowercase().as_str() == "nosub" {
            _valid.push((search_term, 5usize));
            _wants_nosub = true;
        }
        else if search_term.to_lowercase().as_str() == "techniques" {
            _valid.push((search_term, 6usize)); //TODO
        }
        else if search_term.to_lowercase().as_str() == "subtechniques" {
            _valid.push((search_term, 7usize));     //TODO
        }
        else if search_term.to_lowercase().as_str() == "datasources" {
            _valid.push((search_term, 8usize));     //TODO
            _wants_datasources = true;
        }
        else if search_term.to_lowercase().as_str() == "platforms" {
            _valid.push((search_term, 9usize));     //TODO
            _wants_platforms = true;
        }
        else if search_term.to_lowercase().as_str() == "nodatasources" {
            _valid.push((search_term, 10usize));    // TODO
        }
        else if !search_term.contains(",") {
            if _scanner.pattern.is_match(search_term) {
                let _idx: Vec<usize> = _scanner.pattern.matches(search_term).into_iter().collect();
                _valid.push((search_term, _idx[0]));  // Search Term 0usize
            }
        }
        else if search_term.contains(",") {
            let _terms: Vec<&str> = search_term.split(',').collect();
            _valid = _terms.iter()
                        .filter(|_x| _scanner.pattern.is_match(_x))
                        .map(|_x| {
                            let _idx: Vec<_> = _scanner.pattern.matches(_x).into_iter().collect();
                            (*_x, _idx[0]) // Search Term 1usize
                        })
                        .collect();
        }        
        // Query
        // —————
        // Once a full match is valid and a pattern is assigned
        // let's redirect the pattern to the relevant query function
        //      Notice:     Based on the pattern usize, a specific function is called.
        //                  Any query function must return a Stringified Vector from
        //                  the `EnterpriseMatrixBreakdown` struct.
        if _valid.len() >= 1 {
            for (_term, _pattern) in _valid.iter() {
                if _pattern == &0usize {
                    _results.push(self.enterprise_by_id(_term, _wants_subtechniques));
                } else if _pattern == &1usize {
                    _results.push(self.enterprise_by_subtechnique_id(_term));
                } else if _pattern == &2usize {
                    _results.push(self.enterprise_by_name(_term));
                } else if _pattern == &3usize {
                    _results.push(self.enterprise_revoked());
                } else if _pattern == &4usize {
                    _results.push(self.enterprise_stats());
                } else if _pattern == &5usize {
                    _results.push(self.enterprise_by_nosubtechniques());
                } else if _pattern == &6usize {
                    _results.push(self.enterprise_all_techniques());
                } else if _pattern == &7usize {
                    _results.push(self.enterprise_all_subtechniques());
                } else if _pattern == &8usize {
                    _results.push(self.enterprise_all_datasources());
                } else if _pattern == &9usize {
                    _results.push(self.enterprise_all_platforms());
                } else if _pattern == &10usize {
                    _results.push(self.enterprise_by_no_datasources());
                }
            }
            // Render Query Results
            // --------------------
            // Upon getting search query results, apply a renderer to present results.
            // By default, pretty tables are used to render results.
            //
            //      Note:   Transforming results into CSV, JSON should be done within
            //              the renderer functions.
            //    
            if _wants_revoked {
                self.render_enterprise_revoked_table(&_results);
            } else if _wants_stats {
                self.render_enterprise_stats(&_results);
            } else if _wants_datasources {
                self.render_enterprise_datasources_table(&_results);
            } else if _wants_platforms {
                self.render_enterprise_platforms_table(&_results);
            } else {
                self.render_enterprise_table(&_results);
            }
        } else {
            println!(r#"[ "Results": {}, "SearchTerm": {} ]"#, "None Found", search_term);
        }
    }
    /// # **Query Functions**
    /// All of the functions from this source code section are for the queries provided by
    /// the end-user.
    ///
    /// Query functions must return a Stringified version of a JSON object - i.e., Vec<EnterpriseTechnique>
    ///
    /// The searcher uses the `serde_json::to_string` method for the conversion of objects to provide the
    /// Stringified version of the JSON object.
    ///
    ///
    /// ## **Query Functions Are Private**
    /// All of the functions are **private functions** that are not exposed to the end-user.  They are only accessible
    /// from the module itself, and specifically, when invoked by the `self.search()` method.
    ///
    fn enterprise_all_techniques(&self) -> String
    {
        //let mut _results = vec![];
        let _json: EnterpriseMatrixBreakdown = serde_json::from_slice(&self.content[..]).unwrap();
        serde_json::to_string(&_json.breakdown_techniques.platforms).expect("(?) Error: Unable To Deserialize All Techniques")
    }
    fn enterprise_all_subtechniques(&self) -> String
    {
        let _json: EnterpriseMatrixBreakdown = serde_json::from_slice(&self.content[..]).unwrap();
        serde_json::to_string(&_json.breakdown_subtechniques.platforms).expect("(?) Error: Unable To Deserialize All Techniques")
    }
    fn enterprise_all_platforms(&self) -> String
    {
        let _json: EnterpriseMatrixBreakdown = serde_json::from_slice(&self.content[..]).unwrap();
        serde_json::to_string(&_json.platforms).expect("(?) Error: Unable To Deserialize All Platforms")
    }
    fn enterprise_all_datasources(&self) -> String
    {
        let _json: EnterpriseMatrixBreakdown = serde_json::from_slice(&self.content[..]).unwrap();
        serde_json::to_string(&_json.datasources).expect("(?) Error: Unable To Deserialize All Datasources")
    }
    fn enterprise_by_no_datasources(&self) -> String
    {
        let mut _results = vec![];
        let _json: EnterpriseMatrixBreakdown = serde_json::from_slice(&self.content[..]).expect("(?) Error: Unable to Deserialize By No Datasources");
        for _item in _json.breakdown_techniques.platforms.iter() {
            if _item.datasources.as_str() == "None" {
                _results.push(_item);
            }
        }
        for _item in _json.breakdown_subtechniques.platforms.iter() {
            if _item.datasources.as_str() == "None" {
                _results.push(_item);
            }
        }
        serde_json::to_string(&_results).expect("(?) Error: Unable To Serialize By No Datasources")
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
    fn enterprise_by_id(&self, technique_id: &str, _wants_subtechniques: bool) -> String
    {
        let mut _results = vec![];
        let _json: EnterpriseMatrixBreakdown = serde_json::from_slice(&self.content[..]).expect("HERE");
        for _item in _json.breakdown_techniques.platforms.iter() {
            if _item.tid.to_lowercase().as_str() == technique_id.to_lowercase().as_str() {
                if _wants_subtechniques {
                    if _item.has_subtechniques {
                        _results.push(_item);
                        for _subtechnique in _json.breakdown_subtechniques.platforms.iter() {
                            if _subtechnique.tid.contains(&_item.tid) {
                                _results.push(_subtechnique);
                            }
                        }
                    }
                } else {
                    _results.push(_item);
                }
            }
        }
        if _results.len() == 0usize {
            // If no results then we want to search for a two conditions
            //      1. When the user wants subtechniques, then get them
            //      2. Or, when there are revoked techniques, let's add these
            //          to save time for users writing more queries
            if _wants_subtechniques {
                for _subtechnique in _json.breakdown_subtechniques.platforms.iter() {
                    if _subtechnique.tid.contains(technique_id.to_uppercase().as_str()) {
                        _results.push(_subtechnique);
                    }
                }
            }
            // Get Revoked Techniques
            let mut _results = vec![];
            for _revoked in _json.revoked_techniques.iter() {
                if _revoked.0.to_lowercase().as_str() == technique_id.to_lowercase().as_str() {
                    let mut _modified = EnterpriseTechnique::new();
                    _modified.tid = _revoked.0.clone();
                    _modified.technique = _revoked.1.clone();
                    _results.push(_modified);
                }
            }
            serde_json::to_string_pretty(&_results).expect("(?) Error:  Unable To Deserialize Search Results By Revoked Technique ID")
        } else {
            serde_json::to_string_pretty(&_results).expect("(?) Error:  Unable To Deserialize Search Results By Technique ID")
        }
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
        let _json: EnterpriseMatrixBreakdown = serde_json::from_slice(&self.content[..]).unwrap();
        serde_json::to_string_pretty(&_json.stats).expect("(?) Error:  Unable To Deserialize Search Results By Enterprise Stats")
    }
    fn enterprise_by_nosubtechniques(&self) -> String {
        let mut _results = vec![];
        let _json: EnterpriseMatrixBreakdown = serde_json::from_slice(&self.content[..]).unwrap();
        for _item in _json.breakdown_techniques.platforms.iter() {
            if !_item.has_subtechniques {
                _results.push(_item);
            }
        }
        serde_json::to_string_pretty(&_results).expect("(?) Error: Unable To Deserialize Search Results By HAS_NO_SUBTECHNIQUES")
    }
    /// # **Rendering Functions**
    /// This section of the source code is for functions that render queery results
    /// or render information to the end-user.
    ///
    fn render_enterprise_platforms_table(&self, results: &Vec<String>)
    {
        let mut _table = Table::new();
        _table.add_row(Row::new(vec![
            Cell::new("INDEX").style_spec("FW"),
            Cell::new("PLATFORMS").style_spec("FW"),
        ]));
        let _json: Vec<String> = serde_json::from_str(results[0].as_str()).expect("(?) Error: Unable To Deserialize Search Results By DataSources");
        for (_idx, _row) in _json.iter().enumerate() {
            _table.add_row(Row::new(vec![
                Cell::new((_idx + 1).to_string().as_str()).style_spec("FY"),
                Cell::new(_row.as_str()).style_spec("FW"),
            ]));
        }
        _table.printstd();
    } 
    fn render_enterprise_datasources_table(&self, results: &Vec<String>)
    {
        let mut _table = Table::new();
        _table.add_row(Row::new(vec![
            Cell::new("INDEX").style_spec("FW"),
            Cell::new("DATASOURCE").style_spec("FW"),
        ]));
        let _json: Vec<String> = serde_json::from_str(results[0].as_str()).expect("(?) Error: Unable To Deserialize Search Results By DataSources");
        for (_idx, _row) in _json.iter().enumerate() {
            _table.add_row(Row::new(vec![
                Cell::new((_idx + 1).to_string().as_str()).style_spec("FY"),
                Cell::new(_row.as_str()).style_spec("FW"),
            ]));
        }
        _table.printstd();
    } 
    fn render_enterprise_table(&self, results: &Vec<String>)
    {
        let mut _table = Table::new();
        _table.add_row(Row::new(vec![
            Cell::new("INDEX"),
            Cell::new("STATUS"),
            Cell::new("PLATFORMS"),
            Cell::new("TACTIC"),
            Cell::new("TID").style_spec("FG"),
            Cell::new("TECHNIQUE"),
            Cell::new("SUBTECHNIQUES"),
            Cell::new("DATA SOURCES")
        ]));
        // When we get to CSV Exports, put an if statement to build
        // the table cells without the `\n` terminators
        // because that will likely break CSV output
        //let mut _dataset = vec![];
        let mut _sorted_index: Vec<(String, usize, usize)> = vec![];
        for (_ridx, _item) in results.iter().enumerate() {
            let _json: Vec<EnterpriseTechnique> = serde_json::from_str(results[_ridx].as_str()).expect("(?) Error: Render Table Deserialization");
            for (_jidx, _record) in _json.iter().enumerate() {
                _sorted_index.push((_record.tid.clone(), _jidx, _ridx));
            }
        }
        _sorted_index.sort();
        //println!("{:#?}", _sorted_index);
        let mut _st = String::from("");
        let mut _idx: usize = 0;

        for (_technique, _jidx, _ridx) in _sorted_index {
            let _json: Vec<EnterpriseTechnique> = serde_json::from_str(results[_ridx].as_str()).expect("(?) Error: Render Table Deserialization");
            let _row = &_json[_jidx];
            if _row.has_subtechniques {
                _row.subtechniques.iter()
                    .map(|x| { _st.push_str(x.as_str()); _st.push_str("|") }).collect::<Vec<_>>();
            } else {
                _st.push_str("n_a");
            }
            // When a revoked Technique is part of the result
            // then create a row for the revoked technique
            if _row.datasources.as_str() == "n_a" {
                _table.add_row(
                    Row::new(vec![
                        Cell::new((_idx + 1).to_string().as_str()),
                        Cell::new("Revoked").style_spec("FR"),
                        Cell::new(_row.platform.replace("|", "\n").as_str()),
                        Cell::new(_row.tactic.as_str()),
                        Cell::new(_row.tid.as_str()).style_spec("FG"),
                        Cell::new(_row.technique.as_str()).style_spec("FW"),
                        Cell::new(_st.replace("|", "\n").as_str()).style_spec("FW"),
                        Cell::new(_row.datasources.replace("|", "\n").as_str())
                    ])
                ); 
            } else {
                _table.add_row(
                    Row::new(vec![
                        Cell::new((_idx + 1).to_string().as_str()),
                        Cell::new("Active"),
                        Cell::new(_row.platform.replace("|", "\n").as_str()),
                        Cell::new(_row.tactic.as_str()),
                        Cell::new(_row.tid.as_str()).style_spec("FG"),
                        Cell::new(_row.technique.as_str()).style_spec("FW"),
                        Cell::new(_st.replace("|", "\n").as_str()).style_spec("FW"),
                        Cell::new(_row.datasources.replace("|", "\n").as_str())
                    ])
                ); 
            }
            _st.clear();
            _idx += 1;            
        }
        println!("{}", "\n\n");
        _table.printstd();
        println!("{}", "\n\n");
    }
    fn render_enterprise_revoked_table(&self, results: &Vec<String>)
    {
        let mut _table = Table::new();
        _table.add_row(Row::new(vec![
            Cell::new("INDEX").style_spec("FW"),
            Cell::new("STATUS").style_spec("FR"),
            Cell::new("TID").style_spec("FR"),
            Cell::new("TECHNIQUE"),
        ]));
        let mut _idx: usize = 0;
        for _item in results.iter() {
            let mut _json: Vec<(&str, &str)> = serde_json::from_str(_item.as_str()).expect("(?) Error:  Render Table Deserialization For Revoked");
            _json.sort();
            for (_tid, _technique) in _json.iter() {

                _table.add_row(
                    Row::new(vec![
                        Cell::new((_idx + 1).to_string().as_str()),
                        Cell::new("Revoked"),
                        Cell::new(_tid).style_spec("FR"),
                        Cell::new(_technique).style_spec("FW")
                    ])
                );
                _idx += 1;
            }
        }
        println!("{}", "\n\n");
        _table.printstd();
        println!("{}", "\n\n");
    }
    fn render_enterprise_stats(&self, results: &Vec<String>)
    {
        let mut _table = Table::new();
        _table.add_row(Row::new(vec![
            Cell::new("CATEGORY"),
            Cell::new("COUNTS")
        ]));
        let _item = &results[0];
        let _json: EnterpriseMatrixStatistics = serde_json::from_str(_item.as_str()).expect("(?) Error:  Render Table Deserialization For Stats");
        // Uniques - Overview Section
        // Describes the uniq number of techniques
        // by platform only - no tactics are included
        _table.add_row(
            Row::new(vec![
                Cell::new("By Uniques").style_spec("FY"),
                Cell::new(""),
            ])
        );  
        _table.add_row(
            Row::new(vec![
                Cell::new("Active Techniques"),
                Cell::new(_json.count_active_uniq_techniques.to_string().as_str()),
            ])                                                                                                                                
        );
        _table.add_row(
            Row::new(vec![
                Cell::new("Active Subtechniques"),
                Cell::new(_json.count_active_uniq_subtechniques.to_string().as_str())
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
        // Totals - Overview Section
        // Describes the total number of techniques & subtechniques
        // by active, revoked - no tactics are included
        _table.add_empty_row();
        _table.add_row(
            Row::new(vec![
                Cell::new("By Totals").style_spec("FY"),
                Cell::new(""),
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
                Cell::new("Active Techniques"),
                Cell::new(_json.count_active_total_techniques.to_string().as_str()),
        ]));
        _table.add_row(
            Row::new(vec![
                Cell::new("Active Subtechniques"),
                Cell::new(_json.count_active_total_subtechniques.to_string().as_str()),
        ]));
        // Totals - Techniques Section
        // Describes the total number of techniques
        // by platform only - no tactics are included
        _table.add_empty_row();        
        _table.add_row(
            Row::new(vec![
                Cell::new("Totals - Techniques By Platform").style_spec("FY"),
                Cell::new(""),
            ])
        );        
        _table.add_row(
            Row::new(vec![
                Cell::new("AWS"),
                Cell::new(_json.count_techniques_aws.to_string().as_str()),
            ])
        );
        _table.add_row(
            Row::new(vec![
                Cell::new("AZURE"),
                Cell::new(_json.count_techniques_azure.to_string().as_str()),
            ])
        ); 
        _table.add_row(
            Row::new(vec![
                Cell::new("AZURE-AD"),
                Cell::new(_json.count_techniques_azure_ad.to_string().as_str()),
            ])
        );
        _table.add_row(
            Row::new(vec![
                Cell::new("GCP"),
                Cell::new(_json.count_techniques_gcp.to_string().as_str()),
            ])
        );
        _table.add_row(
            Row::new(vec![
                Cell::new("LINUX"),
                Cell::new(_json.count_techniques_linux.to_string().as_str()),
            ])
        );
        _table.add_row(
            Row::new(vec![
                Cell::new("MAC-OS"),
                Cell::new(_json.count_techniques_macos.to_string().as_str()),
            ])
        );
        _table.add_row(
            Row::new(vec![
                Cell::new("OFFICE-365"),
                Cell::new(_json.count_techniques_office365.to_string().as_str()),
            ])
        );
        _table.add_row(
            Row::new(vec![
                Cell::new("SAAS"),
                Cell::new(_json.count_techniques_saas.to_string().as_str()),
            ])
        );
        _table.add_row(
            Row::new(vec![
                Cell::new("WINDOWS"),
                Cell::new(_json.count_techniques_windows.to_string().as_str()),
            ])
        );                                                        
        // Totals - Subtechniques Section
        // Describes the total number of techniques
        // by platform only - no tactics are included
        _table.add_empty_row();
        _table.add_row(
            Row::new(vec![
                Cell::new("Total - Subtechniques By Platform").style_spec("FY"),
                Cell::new(""),
            ])
        );
        _table.add_row(
            Row::new(vec![
                Cell::new("AWS"),
                Cell::new(_json.count_subtechniques_aws.to_string().as_str()),
            ])
        );
        _table.add_row(
            Row::new(vec![
                Cell::new("AZURE"),
                Cell::new(_json.count_subtechniques_azure.to_string().as_str()),
            ])
        ); 
        _table.add_row(
            Row::new(vec![
                Cell::new("AZURE-AD"),
                Cell::new(_json.count_subtechniques_azure_ad.to_string().as_str()),
            ])
        );
        _table.add_row(
            Row::new(vec![
                Cell::new("GCP"),
                Cell::new(_json.count_subtechniques_gcp.to_string().as_str()),
            ])
        );
        _table.add_row(
            Row::new(vec![
                Cell::new("LINUX"),
                Cell::new(_json.count_subtechniques_linux.to_string().as_str()),
            ])
        );
        _table.add_row(
            Row::new(vec![
                Cell::new("MAC-OS"),
                Cell::new(_json.count_subtechniques_macos.to_string().as_str()),
            ])
        );
        _table.add_row(
            Row::new(vec![
                Cell::new("OFFICE-365"),
                Cell::new(_json.count_subtechniques_office365.to_string().as_str()),
            ])
        );
        _table.add_row(
            Row::new(vec![
                Cell::new("SAAS"),
                Cell::new(_json.count_subtechniques_saas.to_string().as_str()),
            ])
        );
        _table.add_row(
            Row::new(vec![
                Cell::new("WINDOWS"),
                Cell::new(_json.count_subtechniques_windows.to_string().as_str()),
            ])
        );                                                              
        // General Section
        // Used for placeholders if items (objects) not yet analyzed
        // These are TODOs
        _table.add_empty_row();
        _table.add_row(
            Row::new(vec![
                Cell::new("General - Pending Analysis").style_spec("FY"),
                Cell::new(""),
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