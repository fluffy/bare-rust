use crate::msg::Msg;

#[cfg(feature = "std")]
extern crate std;

extern crate dev;

pub struct TaskInfo {
    pub run_every_ms: u32,
    pub run_offset_ms: u32,
    pub time_budget_us: u32,
    pub mem_budget_bytes: u32,
}

pub trait Task {
    fn run(&self, msg: &Msg, sender: &mut crate::v_mpsc::Sender, bsp: &mut dev::BSP);
    
    fn info(&self) -> &'static TaskInfo;
}

const MAX_TASKS: usize = 10;

pub struct TaskMgr<'a> {
    tasks: [&'a dyn Task; MAX_TASKS],
    num_tasks: usize,
    sender: &'a mut crate::v_mpsc::Sender,
    bsp: &'a mut dev::BSP,
}

impl<'a> TaskMgr<'a> {
    pub fn new(s: &'a mut crate::v_mpsc::Sender, bsp_in: &'a mut dev::BSP) -> TaskMgr<'a> {
        TaskMgr {
            tasks: [&NO_TASK; MAX_TASKS],
            num_tasks: 0,
            sender: s,
            bsp: bsp_in,
        }
    }

    pub fn add_task(&mut self, task: &'a dyn Task) {
        if self.num_tasks >= MAX_TASKS {
            panic!("Too many tasks");
        }
        self.tasks[self.num_tasks] = task;
        self.num_tasks += 1;
    }

    pub fn run(&mut self) {
        for i in 0..self.num_tasks {
            let t = self.tasks[i];
            let msg = Msg::None;
            t.run( &msg, self.sender, self.bsp);
        }
    }
}

pub struct NoTask {}

const NO_TASK: NoTask = NoTask {};

impl Task for NoTask {
    fn run(&self, _msg: &Msg, _sender: &mut crate::v_mpsc::Sender, _bsp: &mut dev::BSP) {
        panic!("NoTask should never run");
    }

    fn info(&self) -> &'static TaskInfo {
        panic!("NoTask should never be used");
    }
}

pub struct ButtonTask {
    //pub prev_state: bool,
}

const BUTTON_TASK_INFO: TaskInfo = TaskInfo {
    run_every_ms: 100,
    run_offset_ms: 5,
    time_budget_us: 100,
    mem_budget_bytes: 50,
};

impl Task for ButtonTask {
    fn run(&self, _incoming_msg: &Msg, sender: &mut crate::v_mpsc::Sender, bsp: &mut dev::BSP) {
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
