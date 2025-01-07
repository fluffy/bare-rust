use super::{Task, TaskData};
use crate::metrics::Metrics;
use crate::msg::Msg;
use crate::tasks::TaskInfo;

/// Structure representing the netLink task.
pub struct NetLinkTask {}

/// Information about the netLink task.
const NETLINK_TASK_INFO: TaskInfo = TaskInfo {
    name: b"NetLink_",
    run_every_us: 100_000,
    time_budget_us: 10_000,
    mem_budget_bytes: 500,
};


pub fn recv(
    _msg: &Msg,
    _sender: &mut crate::mpsc::Sender<Msg>,
    _bsp: &mut bsp::BSP,
    _task_data: &mut TaskData,
    _metrics: &mut Metrics,
) {
    //let data = &mut task_data.net_link;
}

impl Task for NetLinkTask {
    /// Method to execute the netLink task.
    /// Reads the state of the netLink and sends a message if the state has changed.
    fn run(
        &self,
        _incoming_msg: &Msg,
        _sender: &mut crate::mpsc::Sender<Msg>,
        _bsp: &mut bsp::BSP,
        _task_data: &mut TaskData,
        _metrics: &mut Metrics,
    ) {
    }

    /// Returns the information about the netLink task.
    fn info(&self) -> &'static TaskInfo {
        &NETLINK_TASK_INFO
    }
}
