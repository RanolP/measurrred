use std::pin::Pin;

use super::SetupContext;
use futures::Stream;

pub type Job<'a> = Pin<Box<dyn Stream<Item = eyre::Result<JobStage>> + Send + 'a>>;

pub enum JobStage {
    Progress {
        label: String,
        value: f64,
    },
    Completed {
        label: String,
        finalizer: Box<dyn FnOnce(&mut SetupContext) -> eyre::Result<()> + Send>,
    },
    Fail {
        label: String,
    },
}

impl JobStage {
    pub fn label(&self) -> &str {
        match self {
            JobStage::Progress { label, .. } => &label,
            JobStage::Completed { label, .. } => &label,
            JobStage::Fail { label } => &label,
        }
    }

    pub fn progress(&self) -> f64 {
        match self {
            JobStage::Progress { value, .. } => *value,
            JobStage::Completed { .. } => 1.0,
            JobStage::Fail { .. } => 1.0,
        }
    }

    pub fn has_failed(&self) -> bool {
        matches!(self, JobStage::Fail { .. })
    }
}
