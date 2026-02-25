#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MazieNum {
    pub(crate) value: f64,
    pub(crate) runtime_id: u64,
}

impl MazieNum {
    pub fn unwrap(self) -> f64 {
        self.value
    }

    pub fn runtime_id(self) -> u64 {
        self.runtime_id
    }
}
