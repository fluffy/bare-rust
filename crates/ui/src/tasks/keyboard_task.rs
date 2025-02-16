//! This module handles the keyboard task for the application.
//!
//! The keyboard task is responsible for reading the state of the keyboard and sending messages
//! when the state changes. The keyboard is interfaced using a matrix of rows and columns.
//!
//! - **Keyboard Columns**: These are configured as output pins with a default state of low (pulled low). There are 5 columns. 
//! - **Keyboard Rows**: These are configured as input pins with a default state of low (pulled down). There are 7 rows.
//!
//! The task periodically scans the columns and checks the state of the rows to detect key presses and releases.
//! When a key press or release is detected, a message is sent to notify other parts of the system.

use super::{Task, TaskData};
use crate::metrics::Metrics;
use crate::msg::Msg;
use crate::tasks::TaskInfo;

/// Structure representing the keyboard task.
pub struct KeyboardTask {}

/// Information about the keyboard task.
const KEYBOARD_TASK_INFO: TaskInfo = TaskInfo {
    name: b"Keyboard",
    run_every_us: 10_000,
    time_budget_us: 10_000,
    mem_budget_bytes: 500,
};

impl Task for KeyboardTask {
    /// Method to execute the keyboard task.
    /// Reads the state of the keyboard and sends a message if the state has changed.
    fn run(
        &self,
        sender: &mut crate::mpsc::Sender<Msg>,
        bsp: &mut bsp::BSP,
        _task_data: &mut TaskData,
        _metrics: &mut Metrics,
    ) {
        // this uses the PTT button to mock the keyboard
        let button_moch_keyboard = true;
        if button_moch_keyboard {
            let (state, changed) = bsp.buttons.read_ptt();
            if changed {
                if state {
                    let keyboard_msg = Msg::Keyboard { key: 'A' };
                    sender.send(keyboard_msg);
                } else {
                    let keyboard_msg = Msg::Keyboard { key: '\r' };
                    sender.send(keyboard_msg);
                }
            }
        }
    }

    /// Returns the information about the keyboard task.
    fn info(&self) -> &'static TaskInfo {
        &KEYBOARD_TASK_INFO
    }
}

const Q10_COLS: usize = 5;
const Q10_ROWS: usize = 7;

const SYM: u8 = 1; // Replace with the actual byte value
const ALT: u8 = 2; // Replace with the actual byte value
const MIC: u8 = 3; // Replace with the actual byte value
const SHF: u8 = 4; // Replace with the actual byte value
const DLR: u8 = 5; // Replace with the actual byte value
const ENT: u8 = 0x0D; // Replace with the actual byte value
const BAK: u8 = 0x08; // Replace with the actual byte value
const SPC: u8 = b' '; // 32

const BASE_CHAR_MAP: [[u8; Q10_ROWS]; Q10_COLS] = [
    [b'Q', b'W', SYM, b'A', ALT, SPC, MIC], // col1
    [b'E', b'S', b'D', b'P', b'X', b'Z', SHF], // col2
    [b'R', b'G', b'T', SHF, b'V', b'C', b'F'], // col3
    [b'U', b'H', b'Y', ENT, b'B', b'N', b'J'], // col4
    [b'O', b'L', b'I', BAK, DLR, b'M', b'K'], // col5
];

const SHIFT_CHAR_MAP: [[u8; Q10_ROWS]; Q10_COLS] = [
    [b'1', b'2', SYM, b'3', ALT, SPC, MIC], // col1
    [b'4', b'5', b'6', b'P', b'X', b'Z', SHF], // col2
    [b'7', b'8', b'9', SHF, b'V', b'C', b'0'], // col3
    [b'-', b'0', b'=', ENT, b'B', b'N', b'/'], // col4
    [b'*', b';', b'(', BAK, DLR, b')', b':'], // col5
];