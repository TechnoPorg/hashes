//! SHA-1 `aarch64` backend.

// Per rustc target feature docs for `aarch64-unknown-linux-gnu` and
// `aarch64-apple-darwin` platforms, the `sha2` target feature enables
// SHA-1 as well:
//

#[cfg(target_feature = "sha2")]
pub fn compress(state: &mut [u32; 5], blocks: &[[u8; 64]]) {
    sha1_asm::compress(state, blocks);
}

#[cfg(not(target_feature = "sha2"))]
pub fn compress(state: &mut [u32; 5], blocks: &[[u8; 64]]) {
    super::soft::compress(state, blocks);
}
