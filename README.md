# bunko 📦🔥

**bunko** is a lightweight, flexible, and high-performance Rust library for data compression and decompression. bunko simplifies handling Gzip, Zlib and Deflate compression in both single-pass and streaming modes while leveraging Rust's safety and performance.

### Why choose bunko?

- Built with Rust, bunko offers incredible performance with low overhead.
- No manual memory management, ensuring safety at every step.
- Designed to be intuitive, whether you're compressing small strings or handling large data streams.


## Getting Started

### Installation

Add bunko to your `Cargo.toml`:

```toml
[dependencies]
bunko = "0.1.0"
```

### Example: Gzip Compression and Decompression

```rs
use bunko::{compress_string, decompress_to_string, CompressionLevel};

fn main() {
    let input = "Hello, Bunko!";
    println!("Original: {}", input);

    // Compress the string
    let compressed = compress_string(input, CompressionLevel::Best)
        .expect("Failed to compress string");
    println!("Compressed size: {} bytes", compressed.len());

    // Decompress the string
    let decompressed = decompress_to_string(&compressed)
        .expect("Failed to decompress string");
    println!("Decompressed: {}", decompressed);

    assert_eq!(input, decompressed);
}
```

## License

This library is licensed under [MIT](https://github.com/reinacchi/bunko/blob/master/LICENSE).