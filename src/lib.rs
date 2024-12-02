use flate2::{
    write::{GzEncoder, GzDecoder, DeflateEncoder, DeflateDecoder, ZlibEncoder, ZlibDecoder},
    Compression,
};
use std::io::Write;

/// Supported compression formats.
pub enum CompressionFormat {
    Gzip,
    Deflate,
    Zlib,
}

/// Supported compression levels.
pub enum CompressionLevel {
    Fastest,
    Default,
    Best,
}

impl CompressionLevel {
    /// Maps the custom `CompressionLevel` enum to the `flate2::Compression` enum.
    ///
    /// Returns:
    /// - `flate2::Compression::fast()` for `Fastest`
    /// - `flate2::Compression::default()` for `Default`
    /// - `flate2::Compression::best()` for `Best`
    fn to_flate2_compression(&self) -> Compression {
        match self {
            CompressionLevel::Fastest => Compression::fast(),
            CompressionLevel::Default => Compression::default(),
            CompressionLevel::Best => Compression::best(),
        }
    }
}

/// Compresses a byte slice using the specified format and compression level.
///
/// # Parameters
/// - `input`: The byte slice to be compressed.
/// - `format`: The compression format to use (`Gzip`, `Deflate`, or `Zlib`).
/// - `level`: The compression level to apply (`Fastest`, `Default`, or `Best`).
///
/// # Returns
/// A `Result` containing the compressed data as a `Vec<u8>` on success, or a `String` with an error message on failure.
pub fn compress(
    input: &[u8],
    format: CompressionFormat,
    level: CompressionLevel,
) -> Result<Vec<u8>, String> {
    let compression = level.to_flate2_compression();

    match format {
        CompressionFormat::Gzip => {
            let mut encoder = GzEncoder::new(Vec::new(), compression);
            encoder
                .write_all(input)
                .map_err(|e| format!("Compression error: {}", e))?;
            encoder
                .finish()
                .map_err(|e| format!("Failed to finish compression: {}", e))
        }
        CompressionFormat::Deflate => {
            let mut encoder = DeflateEncoder::new(Vec::new(), compression);
            encoder
                .write_all(input)
                .map_err(|e| format!("Compression error: {}", e))?;
            encoder
                .finish()
                .map_err(|e| format!("Failed to finish compression: {}", e))
        }
        CompressionFormat::Zlib => {
            let mut encoder = ZlibEncoder::new(Vec::new(), compression);
            encoder
                .write_all(input)
                .map_err(|e| format!("Compression error: {}", e))?;
            encoder
                .finish()
                .map_err(|e| format!("Failed to finish compression: {}", e))
        }
    }
}

/// Decompresses a byte slice using the specified format.
///
/// # Parameters
/// - `input`: The byte slice to be decompressed.
/// - `format`: The compression format used (`Gzip`, `Deflate`, or `Zlib`).
///
/// # Returns
/// A `Result` containing the decompressed data as a `Vec<u8>` on success, or a `String` with an error message on failure.
pub fn decompress(input: &[u8], format: CompressionFormat) -> Result<Vec<u8>, String> {
    match format {
        CompressionFormat::Gzip => {
            let mut decoder = GzDecoder::new(Vec::new());
            decoder
                .write_all(input)
                .map_err(|e| format!("Decompression error: {}", e))?;
            decoder
                .finish()
                .map_err(|e| format!("Failed to finish decompression: {}", e))
        }
        CompressionFormat::Deflate => {
            let mut decoder = DeflateDecoder::new(Vec::new());
            decoder
                .write_all(input)
                .map_err(|e| format!("Decompression error: {}", e))?;
            decoder
                .finish()
                .map_err(|e| format!("Failed to finish decompression: {}", e))
        }
        CompressionFormat::Zlib => {
            let mut encoder = ZlibDecoder::new(Vec::new());
            encoder
                .write_all(input)
                .map_err(|e| format!("Compression error: {}", e))?;
            encoder
                .finish()
                .map_err(|e| format!("Failed to finish compression: {}", e))
        }
    }
}

