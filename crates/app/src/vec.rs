
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MyVec {
    pub data: [u8;160],
    pub len: usize,
}

impl MyVec {
    pub const fn new() -> Self {
        MyVec {
            data: [0; 160],
            len: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.data.len()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, value: u8) -> Result<(), ()> {
        if self.len < self.data.len() {
            self.data[self.len] = value;
            self.len += 1;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }
}
