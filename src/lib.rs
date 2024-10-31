use core::arch::x86_64::__cpuid;

/// `rusty_cpuid` provides utilities for detecting CPU features on x86_64.

/// Checks if SSE (Streaming SIMD Extensions) is supported on the CPU.
pub fn is_sse_supported() -> bool {
    unsafe {
        let cpuid_result = __cpuid(1);
        // Check if the SSE bit (bit 25 of EDX) is set
        (cpuid_result.edx & (1 << 25)) != 0
    }
}

/// Checks if AVX (Advanced Vector Extensions) is supported on the CPU.
pub fn is_avx_supported() -> bool {
    unsafe {
        let cpuid_result = __cpuid(1);
        // Check if the AVX bit (bit 28 of ECX) is set
        (cpuid_result.ecx & (1 << 28)) != 0
    }
}

/// Checks if AVX2 (Advanced Vector Extensions 2) is supported on the CPU.
pub fn is_avx2_supported() -> bool {
    unsafe {
        let cpuid_result = __cpuid(7);
        // Check if the AVX2 bit (bit 5 of EBX) is set
        (cpuid_result.ebx & (1 << 5)) != 0
    }
}

/// Checks if FMA (Fused Multiply-Add) is supported on the CPU.
pub fn is_fma_supported() -> bool {
    unsafe {
        let cpuid_result = __cpuid(1);
        // Check if the FMA bit (bit 12 of ECX) is set
        (cpuid_result.ecx & (1 << 12)) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sse_support() {
        println!("SSE Supported: {}", is_sse_supported());
    }

    #[test]
    fn test_avx_support() {
        println!("AVX Supported: {}", is_avx_supported());
    }

    #[test]
    fn test_avx2_support() {
        println!("AVX2 Supported: {}", is_avx2_supported());
    }

    #[test]
    fn test_fma_support() {
        println!("FMA Supported: {}", is_fma_supported());
    }
}
