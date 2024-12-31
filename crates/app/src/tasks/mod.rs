mod no_task;
pub mod fib_task;
pub mod metrics_task;
pub mod buttons_task;

use crate::metrics::Metrics;

use dev::console::Print;
use crate::msg::Msg;
use crate::stack;

#[cfg(feature = "std")]
extern crate std;

extern crate dev;

pub struct TaskInfo {
    pub name: &'static str,
    pub run_every_us: u32,
    pub time_budget_us: u32,
    pub mem_budget_bytes: u32,
}

pub trait Task {
    fn run(&self, msg: &Msg, sender: &mut crate::v_mpsc::Sender<Msg>,
           bsp: &mut dev::BSP,
           metrics: &mut Metrics);

    fn info(&self) -> &'static TaskInfo;
}

pub const MAX_TASKS: usize = 10;

pub struct TaskMgr<'a> {
    tasks: [&'a dyn Task; MAX_TASKS],
    last_run: [hal::timer::MicroSeconds; MAX_TASKS],
    num_tasks: usize,
    sender: &'a mut crate::v_mpsc::Sender<Msg>,
    bsp: &'a mut dev::BSP,
    metrics: &'a mut Metrics,
}

const NO_TASK: no_task::NoTask = no_task::NoTask {};

impl<'a> TaskMgr<'a> {
    
    pub fn new(s: &'a mut crate::v_mpsc::Sender<Msg>, bsp: &'a mut dev::BSP, metrics: &'a mut Metrics) -> TaskMgr<'a> {
        TaskMgr {
            tasks: [&NO_TASK; MAX_TASKS],
            last_run: [hal::timer::MicroSeconds(0); MAX_TASKS],
            num_tasks: 0,
            sender: s,
            bsp: bsp,
            metrics: metrics,
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
        let base_stack_usage = stack::usage() as u32;

        for i in 0..self.num_tasks {
            let t = self.tasks[i];
            let info = t.info();

            let now = hal::timer::current_time();

            if now.sub(self.last_run[i]).as_u64() < info.run_every_us as u64 {
                continue;
            }

            let start_time = hal::timer::current_time();
            let msg = Msg::None;
            t.run(&msg, self.sender, self.bsp,self.metrics);
            let end_time = hal::timer::current_time();
            let end_stack_usage = stack::usage() as u32;

            self.last_run[i] = start_time;

            let duration = end_time.sub(start_time).as_u64();
            if duration > info.time_budget_us as u64 {
                b"Exceededtime budget\r\n".print_console();

                b" start=".print_console();
                start_time.as_u64().print_console();
                b" us\r\n".print_console();

                b" end=".print_console();
                end_time.as_u64().print_console();
                b" us\r\n".print_console();

                b" duration=".print_console();
                duration.print_console();
                b" us\r\n".print_console();
                
                panic!("Task {} overran time budget", info.name);
              }

            let stack_usage = end_stack_usage - base_stack_usage;
            if stack_usage > info.mem_budget_bytes {
                b"Exceeded memorry budget\r\n".print_console();
                panic!("Task {} overran memory budget", info.name);
            }
            
            // Update metrics
            self.metrics.task_run_count[i] += 1;
            if stack_usage > self.metrics.task_max_stack[i] {
                self.metrics.task_max_stack[i] = stack_usage;
            }
            if duration as u32 > self.metrics.task_max_duration_us[i] {
                self.metrics.task_max_duration_us[i] = duration as u32;
            }
        }
    }
}
