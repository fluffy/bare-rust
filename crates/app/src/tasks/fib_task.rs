/// The `fib_task` module contains the implementation of task to compute
/// a  Fibonacci number.
/// This is a bogus task just to test the task manager can
/// deal with memory and cpu usage budgets
///
use super::Task;
use crate::metrics::Metrics;
use crate::msg::Msg;
use crate::tasks::TaskInfo;

/// Structure representing the Fibonacci task.
pub struct FibTask {}

/// Information about the Fibonacci task.
const FIB_TASK_INFO: TaskInfo = TaskInfo {
    name: "fibTask",
    run_every_us: 5_000_000,
    time_budget_us: 2_000_000,
    mem_budget_bytes: 1000,
};

impl Task for FibTask {
    /// Method to execute the Fibonacci task.
    /// Calculates the Fibonacci sequence up to the 34th number.
    fn run(
        &self,
        _msg: &Msg,
        _sender: &mut crate::v_mpsc::Sender<Msg>,
        _bsp: &mut dev::BSP,
        _metrics: &mut Metrics,
    ) {
        fib(34);
    }

    /// Returns the information about the Fibonacci task.
    #[allow(dead_code)]
    fn info(&self) -> &'static TaskInfo {
        &FIB_TASK_INFO
    }
}

/// Function to calculate the Fibonacci sequence.
/// Returns the Fibonacci number at position `x`.
fn fib(x: usize) -> u32 {
    if x > 2 {
        crate::fib(x - 1) + crate::fib(x - 2)
    } else {
        1
    }
}
