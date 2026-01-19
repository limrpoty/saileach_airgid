use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, 
    Eq, Hash, Serialize, Deserialize)]
pub enum NarrativeStatus {
    Draft,
    Revise,
    Final,
    Archived,
}

impl Default for NarrativeStatus {
    fn default() -> Self {
        Self::Draft
    }
}

pub struct WordCount {

}