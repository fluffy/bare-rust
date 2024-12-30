
#[cfg(feature = "std")]
extern crate std;

pub trait Task {
    fn run(&self, sender: crate::v_mpsc::Sender);
}

const MAX_TASKS: usize = 10;

pub struct TaskMgr<'a> {
    tasks: [ Option< &'a dyn Task>; MAX_TASKS],
    num_tasks: usize,
    sender: crate::v_mpsc::Sender,
}

impl<'a> TaskMgr<'a> {
    pub fn new( s: crate::v_mpsc::Sender) -> TaskMgr<'a> {
        TaskMgr {
            tasks: [None; MAX_TASKS],
            num_tasks: 0,
            sender: s,
        }
    }

    pub fn add_task(&mut self, task: &'a dyn Task) {
        if self.num_tasks >= MAX_TASKS {
            panic!("Too many tasks");
        }
        self.tasks[self.num_tasks] = Some(task);
        self.num_tasks += 1;
    }

    pub fn run(&mut self ) {
        for i in 0..MAX_TASKS {
            match self.tasks[i] {
                Some(ref mut task) => {
                    //panic!("Not implemented");
                    let t = *task; //as &mut dyn Task;
                    let s= self.sender.clone();
                    //task.run(sender.clone());
                    //let t2 = t as *mut dyn Task;
                    t.run(s);
                },
                None => break,
            }
        }
    }
}

pub struct NoTask {
}
impl Task for NoTask {
    fn run(&self, _sender: crate::v_mpsc::Sender) {
       panic!("Not implemented");
    }
}

pub struct ButtonTask {
    pub prev_state: bool,
}

impl Task for ButtonTask {
    fn run(&self, sender: crate::v_mpsc::Sender) {
        let state = hal::button::read_ptt();
        if state != self.prev_state {
            sender.send(crate::msg::Msg::PttButton(state));
            //self.prev_state = state;
        }
    }
}