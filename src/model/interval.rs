#[derive(Copy, Clone, Debug)]
pub enum DataInterval { Daily, _Monthly }

impl DataInterval {
    pub fn path(&self) -> &'static str {
        match self {
            DataInterval::Daily => "daily/",
            DataInterval::_Monthly => "monthly/"
        }
    }
}