/// Compresses data in chunks for streaming use cases.
///
/// # Parameters
/// - `chunks`: A slice of byte slices to be compressed in sequence.
/// - `format`: The compression format to use (`Gzip`, `Deflate`, or `Zlib`).
/// - `level`: The compression level to apply (`Fastest`, `Default`, or `Best`).
///
/// # Returns
/// A `Result` containing the compressed data as a `Vec<u8>` on success, or a `String` with an error message on failure.
pub fn compress_stream(
    chunks: &[&[u8]],
    format: CompressionFormat,
    level: CompressionLevel,
) -> Result<Vec<u8>, String> {
    let compression = level.to_flate2_compression();

    match format {
        CompressionFormat::Gzip => {
            let mut encoder = GzEncoder::new(Vec::new(), compression);
            for chunk in chunks {
                encoder
                    .write_all(chunk)
                    .map_err(|e| format!("Stream compression error: {}", e))?;
            }
            encoder
                .finish()
                .map_err(|e| format!("Failed to finish streaming compression: {}", e))
        }
        CompressionFormat::Deflate => {
            let mut encoder = DeflateEncoder::new(Vec::new(), compression);
            for chunk in chunks {
                encoder
                    .write_all(chunk)
                    .map_err(|e| format!("Stream compression error: {}", e))?;
            }
            encoder
                .finish()
                .map_err(|e| format!("Failed to finish streaming compression: {}", e))
        }
        CompressionFormat::Zlib => {
            let mut encoder = ZlibEncoder::new(Vec::new(), compression);
            for chunk in chunks {
                encoder
                    .write_all(chunk)
                    .map_err(|e| format!("Stream compression error: {}", e))?;
            }
            encoder
                .finish()
                .map_err(|e| format!("Failed to finish streaming compression: {}", e))
        }
    }
}

/// Decompresses data in chunks for streaming use cases.
///
/// # Parameters
/// - `chunks`: A slice of byte slices to be decompressed in sequence.
/// - `format`: The compression format used (`Gzip`, `Deflate`, or `Zlib`).
///
/// # Returns
/// A `Result` containing the decompressed data as a `Vec<u8>` on success, or a `String` with an error message on failure.
pub fn decompress_stream(
    chunks: &[&[u8]],
    format: CompressionFormat,
) -> Result<Vec<u8>, String> {
    match format {
        CompressionFormat::Gzip => {
            let mut decoder = GzDecoder::new(Vec::new());
            for chunk in chunks {
                decoder
                    .write_all(chunk)
                    .map_err(|e| format!("Stream decompression error: {}", e))?;
            }
            decoder
                .finish()
                .map_err(|e| format!("Failed to finish streaming decompression: {}", e))
        }
        CompressionFormat::Deflate => {
            let mut decoder = DeflateDecoder::new(Vec::new());
            for chunk in chunks {
                decoder
                    .write_all(chunk)
                    .map_err(|e| format!("Stream decompression error: {}", e))?;
            }
            decoder
                .finish()
                .map_err(|e| format!("Failed to finish streaming decompression: {}", e))
        }
        CompressionFormat::Zlib => {
            let mut decoder = ZlibDecoder::new(Vec::new());
            for chunk in chunks {
                decoder
                    .write_all(chunk)
                    .map_err(|e| format!("Stream decompression error: {}", e))?;
            }
            decoder
                .finish()
                .map_err(|e| format!("Failed to finish streaming decompression: {}", e))
        }
    }
}

/// Compresses a string using gzip and the specified compression level.
///
/// # Parameters
/// - `input`: The input string to be compressed.
/// - `level`: The compression level to apply (`Fastest`, `Default`, or `Best`).
///
/// # Returns
/// A `Result` containing the compressed data as a `Vec<u8>` on success, or a `String` with an error message on failure.
pub fn compress_string(input: &str, level: CompressionLevel) -> Result<Vec<u8>, String> {
    compress(input.as_bytes(), CompressionFormat::Gzip, level)
}

/// Decompresses gzip-compressed data into a string.
///
/// # Parameters
/// - `compressed_data`: The gzip-compressed byte slice to decompress.
///
/// # Returns
/// A `Result` containing the decompressed string on success, or a `String` with an error message on failure.
pub fn decompress_to_string(compressed_data: &[u8]) -> Result<String, String> {
    let decompressed_data = decompress(compressed_data, CompressionFormat::Gzip)?;
    String::from_utf8(decompressed_data).map_err(|e| format!("UTF-8 error: {}", e))
}
