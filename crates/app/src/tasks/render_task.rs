

use super::Task;
use crate::metrics::Metrics;
use crate::msg::Msg;
use crate::tasks::TaskInfo;

/// Structure representing the render task.
pub struct RenderTask {}

/// Information about the render task.
const RENDER_TASK_INFO: TaskInfo = TaskInfo {
    name: "Render",
    run_every_us: 100_000,
    time_budget_us: 10_000,
    mem_budget_bytes: 300,
};

impl Task for RenderTask {
    /// Method to execute the render task.
    /// Reads the state of the render and sends a message if the state has changed.
    fn run(
        &self,
        _incoming_msg: &Msg,
        _sender: &mut crate::mpsc::Sender<Msg>,
        _bsp: &mut bsp::BSP,
        _metrics: &mut Metrics,
    ) {

    }

    /// Returns the information about the render task.
    fn info(&self) -> &'static TaskInfo {
        &RENDER_TASK_INFO
    }
}
