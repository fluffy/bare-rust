use super::{Task, TaskData};
use crate::metrics::Metrics;
use crate::msg::Msg;
use crate::tasks::TaskInfo;

/// Structure representing the display task.
pub struct DisplayTask {}

/// Information about the display task.
const DISPLAY_TASK_INFO: TaskInfo = TaskInfo {
    name: "Display",
    run_every_us: 100_000,
    time_budget_us: 10_000,
    mem_budget_bytes: 300,
};

impl Task for DisplayTask {
    /// Method to execute the display task.
    /// Reads the state of the display and sends a message if the state has changed.
    fn run(
        &self,
        _incoming_msg: &Msg,
        _sender: &mut crate::mpsc::Sender<Msg>,
        _bsp: &mut bsp::BSP,
        _task_data: &mut TaskData,
        _metrics: &mut Metrics,
    ) {
    }

    /// Returns the information about the display task.
    fn info(&self) -> &'static TaskInfo {
        &DISPLAY_TASK_INFO
    }
}
