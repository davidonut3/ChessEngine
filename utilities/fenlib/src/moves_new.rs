use crate::utils_new::*;

/// This struct stores moves without compression.
/// This is fast, but not memory efficient.
#[derive(Debug, Clone)]
pub struct LargeMove {
    pub array: [u128; 3],
}

/// This struct stores moves with compression.
/// First 6 bits for start pos, next 6 bits for end pos, last 4 bits for prom info.
/// This is memory efficient, but may be slower.
#[derive(Debug, Clone)]
pub struct SmallMove {
    pub move1: u16,
}