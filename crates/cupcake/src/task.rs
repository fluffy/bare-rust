use heapless::{mpmc::Q32, Vec};

use crate::traits::*;

pub enum SoftwareEvent {
    // TODO
}

pub enum Event {
    Board(BoardEvent),
    Software(SoftwareEvent),
}

pub trait Task {
    fn can_handle(&self, event: &Event) -> bool;
    fn handle(&mut self, event: &Event, events_out: &dyn Sender<SoftwareEvent>);
}

pub struct BlinkTask<P>
where
    P: GpioOutput,
{
    led: P,
}

impl<P> BlinkTask<P>
where
    P: GpioOutput,
{
    pub fn new(led: P) -> Self {
        Self { led }
    }
}

impl<P> Task for BlinkTask<P>
where
    P: GpioOutput,
{
    fn can_handle(&self, event: &Event) -> bool {
        matches!(
            event,
            Event::Board(BoardEvent::Signal(SignalId::GreenLedTimer))
        )
    }

    fn handle(&mut self, _event: &Event, _events_out: &dyn Sender<SoftwareEvent>) {
        self.led.toggle();
    }
}

pub struct ButtonTask<P>
where
    P: GpioOutput,
{
    button_id: ButtonId,
    led: P,
}

impl<P> ButtonTask<P>
where
    P: GpioOutput,
{
    pub fn new(button_id: ButtonId, led: P) -> Self {
        Self { button_id, led }
    }
}

impl<P> Task for ButtonTask<P>
where
    P: GpioOutput,
{
    fn can_handle(&self, event: &Event) -> bool {
        let Event::Board(BoardEvent::Button(id, _)) = event else {
            return false;
        };

        *id == self.button_id
    }

    fn handle(&mut self, event: &Event, _events_out: &dyn Sender<SoftwareEvent>) {
        let Event::Board(BoardEvent::Button(id, state)) = event else {
            return;
        };

        if *id != self.button_id {
            return;
        }

        match state {
            ButtonState::Down => self.led.set_low(),
            ButtonState::Up => self.led.set_high(),
        }
    }
}

const MAX_TASKS: usize = 8;

impl Sender<SoftwareEvent> for Q32<SoftwareEvent> {
    fn try_send(&self, event: SoftwareEvent) -> Result<(), SoftwareEvent> {
        self.enqueue(event)
    }
}

pub struct TaskMaster<'a, E>
where
    E: Receiver<BoardEvent>,
{
    tasks: Vec<&'a mut dyn Task, { MAX_TASKS }>,
    board_events: E,
    software_events: Q32<SoftwareEvent>,
}

impl<'a, E> TaskMaster<'a, E>
where
    E: Receiver<BoardEvent>,
{
    pub fn new(board_events: E) -> Self {
        Self {
            tasks: Vec::new(),
            board_events,
            software_events: Q32::new(),
        }
    }

    pub fn add_task(&mut self, task: &'a mut dyn Task) {
        let Ok(()) = self.tasks.push(task) else {
            panic!("task list overflow");
        };
    }

    fn next_event(&mut self) -> Option<Event> {
        self.board_events
            .try_recv()
            .map(|e| Event::Board(e))
            .or_else(|| self.software_events.dequeue().map(|e| Event::Software(e)))
    }

    pub fn run(&mut self) {
        // Identify the next event to process
        let Some(event) = self.next_event() else {
            return;
        };

        // Have all tasks that can handle the event handle it
        for t in self.tasks.iter_mut().filter(|t| t.can_handle(&event)) {
            t.handle(&event, &self.software_events);
        }
    }
}
