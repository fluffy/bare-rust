use super::{Task, TaskData};
use crate::metrics::Metrics;
use crate::msg::Msg;
use crate::tasks::TaskInfo;

/// Structure representing the crypto task.
pub struct CryptoTask {}

pub struct Data {
    key_id: u32,
}

impl Data {
    /// Creates a new `Data` instance with an empty buffer.
    pub const fn new() -> Self {
        Data { key_id: 321 }
    }
}

/// Information about the crypto task.
const CRYPTO_TASK_INFO: TaskInfo = TaskInfo {
    name: b"Crypto__",
    run_every_us: 100_000,
    time_budget_us: 10_000,
    mem_budget_bytes: 500,
};

impl Task for CryptoTask {
    /// Method to execute the crypto task.
    /// Reads the state of the crypto and sends a message if the state has changed.
    fn run(
        &self,
        _incoming_msg: &Msg,
        _sender: &mut crate::mpsc::Sender<Msg>,
        _bsp: &mut bsp::BSP,
        _task_data: &mut TaskData,
        _metrics: &mut Metrics,
    ) {
    }

    /// Returns the information about the crypto task.
    fn info(&self) -> &'static TaskInfo {
        &CRYPTO_TASK_INFO
    }
}

pub fn recv(
    msg: &Msg,
    sender: &mut crate::mpsc::Sender<Msg>,
    _bsp: &mut bsp::BSP,
    task_data: &mut TaskData,
    _metrics: &mut Metrics,
) {
    let data = &mut task_data.crypto;

    match msg {
        Msg::TxtMsg {
            object_id,
            group_id,
            track_alias,
            text,
        } => {
            let msg = Msg::EncTxtMsg {
                object_id: *object_id,
                group_id: *group_id,
                track_alias: *track_alias,
                key_id: data.key_id,
                enc_text: text.clone(),
            };

            sender.send(msg);
        }
        _ => {}
    }
}
