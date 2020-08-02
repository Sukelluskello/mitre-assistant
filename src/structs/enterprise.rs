use serde_derive::{Deserialize, Serialize};


#[derive(Debug,Deserialize, Serialize)]
pub struct EnterpriseMatrixStatistics {
    pub count_revoked_techniques:           usize,
    pub count_active_total_techniques:      usize,
    pub count_active_total_subtechniques:   usize,
    pub count_active_uniq_techniques:       usize,
    pub count_active_uniq_subtechniques:    usize,
    pub count_malwares:                     usize,
    pub count_adversaries:                  usize,
    pub count_tools:                        usize,
    pub count_platforms:                    usize,
    pub count_tactics:                      usize,
    pub count_datasources:                  usize,
    // Count of Techniques by Platforms
    // Use these with stats functions
    pub count_techniques_aws:               usize,
    pub count_techniques_azure:             usize,
    pub count_techniques_azure_ad:          usize,
    pub count_techniques_gcp:               usize,
    pub count_techniques_linux:             usize,
    pub count_techniques_macos:             usize,
    pub count_techniques_office365:         usize,
    pub count_techniques_saas:              usize,
    pub count_techniques_windows:           usize,
    // Count of Subtechniques by Platforms
    // Use these with stats functions
    pub count_subtechniques_aws:            usize,
    pub count_subtechniques_azure:          usize,
    pub count_subtechniques_azure_ad:       usize,
    pub count_subtechniques_gcp:            usize,
    pub count_subtechniques_linux:          usize,
    pub count_subtechniques_macos:          usize,
    pub count_subtechniques_office365:      usize,
    pub count_subtechniques_saas:           usize,
    pub count_subtechniques_windows:        usize,        
}
impl EnterpriseMatrixStatistics {
    pub fn new() -> Self
    {
        EnterpriseMatrixStatistics {
            count_revoked_techniques:           0,
            count_active_total_techniques:      0,
            count_active_total_subtechniques:   0,
            count_active_uniq_techniques:       0,
            count_active_uniq_subtechniques:    0,
            count_malwares:                     0,
            count_adversaries:                  0,
            count_tools:                        0,
            count_platforms:                    0,
            count_tactics:                      0,
            count_datasources:                  0,
            count_techniques_aws:               0,
            count_techniques_azure:             0,
            count_techniques_azure_ad:          0,
            count_techniques_gcp:               0,
            count_techniques_linux:             0,
            count_techniques_macos:             0,
            count_techniques_office365:         0,
            count_techniques_saas:              0,
            count_techniques_windows:           0,
            count_subtechniques_aws:            0,
            count_subtechniques_azure:          0,
            count_subtechniques_azure_ad:       0,
            count_subtechniques_gcp:            0,
            count_subtechniques_linux:          0,
            count_subtechniques_macos:          0,
            count_subtechniques_office365:      0,
            count_subtechniques_saas:           0,
            count_subtechniques_windows:        0,                        
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
    pub platform:       String,
    pub tid:            String,
    pub technique:      String,
    pub tactic:         String,
    pub datasources:    String,
    pub has_subtechniques: bool,
    pub subtechniques:  Vec<String>,
    pub count_subtechniques: usize
}
impl EnterpriseTechnique {
    pub fn new() -> Self
    {
        EnterpriseTechnique {
            platform:       String::from(""),
            tid:            String::from(""),
            technique:      String::from(""),
            tactic:         String::from(""),
            datasources:    String::from("None"),
            has_subtechniques: false,
            subtechniques:  vec![],
            count_subtechniques: 0usize
        }
    }
    pub fn update(&mut self)
    {
        self.count_subtechniques = self.subtechniques.len();
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