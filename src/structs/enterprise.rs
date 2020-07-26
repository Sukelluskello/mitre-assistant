use serde_derive::{Deserialize, Serialize};


#[derive(Debug,Deserialize, Serialize)]
pub struct EnterpriseMatrixStatistics {
    pub count_revoked_techniques:   usize,
    pub count_active_techniques:    usize,
    pub count_active_subtechniques: usize,
    pub count_malwares:             usize,
    pub count_adversaries:          usize,
    pub count_tools:                usize,
    pub count_platforms:            usize,
    pub count_tactics:              usize,
    pub count_datasources:          usize
}
impl EnterpriseMatrixStatistics {
    pub fn new() -> Self
    {
        EnterpriseMatrixStatistics {
            count_revoked_techniques:   0,
            count_active_techniques:    0,
            count_active_subtechniques: 0,
            count_malwares:             0,
            count_adversaries:          0,
            count_tools:                0,
            count_platforms:            0,
            count_tactics:              0,
            count_datasources:          0
        }
    }
}


#[derive(Debug,Deserialize, Serialize)]
pub struct EnterpriseTechniquesByPlatform {
    pub count:      usize,
    pub platforms:  Vec<EnterpriseTechnique>
}
impl EnterpriseTechniquesByPlatform {
    pub fn new() -> Self
    {
        EnterpriseTechniquesByPlatform {
            platforms:  vec![],
            count:      0
        }
    }
    pub fn update_count(&mut self)
    {
        self.count = self.platforms.len();
    }
}


#[derive(Debug,Deserialize, Serialize)]
pub struct EnterpriseTechnique {
    pub platform:   String,
    pub tid:        String,
    pub technique:  String,
    pub tactic:     String,
}
impl EnterpriseTechnique {
    pub fn new() -> Self
    {
        EnterpriseTechnique {
            platform:   String::from(""),
            tid:        String::from(""),
            technique:  String::from(""),
            tactic:     String::from("")
        }
    }
}


#[derive(Debug,Deserialize, Serialize)]
pub struct EnterpriseSubtechniquesByPlatform {
    pub count:      usize,
    pub platforms:  Vec<EnterpriseTechnique>
}
impl EnterpriseSubtechniquesByPlatform {
    pub fn new() -> Self
    {
        EnterpriseSubtechniquesByPlatform {
            count:     0,
            platforms: vec![]
        }
    }
    pub fn update_count(&mut self) {
        self.count = self.platforms.len();
    }
}


#[derive(Debug,Deserialize, Serialize)]
pub struct EnterpriseTechniquesByTactic {
    pub count:  usize,
    pub tactic: EnterpriseTactic    
}
impl EnterpriseTechniquesByTactic {
    pub fn new(tactic_name: &str) -> Self
    {
        EnterpriseTechniquesByTactic {
            count:  0,
            tactic: EnterpriseTactic::new(tactic_name)
        }
    }
}


#[derive(Debug,Deserialize, Serialize)]
pub struct EnterpriseTactic {
    pub tactic_name:    String,
    pub techniques:     Vec<EnterpriseTechnique>,
    pub subtechniques:  Vec<EnterpriseTechnique>
}
impl EnterpriseTactic {
    pub fn new(tactic_name: &str) -> Self
    {
        EnterpriseTactic {
            tactic_name:    tactic_name.to_string(),
            techniques:     vec![],
            subtechniques:  vec![]
        }
    }
}