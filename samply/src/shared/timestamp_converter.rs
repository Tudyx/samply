use fxprof_processed_profile::Timestamp;

#[derive(Debug, Clone, Copy)]
pub struct TimestampConverter {
    /// A reference timestamp, as a raw timestamp.
    pub reference_raw: u64,
    /// A "ticks per nanosecond" conversion factor. If raw values are in nanoseconds, this is 1.
    pub raw_to_ns_factor: u64,
}

impl TimestampConverter {
    pub fn convert_time(&self, ktime_ns: u64) -> Timestamp {
        Timestamp::from_nanos_since_reference(
            ktime_ns.saturating_sub(self.reference_raw) * self.raw_to_ns_factor,
        )
    }
}
