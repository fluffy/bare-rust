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

#[cfg(feature = "std")]
#[test]
pub fn test_msg() {
    let (tx, rx) = v_mpsc::channel();

    tx.send(Msg::AiButton(true));
    tx.send(Msg::PttButton(false));
    tx.send(Msg::Keyboard { key: 'a' });
    tx.send(Msg::MoqObject {
        name: 1,
        group: 2,
        id: 3,
    });
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
            Msg::MoqObject { name, group, id } => {
                std::println!("MoqObject: {} {} {}", name, group, id)
            }
        }
    }
}
