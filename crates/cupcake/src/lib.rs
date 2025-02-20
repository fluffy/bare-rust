use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;

struct Peripherals;

impl Peripherals {
    fn take() -> Option<Self> {
        todo!()
    }
}

trait EventSource {
    fn connect(&mut self, sink: Sender<BoardEvent>);
}

struct Timer;

impl Timer {
    fn set_interval(&mut self, _interval: Duration) {
        todo!()
    }
    fn start(&mut self) {
        todo!()
    }
}

impl EventSource for Timer {
    fn connect(&mut self, _sink: Sender<BoardEvent>) {
        todo!()
    }
}

struct Button;

impl EventSource for Button {
    fn connect(&mut self, _sink: Sender<BoardEvent>) {
        todo!()
    }
}

struct Led;

impl Led {
    fn set(&mut self, _lit: bool) {
        todo!()
    }
}

struct Board {
    timer: Timer,
    button_a: Button,
    button_b: Button,
    red_led: Led,
    green_led: Led,
    blue_led: Led,
}

impl Board {
    fn new(_peripherals: Peripherals) -> Self {
        todo!();
    }
}

// Events
#[derive(PartialEq)]
enum ButtonId {
    A,
    B,
}

enum ButtonState {
    Down,
    Up,
}

enum BoardEvent {
    Button(ButtonId, ButtonState),
    Timer,
}

enum SoftwareEvent {
    // TODO
}

enum Event {
    Board(BoardEvent),
    Software(SoftwareEvent),
}

// Tasks
trait Task {
    fn can_handle(&self, event: &Event) -> bool;
    fn handle(&mut self, event: &Event, events_out: &Sender<SoftwareEvent>);
}

struct BlinkTask {
    on: bool,
    led: Led,
}

impl BlinkTask {
    fn new(led: Led) -> Self {
        Self { on: false, led }
    }
}

impl Task for BlinkTask {
    fn can_handle(&self, event: &Event) -> bool {
        matches!(event, Event::Board(BoardEvent::Timer))
    }

    fn handle(&mut self, _event: &Event, _events_out: &Sender<SoftwareEvent>) {
        self.on ^= true;
        self.led.set(self.on);
    }
}

struct ButtonTask {
    button_id: ButtonId,
    led: Led,
}

impl ButtonTask {
    fn new(button_id: ButtonId, led: Led) -> Self {
        Self { button_id, led }
    }
}

impl Task for ButtonTask {
    fn can_handle(&self, event: &Event) -> bool {
        let Event::Board(BoardEvent::Button(id, _)) = event else {
            return false;
        };

        *id == self.button_id
    }

    fn handle(&mut self, event: &Event, _events_out: &Sender<SoftwareEvent>) {
        let Event::Board(BoardEvent::Button(id, state)) = event else {
            return;
        };

        if *id != self.button_id {
            return;
        }

        match state {
            ButtonState::Down => self.led.set(true),
            ButtonState::Up => self.led.set(false),
        }
    }
}

struct TaskMaster<'a> {
    tasks: Vec<&'a mut dyn Task>,
    board_events: Receiver<BoardEvent>,
    software_send: Sender<SoftwareEvent>,
    software_recv: Receiver<SoftwareEvent>,
}

impl<'a> TaskMaster<'a> {
    fn new(board_events: Receiver<BoardEvent>) -> Self {
        let (software_send, software_recv) = channel::<SoftwareEvent>();
        Self {
            tasks: Vec::new(),
            board_events,
            software_send,
            software_recv,
        }
    }

    fn add_task(&mut self, task: &'a mut dyn Task) {
        self.tasks.push(task);
    }

    fn next_event(&mut self) -> Option<Event> {
        self.board_events
            .try_recv()
            .ok()
            .map(|e| Event::Board(e))
            .or_else(|| {
                self.software_recv
                    .try_recv()
                    .ok()
                    .map(|e| Event::Software(e))
            })
    }

    fn run(&mut self) {
        // Identify the next event to process
        let Some(event) = self.next_event() else {
            return;
        };

        // Have all tasks that can handle the event handle it
        let software_send = self.software_send.clone();
        for t in self.tasks.iter_mut().filter(|t| t.can_handle(&event)) {
            t.handle(&event, &software_send);
        }
    }
}

pub fn main() {
    let hal = Peripherals::take().unwrap();
    let mut board = Board::new(hal);

    // Create channels from the board to the app
    let (events_from_board, events_to_tasks) = channel::<BoardEvent>();

    // Wire up events from the board to the app
    board.timer.set_interval(Duration::from_millis(500));
    board.timer.connect(events_from_board.clone());
    board.timer.start();

    board.button_a.connect(events_from_board.clone());
    board.button_b.connect(events_from_board.clone());

    // Configure the task runner
    let mut blink_task = BlinkTask::new(board.green_led);
    let mut button_a_task = ButtonTask::new(ButtonId::A, board.red_led);
    let mut button_b_task = ButtonTask::new(ButtonId::B, board.blue_led);

    let mut task_master = TaskMaster::new(events_to_tasks);
    task_master.add_task(&mut blink_task);
    task_master.add_task(&mut button_a_task);
    task_master.add_task(&mut button_b_task);

    // Run the tasks
    loop {
        task_master.run();
    }
}
