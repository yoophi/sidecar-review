use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Block {
    Diff {
        #[serde(default)]
        title: String,
        file: String,
        #[serde(default)]
        before: String,
        #[serde(default)]
        after: String,
        /// Diagram that explains this code, shown above it. Empty when the code is obvious on its own.
        #[serde(default)]
        mermaid: String,
    },
    Code {
        #[serde(default)]
        title: String,
        file: String,
        code: String,
        /// Diagram that explains this code, shown above it. Empty when the code is obvious on its own.
        #[serde(default)]
        mermaid: String,
    },
    Mermaid {
        #[serde(default)]
        title: String,
        source: String,
    },
    Html {
        #[serde(default)]
        title: String,
        html: String,
    },
}

/// Everything one coding-agent session has shown the reviewer, in order.
#[derive(Clone, Serialize, Deserialize)]
pub struct Document {
    pub session: String,
    pub goal: String,
    pub blocks: Vec<Block>,
}

impl Document {
    pub fn new(session: String) -> Self {
        Self { session, goal: String::new(), blocks: vec![] }
    }

    pub fn append(&mut self, goal: String, blocks: Vec<Block>) {
        self.goal = goal;
        self.blocks.extend(blocks);
    }
}
