//! The `metrics_task` module contains the implementation of the metrics task.
//! This task is responsible for periodically printing all metrics to the console
//! and then resetting the metrics.

use super::Task;
use crate::metrics::Metrics;
use crate::msg::Msg;
use crate::tasks::TaskInfo;

use crate::tasks::MAX_TASKS;
use dev::console::Print;

/// Structure representing the metrics task.
pub struct MetricsTask {}

/// Information about the metrics task.
const METRICS_TASK_INFO: TaskInfo = TaskInfo {
    name: "MetricsTask",
    run_every_us: 5_000_000,
    time_budget_us: 2_000_000,
    mem_budget_bytes: 100,
};

impl Task for MetricsTask {
    /// Method to execute the metrics task.
    /// Prints and resets the metrics of all tasks.
    ///
    /// Prints the number of runs, maximum stack usage, and maximum duration of each task.
    /// Then resets the metrics for all tasks.
    fn run(
        &self,
        _msg: &Msg,
        _sender: &mut crate::v_mpsc::Sender<Msg>,
        _bsp: &mut dev::BSP,
        metrics: &mut Metrics,
    ) {
        b"\r\n\r\n".print_console();

        for i in 0..MAX_TASKS {
            if metrics.task_run_count[i] == 0 {
                continue;
            }

            b"Task ".print_console();
            (i as u64).print_console();
            //metrics.task_name.print_console();
            b": ".print_console();
            metrics.task_run_count[i].print_console();
            b" runs, ".print_console();
            metrics.task_max_stack[i].print_console();
            b" bytes, ".print_console();
            metrics.task_max_duration_us[i].print_console();
            b" uS\r\n".print_console();

            if true {
                metrics.task_run_count[i] = 0;
                metrics.task_max_stack[i] = 0;
                metrics.task_max_duration_us[i] = 0;
            }
        }
    }

    /// Returns the information about the metrics task.
    #[allow(dead_code)]
    fn info(&self) -> &'static TaskInfo {
        &METRICS_TASK_INFO
    }
}
