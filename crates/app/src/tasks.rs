#[cfg(feature = "std")]
extern crate std;

pub trait Task {
    fn run(&self, sender: &mut crate::v_mpsc::Sender);
}

const MAX_TASKS: usize = 10;

pub struct TaskMgr<'a> {
    tasks: [&'a dyn Task; MAX_TASKS],
    num_tasks: usize,
    sender: &'a mut crate::v_mpsc::Sender,
}

impl<'a> TaskMgr<'a> {
    pub fn new(s: &'a mut crate::v_mpsc::Sender) -> TaskMgr<'a> {
        TaskMgr {
            tasks: [ &NO_TASK; MAX_TASKS],
            num_tasks: 0,
            sender: s,
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
        for i in 0..MAX_TASKS {
            let t = self.tasks[i];
                 t.run(self.sender);
        }
    }
}

pub struct NoTask {}

const NO_TASK: NoTask = NoTask {};

impl Task for NoTask {
    fn run(&self, _sender: &mut crate::v_mpsc::Sender) {
        panic!("Not implemented");
    }
}

pub struct ButtonTask {
    //pub prev_state: bool,
}

impl Task for ButtonTask {
    fn run(&self, sender: &mut crate::v_mpsc::Sender) {
        // junk sender.send(crate::msg::Msg::None );
       // let state = dev::button::read_ptt();
       // if state != self.prev_state {
       //     sender.send(crate::msg::Msg::PttButton(state));
       //     //self.prev_state = state;
       // }
    }
}
