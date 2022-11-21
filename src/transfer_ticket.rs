use crate::types::{Bytes, TimestampMs};

#[derive(Debug)]
pub struct TransferTicket {
    pub(crate) timestamp_start: TimestampMs,
    pub(crate) amount_of_bytes: Bytes
}