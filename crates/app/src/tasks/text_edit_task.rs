//! Task to key pess events and edit input text

extern crate heapless;

use super::{Task, TaskData};
use crate::metrics::Metrics;
use crate::msg::Msg;
use crate::tasks::TaskInfo;

/// Structure representing the textEdit task.
pub struct TextEditTask {
    //pub buffer: heapless::Vec<u8, 160>,
}

pub struct Data {
    pub buffer: heapless::Vec<u8, 160>,
}

impl Data {
    /// Creates a new `Data` instance with an empty buffer.
    pub fn new() -> Self {
        Data {
            buffer: heapless::Vec::new(),
        }
    }
}

/// Information about the textEdit task.
const TEXTEDIT_TASK_INFO: TaskInfo = TaskInfo {
    name: "TextEdit",
    run_every_us: 100_000,
    time_budget_us: 10_000,
    mem_budget_bytes: 300,
};

impl Task for TextEditTask {
    /// Method to execute the textEdit task.
    /// Reads the state of the textEdit and sends a message if the state has changed.
    fn run(
        &self,
        incoming_msg: &Msg,
        sender: &mut crate::mpsc::Sender<Msg>,
        _bsp: &mut bsp::BSP,
        task_data: &mut TaskData,
        _metrics: &mut Metrics,
    ) {
        let data = &mut task_data.text_edit;

        match incoming_msg {
            Msg::Keyboard { key } => {
                // Handle the keyboard message here
                if key == &'\r' {
                    // Send the input message
                    let mut text_msg = Msg::TextInput {
                        input_len: data.buffer.len() as u32,
                        input: [0; 160],
                    };

                    match text_msg {
                        Msg::TextInput { ref mut input, .. } => {
                            input[..data.buffer.len()].copy_from_slice(&data.buffer);
                        }
                        _ => {}
                    }

                    sender.send(text_msg);

                    // Clear the buffer
                    data.buffer.clear();
                }
                // For example, you can add the key to the buffer
                if data.buffer.len() < data.buffer.capacity() {
                    data.buffer.push(*key as u8).unwrap();
                }
            }
            _ => {
                // Handle other messages if necessary
            }
        }
    }

    /// Returns the information about the textEdit task.
    fn info(&self) -> &'static TaskInfo {
        &TEXTEDIT_TASK_INFO
    }
}
