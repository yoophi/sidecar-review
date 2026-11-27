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

/// Where a review comes from, as told by the agent. An empty field means it did not say.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Origin {
    #[serde(default)]
    pub agent: String,
    #[serde(default)]
    pub cwd: String,
    #[serde(default)]
    pub agent_session_id: String,
}

/// Everything one coding-agent session has shown the reviewer, in order.
#[derive(Clone, Serialize, Deserialize)]
pub struct Document {
    pub session: String,
    pub goal: String,
    pub blocks: Vec<Block>,
    /// Unix ms. 0 in documents saved before these fields existed.
    #[serde(default)]
    pub created_at: u64,
    #[serde(default)]
    pub updated_at: u64,
    #[serde(default)]
    pub origin: Origin,
}

impl Document {
    pub fn new(session: String, now: u64) -> Self {
        Self { session, goal: String::new(), blocks: vec![], created_at: now, updated_at: now, origin: Origin::default() }
    }

    pub fn append(&mut self, goal: String, blocks: Vec<Block>, origin: Origin, now: u64) {
        // a later push that leaves a field out keeps what an earlier one said
        for (known, told) in [
            (&mut self.origin.agent, origin.agent),
            (&mut self.origin.cwd, origin.cwd),
            (&mut self.origin.agent_session_id, origin.agent_session_id),
        ] {
            if !told.is_empty() {
                *known = told;
            }
        }
        self.goal = goal;
        self.updated_at = now;
        self.blocks.extend(blocks);
    }
}
