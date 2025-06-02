pub use slint::Model;
use std::sync::Arc;
use std::collections::HashMap;

pub mod history {
    use super::*;
    use slint::Model;
    use std::sync::Arc;
    use std::collections::HashMap;

    #[derive(Clone, Debug)]
    pub struct HistoryItem {
        pub id: String,
        pub parent_id: Option<String>,
        pub prompt: String,
        pub service: String,
        pub level: u32,
        pub is_expanded: bool,
        pub has_children: bool,
    }

    #[derive(Clone)]
    pub struct HistoryModel {
        pub items: Vec<HistoryItem>,
        pub children_map: HashMap<String, Vec<usize>>,
    }

    impl HistoryModel {
        pub fn new() -> Self {
            Self {
                items: Vec::new(),
                children_map: HashMap::new(),
            }
        }
    }

    impl Model for HistoryModel {
        type Data = HistoryItem;

        fn get(&self, index: usize) -> Option<Self::Data> {
            self.items.get(index).cloned()
        }

        fn len(&self) -> usize {
            self.items.len()
        }

        fn on_items_changed(&self) -> slint::SharedSignal {
            slint::SharedSignal::new()
        }
    }
}

pub use history::*;

/// Represents a single prompt in the conversation history tree.
/// Each HistoryItem maintains its position in the tree through parent_id and level.
#[derive(Clone, Debug)]
struct HistoryItem {
    /// Unique identifier for this prompt node
    id: String,
    /// ID of the parent node (None for root node)
    parent_id: Option<String>,
    /// The actual prompt text
    prompt: String,
    /// The AI service that generated this response
    service: String,
    /// Depth of this node in the tree (0 for root)
    level: u32,
    /// Whether this node is currently expanded in the UI
    is_expanded: bool,
    /// Whether this node has child nodes
    has_children: bool,
}

/// Represents the entire conversation history as a tree structure.
/// Implements Slint's Model trait to be used with UI components.
pub struct HistoryModel {
    /// Vector containing all history items in the tree
    items: Vec<HistoryItem>,
    /// HashMap for efficient lookup of items by their ID
    id_to_index: HashMap<String, usize>,
    /// ID of the root node of the conversation tree
    root_id: String,
}

impl HistoryModel {
    /// Creates a new empty HistoryModel with a root node
    /// 
    /// Returns:
    ///     A new HistoryModel instance
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            id_to_index: HashMap::new(),
            root_id: "root".to_string(),
        }
    }

    /// Adds a new prompt to the conversation history tree
    /// 
    /// Args:
    ///     id: Unique identifier for the new prompt
    ///     parent_id: ID of the parent prompt (None for root)
    ///     prompt: The prompt text
    ///     service: The AI service that generated this response
    /// 
    /// The method automatically calculates the correct level for the new node
    /// based on its parent's level. The has_children flag will be updated
    /// when children are added to a node.
    pub fn add_prompt(&mut self, id: String, parent_id: Option<String>, prompt: String, service: String) {
        let level = if let Some(parent_id) = &parent_id {
            self.get_level(parent_id) + 1
        } else {
            0
        };

        let has_children = false; // Will be updated when children are added
        let item = HistoryItem {
            id: id.clone(),
            parent_id,
            prompt,
            service,
            level,
            is_expanded: false,
            has_children,
        };

        let index = self.items.len();
        self.items.push(item);
        self.id_to_index.insert(id, index);
    }

    /// Gets the level of a node in the tree
    /// 
    /// Args:
    ///     id: ID of the node to get level for
    /// 
    /// Returns:
    ///     The level of the node (0 for root)
    fn get_level(&self, id: &str) -> u32 {
        if let Some(index) = self.id_to_index.get(id) {
            self.items[*index].level
        } else {
            0
        }
    }

    /// Expands a node in the UI
    /// 
    /// Args:
    ///     id: ID of the node to expand
    pub fn expand_node(&mut self, id: &str) {
        if let Some(index) = self.id_to_index.get(id) {
            self.items[*index].is_expanded = true;
        }
    }

    /// Collapses a node in the UI
    /// 
    /// Args:
    ///     id: ID of the node to collapse
    pub fn collapse_node(&mut self, id: &str) {
        if let Some(index) = self.id_to_index.get(id) {
            self.items[*index].is_expanded = false;
        }
    }
}

impl Model for HistoryModel {
    type Data = HistoryItem;

    fn get(&self, index: usize) -> Option<Self::Data> {
        self.items.get(index).cloned()
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn on_items_changed(&self) -> slint::SharedSignal {
        slint::SharedSignal::new()
    }
}
