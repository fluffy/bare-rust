

#[cfg(feature = "std")]
extern crate std;


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Msg {
    None,
    //Shutdown,
    //AiButton(bool),
    PttButton(bool),
    //Keyboard { key: char },
    //MoqObject { name: u128, group: u64, id: u64 },
}

pub mod v_mpsc {
    use super::Msg;

    const Q_SIZE: usize = 10;
    const NUM_QUEUES: usize = 2;

    static mut Q:[ [Msg; Q_SIZE]; NUM_QUEUES] = [[Msg::None; Q_SIZE]; NUM_QUEUES];
    static mut Q_LEN: [usize; NUM_QUEUES] = [0; NUM_QUEUES];
    static mut NUM_Q: usize = 0;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Sender {
        ch: usize,
    }

    impl Sender {
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
    pub struct Receiver {
        ch: usize,
    }

    impl Receiver {
        pub fn recv(&self) -> Msg {
            let ch = self.ch;
            let q_len = unsafe { Q_LEN[ch] };
            if q_len == 0 {
                return Msg::None;
            }
            let msg = unsafe { Q[ch][0] };
            for i in 0..q_len-1 {
                unsafe { Q[ch][i] = Q[ch][i+1] };
            }
            unsafe { Q_LEN[ch] -= 1 };
            msg
        }
    }

    pub fn channel() -> (Sender, Receiver) {
        let ch = unsafe { NUM_Q };
        unsafe { NUM_Q += 1 };
        if ch >= NUM_QUEUES {
            panic!("Too many channels");
        }

        let sender = Sender { ch };
        let receiver = Receiver { ch };

        (sender, receiver)
    }
}

#[cfg(feature = "std")]
#[test]
pub fn test_msg() {
    let (tx, rx) = v_mpsc::channel();

    tx.send(Msg::AiButton(true));
    tx.send(Msg::PttButton(false));
    tx.send(Msg::Keyboard { key: 'a' });
    tx.send(Msg::MoqObject { name: 1, group: 2, id: 3 });
    tx.send(Msg::Shutdown);

    loop {
        let msg = rx.recv();
        if msg == Msg::None {
            break;
        }
        match msg {
            Msg::None => std::println!("None"),
            Msg::Shutdown => std::println!("Shutdown"),
            Msg::AiButton(b) => std::println!("AiButton: {}", b),
            Msg::PttButton(b) => std::println!("PttButton: {}", b),
            Msg::Keyboard { key } => std::println!("Keyboard: {}", key),
            Msg::MoqObject { name, group, id } => std::println!("MoqObject: {} {} {}", name, group, id),
        }
    }
}
