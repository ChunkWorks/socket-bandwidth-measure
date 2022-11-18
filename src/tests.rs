#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    use crate::bandwidth_measure::BandwidthMeasure;

    #[test]
    fn bandwidth_test() {
        let mut measure = BandwidthMeasure::new();
        let ticket = measure.add_bytes_start(1000);
        thread::sleep(Duration::from_millis(1100));
        measure.add_bytes_end(ticket);
        assert_eq!(measure.get_kbps(), 0.909);
        thread::sleep(Duration::from_millis(2000));
        assert_eq!(measure.get_kbps(), 0.0);
    }
}