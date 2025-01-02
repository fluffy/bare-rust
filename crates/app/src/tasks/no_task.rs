//! The `no_task` module contains the implementation of a placeholder task.
//! This task is used when no task is assigned and should never be run.

use super::Task;
use crate::metrics::Metrics;
use crate::msg::Msg;
use crate::tasks::TaskInfo;

/// Structure representing the NoTask.
pub struct NoTask {}

/// Information about the NoTask.
const NO_TASK_INFO: TaskInfo = TaskInfo {
    name: "NoTask",
    run_every_us: 0,
    time_budget_us: 0,
    mem_budget_bytes: 0,
};

impl Task for NoTask {
    /// Method to execute the NoTask.
    /// This method should never be called and will panic if invoked.
    fn run(
        &self,
        _msg: &Msg,
        _sender: &mut crate::v_mpsc::Sender<Msg>,
        _bsp: &mut dev::BSP,
        _metrics: &mut Metrics,
    ) {
        panic!("NoTask should never run");
    }

    /// Returns the information about the NoTask.
    #[allow(dead_code)]
    fn info(&self) -> &'static TaskInfo {
        &NO_TASK_INFO
    }
}
