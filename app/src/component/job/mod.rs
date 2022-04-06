use std::fmt::Display;

use futures::Stream;

use super::SetupContext;

mod wait_completion;

pub use wait_completion::*;

pub trait Job: Stream<Item = JobStage> + Send {
}

pub enum JobStage {
    Progress {
        label: String,
        value: f64,
    },
    Completed {
        label: String,
        finalizer: Box<dyn FnOnce(&mut SetupContext) -> eyre::Result<()>>
    }
}

impl JobStage {
    pub fn label(&self) -> &str {
        match self {
            JobStage::Progress { label, .. } => &label,
            JobStage::Completed { label, .. } => &label,
        }
    }

    pub fn progress(&self) -> f64 {
        match self {
            JobStage::Progress { value, .. } => *value,
            JobStage::Completed { .. } => 1.0,
        }
    }
}
