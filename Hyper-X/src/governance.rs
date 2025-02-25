use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Governance {
    pub proposals: HashMap<u64, String>,
    pub votes: HashMap<u64, HashMap<String, bool>>,
}

impl Governance {
    pub fn new() -> Self {
        Self {
            proposals: HashMap::new(),
            votes: HashMap::new(),
        }
    }

    pub fn create_proposal(&mut self, id: u64, description: &str) {
        self.proposals.insert(id, description.to_string());
    }

    pub fn vote(&mut self, proposal_id: u64, user: &str, approve: bool) {
        self.votes.entry(proposal_id).or_insert_with(HashMap::new).insert(user.to_string(), approve);
    }

    pub fn tally_votes(&self, proposal_id: u64) -> (u64, u64) {
        let mut approve_count = 0;
        let mut reject_count = 0;

        if let Some(votes) = self.votes.get(&proposal_id) {
            for &vote in votes.values() {
                if vote {
                    approve_count += 1;
                } else {
                    reject_count += 1;
                }
            }
        }

        (approve_count, reject_count)
    }
}
