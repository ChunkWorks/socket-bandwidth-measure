use std::collections::HashMap;
use std::time::SystemTime;

use crate::transfer_ticket::TransferTicket;
use crate::types::{Bytes, TimestampMs};

pub struct BandwidthEntry {
    pub amount_of_bytes: Bytes
}

pub struct BandwidthMeasure {
    start_time: SystemTime,
    entries: HashMap<TimestampMs, Bytes>,
}

impl BandwidthMeasure {
    pub fn new() -> Self {
        return Self {
            start_time: SystemTime::now(),
            entries: HashMap::new()
        }
    }

    fn current_duration(&self) -> TimestampMs {
        return SystemTime::now().duration_since(self.start_time).unwrap().as_millis() as u32
    }

    fn clean_old_entries(&mut self) {
        let current_duration = self.current_duration();
        self.entries.retain(|entry, _| {
            (current_duration - entry) < 1000
        });
    }

    pub fn get_kbps(&mut self) -> f32 {
        self.clean_old_entries();
        let sum = self.entries.values().sum::<Bytes>();
        let result = (sum as f32) / 1000.0;
        return result;
    }

    pub fn add_bytes_start(&self, bytes: usize) -> TransferTicket {
        let current_duration = self.current_duration();
        return TransferTicket {
            timestamp_start: current_duration,
            amount_of_bytes: bytes
        };
    }

    pub fn add_bytes_end(&mut self, ticket: TransferTicket) {
        let current_duration = self.current_duration();
        let total_duration = current_duration - ticket.timestamp_start;
        let bytes_per_second = (((ticket.amount_of_bytes as f32) / (total_duration as f32)) * 1000.0) as usize;
        let current_bytes = self.entries.entry(current_duration).or_default();
        *current_bytes += bytes_per_second;
    }
}