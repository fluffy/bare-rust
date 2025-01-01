//! # Channel Module
//!
//! This module provides a asynchronous channel, with multiple producers and single consumer.
//! It follows the standard Rust `std::sync::mpsc` API.
//!
//! It includes functionality for creating channels, sending messages, and receiving messages.
//!
//! ## Structs
//!
//! - `Sender`: Represents the sending side of a message channel.
//! - `Receiver`: Represents the receiving side of a message channel.
//!
//! ## Functions
//!
//! - `channel`: Creates a new message channel and returns a tuple containing the sender and receiver.
//!
//! ## Usage
//!
//! To use this module, create a channel using the `channel` function,
//! which returns a `Sender` and `Receiver`.
//! You can then use the `send` method on the `Sender` to send messages
//! and the `recv` method on the `Receiver` to receive messages.
//!
//! ## Example
//!
//! ```rust
//! use crate::channel::v_mpsc;
//! use crate::msg::Msg;
//!
//! let (mut sender, receiver): (v_mpsc::Sender<Msg>, v_mpsc::Receiver<Msg>) = v_mpsc::channel();
//!
//! sender.send(Msg::PttButton(true));
//!
//! loop {
//!     let msg = receiver.recv();
//!     if msg == Msg::None {
//!         break;
//!     }
//!     match msg {
//!         Msg::None => println!("None"),
//!         Msg::PttButton(state) => println!("PttButton: {}", state),
//!     }
//! }
//! ```
use crate::msg::Msg;

pub mod v_mpsc {
    use super::Msg;

    const Q_SIZE: usize = 10;
    const NUM_QUEUES: usize = 2;

    static mut Q: [[Msg; Q_SIZE]; NUM_QUEUES] = [[Msg::None; Q_SIZE]; NUM_QUEUES];
    static mut Q_LEN: [usize; NUM_QUEUES] = [0; NUM_QUEUES];
    static mut NUM_Q: usize = 0;

    /// A sender for a message channel.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Sender<T> {
        ch: usize,
        _marker: *const T,
    }

    impl<T> Sender<T> {
        /// Sends a message to the channel.
        ///
        /// # Arguments
        ///
        /// * `msg` - The message to send.
        ///
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

    /// A receiver for a message channel.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Receiver<T> {
        ch: usize,
        _marker: *const T,
    }

    impl<T> Receiver<T> {
        /// Receives a message from the channel.
        ///
        /// # Returns
        ///
        /// * `Msg` - The received message.
        ///            If the queue is empty, returns `Msg::None`.
        ///
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

    /// Creates a new message channel.
    ///
    /// # Returns
    ///
    /// * `(Sender<T>, Receiver<T>)` - A tuple containing the sender and receiver for the channel.
    ///
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

#[cfg(feature = "std")]
#[test]
pub fn test_channel() {
    let (mut tx, rx): (v_mpsc::Sender<Msg>, v_mpsc::Receiver<Msg>) = v_mpsc::channel();

    //tx.send(Msg::AiButton(true));
    tx.send(Msg::PttButton(false));
    //tx.send(Msg::Keyboard { key: 'a' });
    //tx.send(Msg::MoqObject {
    //    name: 1,
    //    group: 2,
    //    id: 3,
    //});
    //tx.send(Msg::Shutdown);

    loop {
        let msg = rx.recv();
        if msg == Msg::None {
            break;
        }
        match msg {
            Msg::None => std::println!("None"),
            //Msg::Shutdown => std::println!("Shutdown"),
            //Msg::AiButton(b) => std::println!("AiButton: {}", b),
            Msg::PttButton(b) => std::println!("PttButton: {}", b),
            //Msg::Keyboard { key } => std::println!("Keyboard: {}", key),
            //Msg::MoqObject { name, group, id } => {
            //    std::println!("MoqObject: {} {} {}", name, group, id)
            //}
        }
    }
}
