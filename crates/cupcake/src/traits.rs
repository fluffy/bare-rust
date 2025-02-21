#[derive(Copy, Clone, PartialEq)]
pub enum ButtonId {
    A,
    B,
}

#[derive(Copy, Clone)]
pub enum SignalId {
    GreenLedTimer,
}

pub enum ButtonState {
    Down,
    Up,
}

pub enum BoardEvent {
    Button(ButtonId, ButtonState),
    Signal(SignalId),
}

#[allow(dead_code)]
pub trait GpioOutput {
    fn set_low(&mut self);
    fn set_high(&mut self);
    fn is_set_low(&self) -> bool;
    fn is_set_high(&self) -> bool;

    fn toggle(&mut self) {
        if self.is_set_low() {
            self.set_high()
        } else {
            self.set_low()
        }
    }
}

pub trait GpioInput {
    fn is_low(&self) -> bool;
}

pub trait TimerReset {
    fn reset(&mut self);
}

pub trait Sender<T> {
    fn try_send(&self, t: T) -> Result<(), T>;
}

pub trait Receiver<T> {
    fn try_recv(&self) -> Option<T>;
}

pub trait Source<S> {
    fn connect(&mut self, s: S);
}

pub trait Button<S>: Source<S>
where
    S: Sender<BoardEvent>,
{
    fn scan(&mut self);
}

pub trait Signal<S>: Source<S>
where
    S: Sender<BoardEvent>,
{
    fn fire(&mut self);
}

pub trait Board {
    type EventSender: Sender<BoardEvent>;
    type EventReceiver: Receiver<BoardEvent>;

    type ButtonA: Button<Self::EventSender>;
    type ButtonB: Button<Self::EventSender>;
    type GreenLedSignal: Signal<Self::EventSender>;
    type ScanInputsTimer: TimerReset;
    type GreenLedTimer: TimerReset;

    type RedLed: GpioOutput;
    type GreenLed: GpioOutput;
    type BlueLed: GpioOutput;

    fn events(&mut self) -> Self::EventReceiver;

    fn button_a(&mut self) -> Self::ButtonA;
    fn button_b(&mut self) -> Self::ButtonB;
    fn green_led_signal(&mut self) -> Self::GreenLedSignal;
    fn scan_inputs_timer(&mut self) -> Self::ScanInputsTimer;
    fn green_led_timer(&mut self) -> Self::GreenLedTimer;

    fn red_led(&mut self) -> Self::RedLed;
    fn green_led(&mut self) -> Self::GreenLed;
    fn blue_led(&mut self) -> Self::BlueLed;
}

pub type EventReceiver<B> = <B as Board>::EventReceiver;
pub type ButtonA<B> = <B as Board>::ButtonA;
pub type ButtonB<B> = <B as Board>::ButtonB;
pub type GreenLedSignal<B> = <B as Board>::GreenLedSignal;
pub type ScanInputsTimer<B> = <B as Board>::ScanInputsTimer;
pub type GreenLedTimer<B> = <B as Board>::GreenLedTimer;
pub type RedLed<B> = <B as Board>::RedLed;
pub type GreenLed<B> = <B as Board>::GreenLed;
pub type BlueLed<B> = <B as Board>::BlueLed;
