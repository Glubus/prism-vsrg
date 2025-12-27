//! Replay compression and file storage utilities.
//!
//! Provides zstd compression with rkyv serialization for efficient replay storage.

use crate::types::ReplayData;
use rkyv::rancor::Error;
use zstd::stream::{decode_all, encode_all};

/// Compression level for zstd (21 = maximum, best compression).
pub const COMPRESSION_LEVEL: i32 = 21;

/// Compress replay data to bytes using rkyv + zstd.
///
/// Returns compressed bytes ready for storage or transmission.
pub fn compress(data: &ReplayData) -> std::io::Result<Vec<u8>> {
    let binary_data = rkyv::to_bytes::<Error>(data).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Serialization error: {}", e),
        )
    })?;

    encode_all(&binary_data[..], COMPRESSION_LEVEL)
}

/// Decompress replay data from bytes.
///
/// Takes compressed bytes and returns the original ReplayData.
pub fn decompress(compressed: &[u8]) -> std::io::Result<ReplayData> {
    let binary_data = decode_all(compressed)?;

    rkyv::from_bytes::<ReplayData, Error>(&binary_data).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Deserialization error: {}", e),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_decompress_roundtrip() {
        let test_data = ReplayData::new(1.0);

        let compressed = compress(&test_data).unwrap();
        let decompressed = decompress(&compressed).unwrap();

        assert_eq!(decompressed, test_data);
    }

    #[test]
    fn test_compress_with_inputs() {
        let mut data = ReplayData::new(1.5);
        data.add_press(1000, 0);
        data.add_release(1500, 0);
        data.add_press(2000, 1);

        let compressed = compress(&data).unwrap();
        let decompressed = decompress(&compressed).unwrap();

        assert_eq!(decompressed, data);
        assert_eq!(decompressed.input_count(), 3);
    }
}
