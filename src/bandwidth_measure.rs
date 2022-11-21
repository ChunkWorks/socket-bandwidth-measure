use std::collections::HashMap;
use chrono::Utc;
use log::debug;

use crate::transfer_ticket::TransferTicket;
use crate::types::{Bytes, TimestampMs};

pub fn get_unix_time_ms() -> TimestampMs {
    let now = Utc::now();
    now.timestamp_millis() as TimestampMs
}

#[derive(Clone, Debug)]
pub struct BandwidthMeasure {
    start_time: TimestampMs,
    entries: HashMap<TimestampMs, Bytes>,
}

impl BandwidthMeasure {
    pub fn new() -> Self {
        return Self {
            start_time: get_unix_time_ms(),
            entries: HashMap::new()
        }
    }

    fn current_duration(&self) -> TimestampMs {
        return get_unix_time_ms() - self.start_time
    }

    fn clean_old_entries(&mut self) {
        let current_duration = self.current_duration();
        self.entries.retain(|entry, _| {
            (current_duration - entry) < 1000
        });
    }

    pub fn get_kbps(&mut self) -> f32 {
        self.clean_old_entries();
        if self.entries.is_empty() {
            return 0.0;
        }
        let sum = self.entries.values().sum::<Bytes>();
        let result = (sum as f32) / 1000.0 / (self.entries.len() as f32);
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
        let bytes_per_second = if total_duration > 1000 {
            (((ticket.amount_of_bytes as f32) / (total_duration as f32)) * 1000.0) as usize
        } else {
            ticket.amount_of_bytes
        };
        let current_bytes = self.entries.entry(current_duration).or_default();
        *current_bytes += bytes_per_second;
    }

    pub fn add_bytes(&mut self, bytes: usize) {
        let ticket = self.add_bytes_start(bytes);
        self.add_bytes_end(ticket);
    }
}