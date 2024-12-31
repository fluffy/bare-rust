use super::Task;
use crate::msg::Msg;
use crate::tasks::TaskInfo;

pub struct ButtonTask {
    //pub prev_state: bool,
}

const BUTTON_TASK_INFO: TaskInfo = TaskInfo {
    name: "Button",
    run_every_ms: 100,
    time_budget_us: 5000,
    mem_budget_bytes: 500,
};

impl Task for ButtonTask {
    fn run(
        &self,
        _incoming_msg: &Msg,
        sender: &mut crate::v_mpsc::Sender<Msg>,
        bsp: &mut dev::BSP,
    ) {
        // junk sender.send(crate::msg::Msg::None );
        let (state, changed) = bsp.button.read_ptt();
        if changed {
            sender.send(crate::msg::Msg::PttButton(state));
        }
    }

    fn info(&self) -> &'static TaskInfo {
        &BUTTON_TASK_INFO
    }
}