use crate::types::{Bytes, TimestampMs};

pub struct TransferTicket {
    pub(crate) timestamp_start: TimestampMs,
    pub(crate) amount_of_bytes: Bytes
}