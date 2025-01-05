extern crate hal;

#[cfg(feature = "std")]
extern crate std;

pub enum NetlinkMessage {
    None,
}

pub struct Netlink {}

impl crate::netlink::Netlink {
    #[inline(never)]
    pub fn new() -> Self {
        crate::netlink::Netlink {}
    }

    #[inline(never)]
    pub fn init(&self) {}

    pub fn send(&self, _message: NetlinkMessage) {
        // Send a message
    }

    pub fn receive(&self) -> NetlinkMessage {
        NetlinkMessage::None
    }
}
