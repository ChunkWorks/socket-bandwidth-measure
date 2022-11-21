#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;
    use log::debug;
    use test_log::test;

    use crate::bandwidth_measure::BandwidthMeasure;

    static EPSILON: f32 = 0.5;
    macro_rules! assert_eq_epsilon {
        ($a:expr, $b:expr) => {
            assert!(($a - $b) < EPSILON, "($a({}) - $b({})) < EPSILON({})", $a, $b, EPSILON)
        };
    }

    #[test]
    fn large_bytes_test() {
        let mut measure = BandwidthMeasure::new();
        let ticket = measure.add_bytes_start(1024 * 1024 * 2);
        debug!("ticket: {:#?}", ticket);
        thread::sleep(Duration::from_millis(100));
        measure.add_bytes_end(ticket);
        let ticket = measure.add_bytes_start(1024 * 1024 * 2);
        thread::sleep(Duration::from_millis(100));
        measure.add_bytes_end(ticket);
        debug!("measure: {:#?}", measure);
        assert_eq_epsilon!(measure.get_kbps(), 2097.0);
    }

    #[test]
    fn bandwidth_test() {
        let mut measure = BandwidthMeasure::new();
        let ticket = measure.add_bytes_start(1000);
        thread::sleep(Duration::from_millis(1100));
        measure.add_bytes_end(ticket);
        assert_eq_epsilon!(measure.get_kbps(), 0.909);
        thread::sleep(Duration::from_millis(2000));
        assert_eq_epsilon!(measure.get_kbps(), 0.0);

        let mut measure = BandwidthMeasure::new();
        let ticket = measure.add_bytes_start(1000);
        thread::sleep(Duration::from_millis(10));
        measure.add_bytes_end(ticket);
        let ticket = measure.add_bytes_start(5000);
        thread::sleep(Duration::from_millis(100));
        measure.add_bytes_end(ticket);
        assert_eq_epsilon!(measure.get_kbps(), 75.0);
        measure.add_bytes(1000);
        assert_eq_epsilon!(measure.get_kbps(), 75.25);
    }
}