# CompatLib

Shared, dependency-free contract primitives for the Compat ecosystem.

CompatLib deliberately stays small. It defines the common experiment
 lifecycle that Compat tools can exchange:

- `Status` describes an experiment's planned, running, successful, failed, or
  cancelled state.
- `Stage` and `StageStatus` describe ordered units of work.
- `Experiment` contains an identifier, name, lifecycle status, and stages.
- `ExperimentReport` captures the final status and an optional summary.

The crate does not define process control, PyPI/package behaviour, persistence,
or a serialization format. Consumers can map these plain Rust values to the
wire format and platform conventions they already use.

## Example

```rust
use compatlib::{Experiment, ExperimentReport, Stage, Status, StageStatus};

let mut experiment = Experiment::new("exp-1", "compatibility sweep")
    .with_stage(Stage::new("run"));
experiment.status = Status::Succeeded;
experiment.stages[0].status = StageStatus::Succeeded;

let report = ExperimentReport::from_experiment(
    &experiment,
    Some("all checks passed".to_owned()),
);
assert_eq!(report.status, Status::Succeeded);
```

Run the focused unit tests with:

```text
cargo test
```
