use std::{pin::Pin, task};

use futures::Stream;

use crate::component::SetupContext;

use super::{Job, JobStage};

pub struct WaitCompletion {
    label: String,
    finalizer: Option<Box<dyn FnOnce(&mut SetupContext) -> eyre::Result<()> + Send + 'static>>,
}

impl WaitCompletion {
    pub fn new(
        label: impl ToString,
        finalizer: impl FnOnce(&mut SetupContext) -> eyre::Result<()> + Send + 'static,
    ) -> Pin<Box<Self>> {
        Box::pin(WaitCompletion {
            label: label.to_string(),
            finalizer: Some(Box::new(finalizer)),
        })
    }
}

impl Stream for WaitCompletion {
    type Item = JobStage;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Option<Self::Item>> {
        if let Some(finalizer) = self.finalizer.take() {
            task::Poll::Ready(Some(JobStage::Completed {
                label: self.label.clone(),
                finalizer,
            }))
        } else {
            task::Poll::Ready(None)
        }
    }
}

impl Job for WaitCompletion {}
