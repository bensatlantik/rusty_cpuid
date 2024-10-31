## rusty_cpuid
rusty_cpuid is a Rust library that provides easy-to-use utilities for detecting CPU features on x86_64 architectures. It enables developers to optimize performance-critical applications by checking for support of key instruction sets like SSE, AVX, AVX2, and FMA.

## Features
SSE Detection: Check if the CPU supports Streaming SIMD Extensions (SSE).
AVX Detection: Verify support for Advanced Vector Extensions (AVX).
AVX2 Detection: Determine if the CPU has Advanced Vector Extensions 2 (AVX2) capabilities.
FMA Detection: Identify support for Fused Multiply-Add (FMA) instructions.

##Installation
Add rusty_cpuid to your Cargo.toml:

toml
[dependencies]
rusty_cpuid = "0.1.0"

Then, import it in your Rust code:
```
rust
Copy code
use rusty_cpuid::*;
```
## Usage
Example: Checking for CPU Feature Support
```use rusty_cpuid::*;

fn main() {
    if is_sse_supported() {
        println!("SSE is supported!");
    } else {
        println!("SSE is not supported.");
    }

    if is_avx_supported() {
        println!("AVX is supported!");
    } else {
        println!("AVX is not supported.");
    }

    if is_avx2_supported() {
        println!("AVX2 is supported!");
    } else {
        println!("AVX2 is not supported.");
    }

    if is_fma_supported() {
        println!("FMA is supported!");
    } else {
        println!("FMA is not supported.");
    }
}
```
## Why Use rusty_cpuid?
Performance Optimization: Tailor your application's performance based on available CPU features.
Easy to Use: Simple and straightforward API for checking popular CPU features.
Safe and Reliable: Leverages Rust's powerful abstractions while providing direct access to low-level CPU information.

## License
This project is licensed under the MIT License. See the LICENSE file for details.

## Acknowledgements
Special thanks to the Rust community for their continuous support and contributions to low-level programming tools.

## Author
Ben Santora (<bensatlantik@gmail.com>)