#[cfg(feature = "std")]
extern crate std;

extern crate dev;

pub trait Task {
    fn run(&self, sender: &mut crate::v_mpsc::Sender, bsp: &mut dev::BSP);
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
            t.run(self.sender, self.bsp);
        }
    }
}

pub struct NoTask {}

const NO_TASK: NoTask = NoTask {};

impl Task for NoTask {
    fn run(&self, _sender: &mut crate::v_mpsc::Sender, _bsp: &mut dev::BSP) {
        panic!("NoTask should never run");
    }
}

pub struct ButtonTask {
    //pub prev_state: bool,
}

impl Task for ButtonTask {
    fn run(&self, sender: &mut crate::v_mpsc::Sender, bsp: &mut dev::BSP) {
        // junk sender.send(crate::msg::Msg::None );
        let (state, changed) = bsp.button.read_ptt();
        if changed {
            sender.send(crate::msg::Msg::PttButton(state));
        }
    }
}
