use super::{Task, TaskData};
use crate::metrics::Metrics;
use crate::msg::Msg;
use crate::tasks::TaskInfo;
use bsp::board;

/// Structure representing the render task.
pub struct RenderTask {}

const FONT_WIDTH: usize = 11;
const FONT_HEIGHT: usize = 16;
const DISPLAY_WIDTH: usize = board::info::DISP_NUM_COLS;
const DISPLAY_HEIGHT: usize =  board::info::DISP_NUM_ROWS;
const DISPLAY_BAND_HEIGHT: usize = 320 / 10;
const TEXT_ROWS: usize = DISPLAY_HEIGHT / FONT_HEIGHT;
const TEXT_COLS: usize = DISPLAY_WIDTH / FONT_WIDTH;

pub struct Data {
    text: [[u8; TEXT_COLS ]; TEXT_ROWS ],
    dirty: [bool; TEXT_ROWS ],

    bitmap: [u16; DISPLAY_WIDTH * DISPLAY_BAND_HEIGHT ],
    current_band: usize,
    
    junk: u32,
}

impl Data {
    /// Creates a new `Data` instance with an empty buffer.
    pub const fn new() -> Self {
        Data {
            text: [[0; TEXT_COLS ]; TEXT_ROWS ],
            dirty: [true; TEXT_ROWS ],
            bitmap: [0; (DISPLAY_WIDTH * DISPLAY_BAND_HEIGHT) ],
            current_band: 0,
            junk: 0 
        }
    }
}

/// Information about the render task.
const RENDER_TASK_INFO: TaskInfo = TaskInfo {
    name: b"Render__",
    run_every_us: 100_000,
    time_budget_us: 100_000,
    mem_budget_bytes: 500,
};

pub fn recv(
    msg: &Msg,
    _sender: &mut crate::mpsc::Sender<Msg>,
    _bsp: &mut bsp::BSP,
    task_data: &mut TaskData,
    _metrics: &mut Metrics,
) {
    let data = &mut task_data.render;

    match msg {
        Msg::PrintMsg { text } => {
            assert!(TEXT_ROWS > 3);
            // scroll the text up
            for r  in 0..TEXT_ROWS  - 3 {
                for c  in 0..TEXT_COLS  {
                    data.text[r][c] = data.text[r + 1][c];
                }
                data.dirty[r] = true;
            }

            // add new line
            let row = TEXT_ROWS - 2;
            let mut num_cols = text.len();
            if num_cols > TEXT_COLS {
                num_cols = TEXT_COLS;
            }
            for col in 0..num_cols {
                data.text[row ][col ] = text[col ];
            }
            for col in num_cols..TEXT_COLS {
                data.text[row ][col ] = b' ';
            }
            data.dirty[row] = true;
        }
        Msg::PrintInputMsg { text } => {
            assert!(TEXT_ROWS > 1);
            let row = TEXT_ROWS - 1;
            let mut num_cols = text.len();
            if num_cols > TEXT_COLS {
                num_cols = TEXT_COLS;
            }
            for col in 0..num_cols {
                data.text[row][col] = text[col];
            }
            for col in num_cols..TEXT_COLS {
                data.text[row][col] = b' ';
            }
            data.dirty[row] = true;
        }
        Msg::PrintClearMsg {} => {
            for r in 0..TEXT_ROWS - 1 {
                for c in 0..TEXT_COLS {
                    data.text[r][c] = b' ';
                }
                data.dirty[r] = true;
            }
        }
        Msg::PrintClearInputMsg {} => {
            let r = TEXT_ROWS - 1;
            for c in 0..TEXT_COLS {
                data.text[r][c] = b' ';
            }
            data.dirty[r] = true;
        }
        _ => {
            // Handle other messages if necessary
        }
    }
}

impl Task for RenderTask {
    /// Method to execute the render task.
    /// Reads the state of the render and sends a message if the state has changed.
    fn run(
        &self,
        _sender: &mut crate::mpsc::Sender<Msg>,
        bsp: &mut bsp::BSP,
        task_data: &mut TaskData,
        _metrics: &mut Metrics,
    ) {
        let data = &mut task_data.render;

        if data.current_band == 0 {
            data.current_band = DISPLAY_HEIGHT / DISPLAY_BAND_HEIGHT - 1;
        } else {
            data.current_band -= 1;
        }

        data.junk += 32;
        if data.junk > 200 {
            data.junk = 0;
        }
        
        let red :u16 = (255 - data.junk as u16)  >> ( 8-5);
        let green:u16 = 0 >> ( 8-6);
        let blue:u16 = 0 >> ( 8-5);
        let color:u16 = (red << 11) | (green << 5) | blue;
        
        
        for y in 0..DISPLAY_BAND_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                data.bitmap[y * DISPLAY_WIDTH + x] = color; // red 
            }
        }

        bsp.display.draw_bitmap(
            &data.bitmap,
            0,                                       // x
            data.current_band * DISPLAY_BAND_HEIGHT, //y
            DISPLAY_WIDTH,
            DISPLAY_BAND_HEIGHT
        );
    }

    /// Returns the information about the render task.
    fn info(&self) -> &'static TaskInfo {
        &RENDER_TASK_INFO
    }
}
