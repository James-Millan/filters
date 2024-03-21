#[derive(Debug)]
pub struct QuotientInfo {
    pub(crate) is_occupied: bool,
    pub(crate) is_shifted: bool,
    pub(crate) is_continuation: bool
}

impl QuotientInfo {
    pub(crate) fn new() -> QuotientInfo {
        return QuotientInfo {
            is_occupied: false,
            is_shifted: false,
            is_continuation: false
        }
    }
}