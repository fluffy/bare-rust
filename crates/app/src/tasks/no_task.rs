use super::Task;
use crate::msg::Msg;
use crate::tasks::TaskInfo;
use crate::metrics::Metrics;

pub struct NoTask {}

const NO_TASK_INFO: TaskInfo = TaskInfo {
    name: "NoTask",
    run_every_us: 0,
    time_budget_us: 0,
    mem_budget_bytes: 0,
};

impl Task for NoTask {
    fn run(&self, _msg: &Msg, _sender: &mut crate::v_mpsc::Sender<Msg>, 
           _bsp: &mut dev::BSP, _metrics: &mut Metrics) {
        panic!("NoTask should never run");
    }

    #[allow(dead_code)]
    fn info(&self) -> &'static TaskInfo {
        &NO_TASK_INFO
    }
}
