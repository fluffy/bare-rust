use super::{Task, TaskData};
use crate::metrics::Metrics;
use crate::msg::Msg;
use crate::tasks::TaskInfo;

/// Structure representing the chat task.
pub struct ChatTask {}

/// Information about the chat task.
const CHAT_TASK_INFO: TaskInfo = TaskInfo {
    name: "Chat",
    run_every_us: 100_000,
    time_budget_us: 10_000,
    mem_budget_bytes: 500,
};

impl Task for ChatTask {
    /// Method to execute the chat task.
    /// Reads the state of the chat and sends a message if the state has changed.
    fn run(
        &self,
        _incoming_msg: &Msg,
        _sender: &mut crate::mpsc::Sender<Msg>,
        _bsp: &mut bsp::BSP,
        _task_data: &mut TaskData,
        _metrics: &mut Metrics,
    ) {
    }

    /// Returns the information about the chat task.
    fn info(&self) -> &'static TaskInfo {
        &CHAT_TASK_INFO
    }
}
