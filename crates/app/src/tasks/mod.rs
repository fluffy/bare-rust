pub mod buttons_task;
mod no_task;

use crate::msg::Msg;
use crate::stack;

#[cfg(feature = "std")]
extern crate std;

extern crate dev;

pub struct TaskInfo {
    pub name: &'static str,
    pub run_every_ms: u32,
    pub run_offset_ms: u32,
    pub time_budget_us: u32,
    pub mem_budget_bytes: u32,
}

pub trait Task {
    fn run(&self, msg: &Msg, sender: &mut crate::v_mpsc::Sender<Msg>, bsp: &mut dev::BSP);

    fn info(&self) -> &'static TaskInfo;
}

const MAX_TASKS: usize = 10;

pub struct TaskMgr<'a> {
    tasks: [&'a dyn Task; MAX_TASKS],
    last_run: [hal::timer::MicroSeconds; MAX_TASKS],
    num_tasks: usize,
    sender: &'a mut crate::v_mpsc::Sender<Msg>,
    bsp: &'a mut dev::BSP,
}

const NO_TASK: no_task::NoTask = no_task::NoTask {};

impl<'a> TaskMgr<'a> {
    pub fn new(s: &'a mut crate::v_mpsc::Sender<Msg>, bsp_in: &'a mut dev::BSP) -> TaskMgr<'a> {
        TaskMgr {
            tasks: [&NO_TASK; MAX_TASKS],
            last_run: [hal::timer::MicroSeconds(0); MAX_TASKS],
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
            let info = t.info();

            let start_time = hal::timer::current_time();
            let msg = Msg::None;
            t.run(&msg, self.sender, self.bsp);
            let end_time = hal::timer::current_time();
            
            self.last_run[i] = start_time;

            let duration = end_time.sub(start_time);
            if duration.as_u64() > info.time_budget_us as u64 {
                panic!("Task {} overran time budget", info.name);
            }

            let stack_usage = stack::usage() as u32;
            if stack_usage > info.mem_budget_bytes {
                panic!("Task {} overran memory budget", info.name);
            }
            
            let duration = end_time.sub(start_time);
        }
    }
}
