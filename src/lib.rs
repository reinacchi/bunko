use flate2::{
    write::{GzEncoder, GzDecoder, DeflateEncoder, DeflateDecoder, ZlibEncoder, ZlibDecoder},
    Compression,
};
use flate2::read::{DeflateEncoder as RawDeflateEncoder, DeflateDecoder as RawDeflateDecoder};
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};
use bincode;

/// Bunko custom error handling.
#[derive(Debug)]
pub enum BunkoError {
    CompressionError(String),
    DecompressionError(String),
    Utf8Error(String),
    SerializationError(String),
    DeserializationError(String),
}

impl std::fmt::Display for BunkoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for BunkoError {}

impl From<std::string::FromUtf8Error> for BunkoError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        BunkoError::Utf8Error(err.to_string())
    }
}

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

/// Compresses raw Deflate data.
///
/// # Parameters
/// - `input`: The byte slice to be compressed.
/// - `level`: The compression level to apply (`Fastest`, `Default`, or `Best`).
///
/// # Returns
/// A `Result` containing the compressed data as a `Vec<u8>` on success, or a `BunkoError::CompressionError` on failure.

pub fn compress_raw(input: &[u8], level: CompressionLevel) -> Result<Vec<u8>, BunkoError> {
    let compression = level.to_flate2_compression();
    let mut encoder = RawDeflateEncoder::new(input, compression);
    let mut compressed = Vec::new();
    encoder
        .read_to_end(&mut compressed)
        .map_err(|e| BunkoError::CompressionError(e.to_string()))?;
    Ok(compressed)
}

/// Decompresses raw Deflate data.
///
/// # Parameters
/// - `input`: The byte slice to be decompressed.
///
/// # Returns
/// A `Result` containing the decompressed data as a `Vec<u8>` on success, or a `BunkoError::DecompressionError` on failure.

pub fn decompress_raw(input: &[u8]) -> Result<Vec<u8>, BunkoError> {
    let mut decoder = RawDeflateDecoder::new(input);
    let mut decompressed = Vec::new();
    decoder
        .read_to_end(&mut decompressed)
        .map_err(|e| BunkoError::DecompressionError(e.to_string()))?;
    Ok(decompressed)
}

/// Compresses a serializable Rust struct.
///
/// # Parameters
/// - `data`: A reference to the data structure to be compressed.
/// - `format`: The compression format to use (`Gzip`, `Deflate`, or `Zlib`).
/// - `level`: The compression level to apply (`Fastest`, `Default`, or `Best`).
///
/// # Returns
/// A `Result` containing the compressed data as a `Vec<u8>` on success, or a `BunkoError` on failure.

pub fn compress_struct<T: Serialize>(
    data: &T,
    format: CompressionFormat,
    level: CompressionLevel,
) -> Result<Vec<u8>, BunkoError> {
    let serialized = bincode::serialize(data).map_err(|e| BunkoError::SerializationError(e.to_string()))?;
    compress(&serialized, format, level).map_err(BunkoError::CompressionError)
}

/// Decompresses a byte slice into a Rust struct.
///
/// # Parameters
/// - `compressed_data`: The byte slice to be decompressed.
/// - `format`: The compression format used (`Gzip`, `Deflate`, or `Zlib`).
///
/// # Returns
/// A `Result` containing the deserialized struct on success, or a `BunkoError` on failure.

pub fn decompress_struct<T: for<'de> Deserialize<'de>>(
    compressed_data: &[u8],
    format: CompressionFormat,
) -> Result<T, BunkoError> {
    // Decompress the input
    let decompressed = decompress(compressed_data, format).map_err(BunkoError::DecompressionError)?;

    // Deserialize the decompressed data into the desired type
    bincode::deserialize(&decompressed).map_err(|e| BunkoError::DeserializationError(e.to_string()))
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

/// Compresses data with a specified buffer size.
///
/// # Parameters
/// - `input`: The byte slice to be compressed.
/// - `format`: The compression format to use (`Gzip`, `Deflate`, or `Zlib`).
/// - `level`: The compression level to apply (`Fastest`, `Default`, or `Best`).
/// - `buffer_size`: The size of the buffer for processing chunks of data.
///
/// # Returns
/// A `Result` containing the compressed data as a `Vec<u8>` on success, or a `BunkoError::CompressionError` on failure.

pub fn compress_with_buffer(
    input: &[u8],
    format: CompressionFormat,
    level: CompressionLevel,
    buffer_size: usize,
) -> Result<Vec<u8>, BunkoError> {
    let compression = level.to_flate2_compression();

    match format {
        CompressionFormat::Gzip => {
            let mut encoder = GzEncoder::new(Vec::new(), compression);
            for chunk in input.chunks(buffer_size) {
                encoder
                    .write_all(chunk)
                    .map_err(|e| BunkoError::CompressionError(e.to_string()))?;
            }
            encoder
                .finish()
                .map_err(|e| BunkoError::CompressionError(e.to_string()))
        }
        CompressionFormat::Deflate => {
            let mut encoder = DeflateEncoder::new(Vec::new(), compression);
            for chunk in input.chunks(buffer_size) {
                encoder
                    .write_all(chunk)
                    .map_err(|e| BunkoError::CompressionError(e.to_string()))?;
            }
            encoder
                .finish()
                .map_err(|e| BunkoError::CompressionError(e.to_string()))
        }
        CompressionFormat::Zlib => {
            let mut encoder = ZlibEncoder::new(Vec::new(), compression);
            for chunk in input.chunks(buffer_size) {
                encoder
                    .write_all(chunk)
                    .map_err(|e| BunkoError::CompressionError(e.to_string()))?;
            }
            encoder
                .finish()
                .map_err(|e| BunkoError::CompressionError(e.to_string()))
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

/// Calculates the compression ratio.
///
/// # Parameters
/// - `original_size`: The size of the original uncompressed data.
/// - `compressed_size`: The size of the compressed data.
///
/// # Returns
/// The compression ratio as a floating-point value (e.g., 0.25 means 25% reduction).
/// Returns `0.0` if `original_size` is zero.

pub fn calculate_compression_ratio(original_size: usize, compressed_size: usize) -> f64 {
    if original_size == 0 {
        0.0
    } else {
        1.0 - (compressed_size as f64 / original_size as f64)
    }
}
