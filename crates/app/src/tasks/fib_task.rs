use super::Task;
use crate::msg::Msg;
use crate::tasks::TaskInfo;
use crate::metrics::Metrics;

pub struct FibTask {}

const FIB_TASK_INFO: TaskInfo = TaskInfo {
    name: "fibTask",
    run_every_us: 5_000_000,
    time_budget_us: 2_000_000,
    mem_budget_bytes: 1000,
};

impl Task for FibTask {
    fn run(&self, _msg: &Msg, _sender: &mut crate::v_mpsc::Sender<Msg>, 
           _bsp: &mut dev::BSP,
           _metrics: &mut Metrics) {
       fib(34);
    }

    #[allow(dead_code)]
    fn info(&self) -> &'static TaskInfo {
        &FIB_TASK_INFO
    }
}

fn fib(x: usize) -> u32 {
    if x > 2 {
        crate::fib(x - 1) + crate::fib(x - 2)
    } else {
        1
    }
}