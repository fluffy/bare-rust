

use super::Task;
use crate::metrics::Metrics;
use crate::msg::Msg;
use crate::tasks::TaskInfo;

/// Structure representing the textEdit task.
pub struct TextEditTask {}

/// Information about the textEdit task.
const TEXTEDIT_TASK_INFO: TaskInfo = TaskInfo {
    name: "TextEdit",
    run_every_us: 100_000,
    time_budget_us: 10_000,
    mem_budget_bytes: 300,
};

impl Task for TextEditTask {
    /// Method to execute the textEdit task.
    /// Reads the state of the textEdit and sends a message if the state has changed.
    fn run(
        &self,
        _incoming_msg: &Msg,
        _sender: &mut crate::mpsc::Sender<Msg>,
        _bsp: &mut bsp::BSP,
        _metrics: &mut Metrics,
    ) {

    }

    /// Returns the information about the textEdit task.
    fn info(&self) -> &'static TaskInfo {
        &TEXTEDIT_TASK_INFO
    }
}
