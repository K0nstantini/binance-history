#[derive(Copy, Clone, Debug)]
pub enum DataInterval { Daily, Monthly }

impl DataInterval {
    pub(crate) fn path(&self) -> &'static str {
        match self {
            DataInterval::Daily => "daily",
            DataInterval::Monthly => "monthly"
        }
    }
}
