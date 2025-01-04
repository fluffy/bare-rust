//! The `tasks` module is responsible for managing and running various tasks in the system.
//! It provides the necessary structures and traits to define tasks, manage their execution,
//! and track their metrics.

pub mod buttons_task;
pub mod fib_task;
pub mod metrics_task;
mod no_task;

use crate::metrics::Metrics;
use crate::msg::Msg;
use crate::stack;
use dev::console::Print;

#[cfg(feature = "std")]
extern crate std;

extern crate dev;

/// Structure containing information about a task.
pub struct TaskInfo {
    /// The name of the task.
    pub name: &'static str,
    /// The interval at which the task should run, in microseconds.
    pub run_every_us: u32,
    /// The maximum allowed execution time for the task, in microseconds.
    pub time_budget_us: u32,
    /// The maximum allowed memory usage for the task, in bytes.
    pub mem_budget_bytes: u32,
}

/// Trait that defines the behavior of a task.
pub trait Task {
    /// Method to execute the task.
    fn run(
        &self,
        msg: &Msg,
        sender: &mut crate::mpsc::Sender<Msg>,
        bsp: &mut dev::BSP,
        metrics: &mut Metrics,
    );

    /// Returns the task information.
    fn info(&self) -> &'static TaskInfo;
}

/// The maximum number of tasks that can be managed by the `TaskMgr`.
pub const MAX_TASKS: usize = 10;

/// Structure that manages the execution of tasks.
pub struct TaskMgr<'a> {
    /// An array of tasks to be managed.
    tasks: [&'a dyn Task; MAX_TASKS],
    /// An array of timestamps indicating the last run time of each task.
    last_run: [hal::timer::MicroSeconds; MAX_TASKS],
    /// The number of tasks currently managed.
    num_tasks: usize,
    /// A message sender for inter-task communication.
    sender: &'a mut crate::mpsc::Sender<Msg>,
    /// A reference to the Board Support Package (BSP).
    bsp: &'a mut dev::BSP,
    /// A reference to the metrics structure for tracking task performance.
    metrics: &'a mut Metrics,
}

/// A placeholder task used when no task is assigned.
const NO_TASK: no_task::NoTask = no_task::NoTask {};

impl<'a> TaskMgr<'a> {
    /// Creates a new `TaskMgr` instance.
    pub fn new(
        s: &'a mut crate::mpsc::Sender<Msg>,
        bsp: &'a mut dev::BSP,
        metrics: &'a mut Metrics,
    ) -> TaskMgr<'a> {
        TaskMgr {
            tasks: [&NO_TASK; MAX_TASKS],
            last_run: [hal::timer::MicroSeconds(0); MAX_TASKS],
            num_tasks: 0,
            sender: s,
            bsp: bsp,
            metrics: metrics,
        }
    }

    /// Adds a task to the `TaskMgr`.
    pub fn add_task(&mut self, task: &'a dyn Task) {
        if self.num_tasks >= MAX_TASKS {
            panic!("Too many tasks");
        }
        self.tasks[self.num_tasks] = task;
        self.num_tasks += 1;
    }

    /// Runs all the tasks managed by the `TaskMgr`, ensuring
    /// they adhere to their time and memory budgets.
    /// The method also updates the task metrics.
    pub fn run(&mut self) {
        stack::usage(true); // reset stack usage
        let base_stack_usage = stack::usage(false) as u32;

        for i in 0..self.num_tasks {
            let t = self.tasks[i];
            let info = t.info();

            let now = hal::timer::current_time();

            if now.sub(self.last_run[i]).as_u64() < info.run_every_us as u64 {
                continue;
            }

            let start_time = hal::timer::current_time();
            let msg = Msg::None;
            t.run(&msg, self.sender, self.bsp, self.metrics);
            let end_time = hal::timer::current_time();
            let end_stack_usage = stack::usage(false) as u32;

            self.last_run[i] = start_time;

            let duration = end_time.sub(start_time).as_u64();
            if duration > info.time_budget_us as u64 {
                b"Exceeded time budget\r\n".print_console();

                b" start=".print_console();
                start_time.as_u64().print_console();
                b" us\r\n".print_console();

                b" end=".print_console();
                end_time.as_u64().print_console();
                b" us\r\n".print_console();

                b" duration=".print_console();
                duration.print_console();
                b" us\r\n".print_console();

                panic!("Task {} overran time budget", info.name);
            }

            let stack_usage = end_stack_usage - base_stack_usage;
            if stack_usage > info.mem_budget_bytes {
                b"Exceeded memory budget\r\n  usage==".print_console();
                (stack_usage as u64).print_console();
                b"\r\n".print_console();
                panic!("Task {} overran memory budget", info.name);
            }

            // Update metrics
            self.metrics.task_run_count[i] += 1;
            if stack_usage > self.metrics.task_max_stack[i] {
                self.metrics.task_max_stack[i] = stack_usage;
            }
            if duration as u32 > self.metrics.task_max_duration_us[i] {
                self.metrics.task_max_duration_us[i] = duration as u32;
            }
        }
    }
}
