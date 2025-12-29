//! State Machine for workflow/status transitions.
//!
//! # Example
//!
//! ```ignore
//! use rust::helpers::state_machine::{StateMachine, Transition};
//!
//! let mut sm = StateMachine::new("draft");
//! sm.add_transition("draft", "pending", "submit");
//! sm.add_transition("pending", "approved", "approve");
//! sm.add_transition("pending", "rejected", "reject");
//!
//! sm.transition("submit")?; // draft -> pending
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// State machine error.
#[derive(Debug, thiserror::Error)]
pub enum StateMachineError {
    #[error("Invalid transition '{0}' from state '{1}'")]
    InvalidTransition(String, String),
    #[error("State '{0}' not found")]
    StateNotFound(String),
    #[error("Transition '{0}' not found")]
    TransitionNotFound(String),
}

/// Transition definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    pub name: String,
    pub from: String,
    pub to: String,
}

impl Transition {
    pub fn new(from: &str, to: &str, name: &str) -> Self {
        Self {
            name: name.to_string(),
            from: from.to_string(),
            to: to.to_string(),
        }
    }
}

/// State machine configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMachineConfig {
    pub initial_state: String,
    pub states: Vec<String>,
    pub transitions: Vec<Transition>,
}

/// State machine instance.
#[derive(Debug, Clone)]
pub struct StateMachine {
    current_state: String,
    transitions: HashMap<String, Vec<Transition>>,
    history: Vec<String>,
}

impl StateMachine {
    /// Create a new state machine.
    pub fn new(initial_state: &str) -> Self {
        Self {
            current_state: initial_state.to_string(),
            transitions: HashMap::new(),
            history: vec![initial_state.to_string()],
        }
    }

    /// Create from config.
    pub fn from_config(config: StateMachineConfig) -> Self {
        let mut sm = Self::new(&config.initial_state);
        for t in config.transitions {
            sm.transitions.entry(t.from.clone()).or_default().push(t);
        }
        sm
    }

    /// Add a transition.
    pub fn add_transition(&mut self, from: &str, to: &str, name: &str) -> &mut Self {
        let transition = Transition::new(from, to, name);
        self.transitions
            .entry(from.to_string())
            .or_default()
            .push(transition);
        self
    }

    /// Get current state.
    pub fn state(&self) -> &str {
        &self.current_state
    }

    /// Check if in a specific state.
    pub fn is(&self, state: &str) -> bool {
        self.current_state == state
    }

    /// Get available transitions from current state.
    pub fn available_transitions(&self) -> Vec<&Transition> {
        self.transitions
            .get(&self.current_state)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    /// Check if a transition is available.
    pub fn can(&self, transition_name: &str) -> bool {
        self.available_transitions()
            .iter()
            .any(|t| t.name == transition_name)
    }

    /// Perform a transition.
    pub fn transition(&mut self, name: &str) -> Result<&str, StateMachineError> {
        let transitions = self
            .transitions
            .get(&self.current_state)
            .ok_or_else(|| StateMachineError::StateNotFound(self.current_state.clone()))?;

        let transition = transitions.iter().find(|t| t.name == name).ok_or_else(|| {
            StateMachineError::InvalidTransition(name.to_string(), self.current_state.clone())
        })?;

        self.current_state = transition.to.clone();
        self.history.push(self.current_state.clone());

        Ok(&self.current_state)
    }

    /// Force set state (bypasses transition rules).
    pub fn force_state(&mut self, state: &str) {
        self.current_state = state.to_string();
        self.history.push(state.to_string());
    }

    /// Get state history.
    pub fn history(&self) -> &[String] {
        &self.history
    }

    /// Reset to initial state.
    pub fn reset(&mut self, initial: &str) {
        self.current_state = initial.to_string();
        self.history = vec![initial.to_string()];
    }
}

/// Trait for models with state.
pub trait HasState {
    /// Get current state.
    fn state(&self) -> &str;

    /// Set state.
    fn set_state(&mut self, state: &str);

    /// Get state machine configuration.
    fn state_machine_config() -> StateMachineConfig;
}

/// Common order states.
pub mod order_states {
    use super::*;

    pub fn config() -> StateMachineConfig {
        StateMachineConfig {
            initial_state: "pending".to_string(),
            states: vec![
                "pending".to_string(),
                "processing".to_string(),
                "shipped".to_string(),
                "delivered".to_string(),
                "cancelled".to_string(),
            ],
            transitions: vec![
                Transition::new("pending", "processing", "process"),
                Transition::new("pending", "cancelled", "cancel"),
                Transition::new("processing", "shipped", "ship"),
                Transition::new("processing", "cancelled", "cancel"),
                Transition::new("shipped", "delivered", "deliver"),
            ],
        }
    }
}

/// Common post states.
pub mod post_states {
    use super::*;

    pub fn config() -> StateMachineConfig {
        StateMachineConfig {
            initial_state: "draft".to_string(),
            states: vec![
                "draft".to_string(),
                "pending".to_string(),
                "published".to_string(),
                "archived".to_string(),
            ],
            transitions: vec![
                Transition::new("draft", "pending", "submit"),
                Transition::new("pending", "published", "publish"),
                Transition::new("pending", "draft", "reject"),
                Transition::new("published", "archived", "archive"),
                Transition::new("archived", "published", "restore"),
            ],
        }
    }
}
