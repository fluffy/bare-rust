use super::Task;
use crate::metrics::Metrics;
use crate::msg::Msg;
use crate::tasks::TaskInfo;

use dev::console::Print;

use crate::tasks::MAX_TASKS;

pub struct MetricsTask {}

const METRICS_TASK_INFO: TaskInfo = TaskInfo {
    name: "MetricsTask",
    run_every_us: 10_000_000,
    time_budget_us: 2_000_000,
    mem_budget_bytes: 100,
};

impl Task for MetricsTask {
    fn run(
        &self,
        _msg: &Msg,
        _sender: &mut crate::v_mpsc::Sender<Msg>,
        _bsp: &mut dev::BSP,
        metrics: &mut Metrics,
    ) {
        b"\r\n\r\n".print_console();

        for i in 0..MAX_TASKS {
            if metrics.task_run_count[i] == 0 {
                continue;
            }

            b"Task ".print_console();
            (i as u64).print_console();
            //metrics.task_name.print_console();
            b": ".print_console();
            metrics.task_run_count[i].print_console();
            b" runs, ".print_console();
            metrics.task_max_stack[i].print_console();
            b" bytes, ".print_console();
            metrics.task_max_duration_us[i].print_console();
            b" uS\r\n".print_console();

            if true {
                metrics.task_run_count[i] = 0;
                metrics.task_max_stack[i] = 0;
                metrics.task_max_duration_us[i] = 0;
            }
        }
    }

    #[allow(dead_code)]
    fn info(&self) -> &'static TaskInfo {
        &METRICS_TASK_INFO
    }
}
