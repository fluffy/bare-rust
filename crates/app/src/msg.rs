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
