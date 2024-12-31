//use core::marker::PhantomData;

use crate::msg::Msg;

pub mod v_mpsc {
    use super::Msg;

    const Q_SIZE: usize = 10;
    const NUM_QUEUES: usize = 2;

    static mut Q: [[Msg; Q_SIZE]; NUM_QUEUES] = [[Msg::None; Q_SIZE]; NUM_QUEUES];
    static mut Q_LEN: [usize; NUM_QUEUES] = [0; NUM_QUEUES];
    static mut NUM_Q: usize = 0;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Sender<T> {
        ch: usize,
        _marker: *const T,
        //_marker: core::marker::PhantomData<T>,
    }

    impl<T> Sender<T> {
        pub fn send(&self, msg: Msg) {
            let ch = self.ch;
            let q_len = unsafe { Q_LEN[ch] };
            if q_len >= Q_SIZE {
                panic!("Queue full");
            }
            unsafe { Q[ch][q_len] = msg };
            unsafe { Q_LEN[ch] += 1 };
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Receiver<T> {
        ch: usize,
        _marker: *const T,
        //_marker: core::marker::PhantomData<T>,
    }

    impl<T> Receiver<T> {
        pub fn recv(&self) -> Msg {
            let ch = self.ch;
            let q_len = unsafe { Q_LEN[ch] };
            if q_len == 0 {
                return Msg::None;
            }
            let msg = unsafe { Q[ch][0] };
            for i in 0..q_len - 1 {
                unsafe { Q[ch][i] = Q[ch][i + 1] };
            }
            unsafe { Q_LEN[ch] -= 1 };
            msg
        }
    }

    pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
        let ch = unsafe { NUM_Q };
        unsafe { NUM_Q += 1 };
        if ch >= NUM_QUEUES {
            panic!("Too many channels");
        }

        let sender = Sender {
            ch: ch,
            _marker: core::ptr::null(),
            //_marker: core::marker::PhantomData
        } as Sender<T>;
        let receiver = Receiver {
            ch: ch,
            _marker: core::ptr::null(),
            //_marker: core::marker::PhantomData
        } as Receiver<T>;

        (sender, receiver)
    }
}
