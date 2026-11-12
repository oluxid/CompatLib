//! Small, language-neutral contract primitives shared by Compat tools.
//!
//! The types in this crate intentionally model only the common lifecycle of
//! an experiment: its identity, stages, status, and a final report. They use
//! ordinary strings, booleans, and lists so other Compat implementations can
//! map them to their own wire formats without adopting a process or platform
//! abstraction.

/// Lifecycle status for an experiment or report.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    /// The work has been defined but not started.
    Planned,
    /// The work is currently running.
    Running,
    /// The work completed successfully.
    Succeeded,
    /// The work completed with an error.
    Failed,
    /// The work was intentionally stopped before completion.
    Cancelled,
}

/// Lifecycle status for an individual stage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StageStatus {
    /// The stage is waiting to run.
    Pending,
    /// The stage is currently running.
    Running,
    /// The stage completed successfully.
    Succeeded,
    /// The stage completed with an error.
    Failed,
    /// The stage was not run.
    Skipped,
}

/// A named unit of work within an experiment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Stage {
    /// Stable, human-readable stage name.
    pub name: String,
    /// Current stage lifecycle status.
    pub status: StageStatus,
}

impl Stage {
    /// Creates a stage that is waiting to run.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: StageStatus::Pending,
        }
    }
}

/// The shared description and lifecycle state of an experiment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Experiment {
    /// Stable identifier assigned by the caller.
    pub id: String,
    /// Human-readable experiment name.
    pub name: String,
    /// Current experiment lifecycle status.
    pub status: Status,
    /// Ordered stages belonging to the experiment.
    pub stages: Vec<Stage>,
}

impl Experiment {
    /// Creates a planned experiment with no stages.
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            status: Status::Planned,
            stages: Vec::new(),
        }
    }

    /// Adds a stage and returns the updated experiment.
    pub fn with_stage(mut self, stage: Stage) -> Self {
        self.stages.push(stage);
        self
    }
}

/// A platform-neutral result emitted after an experiment run.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExperimentReport {
    /// Identifier of the experiment that produced the report.
    pub experiment_id: String,
    /// Final experiment status.
    pub status: Status,
    /// Final status of each stage, in execution order.
    pub stages: Vec<Stage>,
    /// Optional human-readable summary.
    pub summary: Option<String>,
}

impl ExperimentReport {
    /// Creates a report from an experiment's current state.
    pub fn from_experiment(experiment: &Experiment, summary: Option<String>) -> Self {
        Self {
            experiment_id: experiment.id.clone(),
            status: experiment.status,
            stages: experiment.stages.clone(),
            summary,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_experiment_is_planned_and_accepts_ordered_stages() {
        let experiment = Experiment::new("exp-1", "compatibility sweep")
            .with_stage(Stage::new("prepare"))
            .with_stage(Stage::new("run"));

        assert_eq!(experiment.status, Status::Planned);
        assert_eq!(
            experiment
                .stages
                .iter()
                .map(|stage| stage.name.as_str())
                .collect::<Vec<_>>(),
            ["prepare", "run"]
        );
        assert!(experiment
            .stages
            .iter()
            .all(|stage| stage.status == StageStatus::Pending));
    }

    #[test]
    fn report_copies_identity_and_stage_state_without_sharing_storage() {
        let mut experiment =
            Experiment::new("exp-2", "reporting").with_stage(Stage::new("execute"));
        experiment.status = Status::Succeeded;
        experiment.stages[0].status = StageStatus::Succeeded;

        let report =
            ExperimentReport::from_experiment(&experiment, Some("all checks passed".into()));

        assert_eq!(report.experiment_id, "exp-2");
        assert_eq!(report.status, Status::Succeeded);
        assert_eq!(report.stages[0].status, StageStatus::Succeeded);
        assert_eq!(report.summary.as_deref(), Some("all checks passed"));

        experiment.stages[0].status = StageStatus::Failed;
        assert_eq!(report.stages[0].status, StageStatus::Succeeded);
    }
}
