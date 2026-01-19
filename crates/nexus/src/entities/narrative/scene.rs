use crate::value_objects::id::SceneId;
use words_count::WordsCount;

struct Scene {
    id: SceneId,

    // Content
    title      : String,
    status     : NarrativeStatus,
    content    : String,
    notes      : String,

    // References (Options)
    perspective: Option<CharacterId>,
    place      : Option<LocationId>,

    // Lists
    characters : Vec<CharacterId>,
    itens      : Vec<ItemId>,

    // Metrics
    words      : u32,
}

impl Scene {
    pub fn new(title: String) -> Self {
    
        Self {
            id         : SceneId::new(),

            title,
            status     : NarrativeStatus::default(),
            content    : String::new(),
            notes      : String::new(),

            perspective: None,
            place      : None,

            characters : Vec::new(),
            itens      : Vec::new(),

            words      : 0,
        }
    }

    pub fn update_content(&mut self, new_text: String) {
        
    }
}