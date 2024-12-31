use crate::tasks::MAX_TASKS;

pub struct Metrics {
    pub task_run_count: [u32; MAX_TASKS],
    pub task_max_stack: [u32; MAX_TASKS],
    pub task_max_duration_us: [u32; MAX_TASKS],
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            task_run_count: [0; MAX_TASKS],

            task_max_stack: [0; MAX_TASKS],
            task_max_duration_us: [0; MAX_TASKS],
        }
    }
}
