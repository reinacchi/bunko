#[cfg(test)]
mod tests {
    use bunko::{compress, decompress_to_string, CompressionFormat, CompressionLevel};

    #[test]
    fn main() {
        let input = "This is a test string for compression!";
        let compressed = compress(&input.as_bytes(), CompressionFormat::Gzip, CompressionLevel::Fastest).expect("Compression failed");
        assert!(!compressed.is_empty(), "Compressed data should not be empty");

        let decompressed = decompress_to_string(&compressed).expect("Decompression failed");
        assert_eq!(input, decompressed, "Decompressed data should match the input");
    }
}
