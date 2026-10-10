use crate::view::*;
use nddev_device_sync_adapter_io::{CancellationToken, IoError, ProcessIo, ProcessRequest};

use std::{path::PathBuf, time::Duration};

#[derive(Clone)]
pub struct Updater {
    io: ProcessIo,
    executable: PathBuf,
}

impl Updater {
    pub fn new(io: ProcessIo, executable: PathBuf) -> Result<Self, IoError> {
        if !executable.is_absolute() {
            return Err(IoError::InvalidConfiguration);
        }
        Ok(Self { io, executable })
    }

    fn request(&self, operation: &str) -> ProcessRequest {
        ProcessRequest {
            executable: self.executable.clone(),
            arguments: [operation, "--json"].into_iter().map(Into::into).collect(),
            directory: None,
            timeout: Duration::from_secs(10),
            max_output_bytes: 1_048_576,
        }
    }

    pub async fn plan(&self, cancellation: CancellationToken) -> Result<Plan, IoError> {
        let (plan, _): (Plan, _) = self
            .io
            .json(self.request("plan"), &[0], cancellation)
            .await?;
        if plan.schema != 2 || plan.actions.len() > 16 || !valid_channel(&plan.channel) {
            return Err(IoError::InvalidResponse);
        }
        Ok(plan)
    }

    pub async fn last_run(
        &self,
        cancellation: CancellationToken,
    ) -> Result<UpdateRunSummary, IoError> {
        let (report, _): (UpdateRunSummary, _) = self
            .io
            .json(self.request("status"), &[0], cancellation)
            .await?;
        if report.schema != 2
            || report.results.len() > 128
            || !valid_channel(&report.channel)
            || report.finished_at < report.started_at
            || report
                .results
                .iter()
                .any(|result| result.state.is_empty() || result.action.is_empty())
        {
            return Err(IoError::InvalidResponse);
        }
        Ok(report)
    }
}

fn valid_channel(channel: &str) -> bool {
    !channel.is_empty() && channel.len() <= 32
}
