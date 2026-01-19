use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Macro to define strongly-typed IDs
macro_rules! define_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, 
            Eq, Hash, Serialize, Deserialize)]
        pub struct $name(Uuid);

        
        impl $name {
            /// Generate new random ID (Uuid v4)
            pub fn new() -> Self {
                Self(Uuid::new_v4())
            }

            /// Create ID from raw string Uuid (useful when loading from DB)
            pub fn parse(s: &str) -> Result<Self, uuid::Error> {
                Ok(Self(Uuid::parse_str(s)?))
            }

            /// Returns inner raw Uuid
            pub fn into_inner(self) -> Uuid {
                self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

// Narrative IDs
define_id!(BookId);
define_id!(ChapterId);
define_id!(SceneId);

// World Building IDs
define_id!(CharacterId);
define_id!(LocationId);
define_id!(ItemId);
define_id!(CosmologyId);

// Chronology IDs
define_id!(EventId);
define_id!(TimelineId);

// Knowledge Base IDs
define_id!(NoteId);
define_id!(FolderId);