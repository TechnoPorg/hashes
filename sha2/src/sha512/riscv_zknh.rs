use core::arch::asm;

#[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
compile_error!("riscv-zknh backend can be used only on riscv32 and riscv64 target arches");

cfg_if::cfg_if! {
    if #[cfg(sha2_backend_riscv_zknh = "compact")] {
        mod compact;
        use compact::compress_block;
    } else {
        mod unroll;
        use unroll::compress_block;
    }
}

#[target_feature(enable = "zknh")]
pub(super) fn compress(state: &mut [u64; 8], blocks: &[[u8; 128]]) {
    for block in blocks {
        let block: [u64; 16] = core::array::from_fn(|i| {
            let chunk = block[8 * i..][..8].try_into().unwrap();
            u64::from_be_bytes(chunk)
        });
        compress_block(state, block);
    }
}

cfg_if::cfg_if! {
    if #[cfg(target_arch = "riscv64")] {
        fn sha512sum0(x: u64) -> u64 {
            let a: u64;
            unsafe { asm!("sha512sum0 {rd}, {rs1}", rd = out(reg) a, rs1 = in(reg) x); }
            a
        }

        fn sha512sum1(x: u64) -> u64 {
            let a: u64;
            unsafe { asm!("sha512sum1 {rd}, {rs1}", rd = out(reg) a, rs1 = in(reg) x); }
            a
        }

        fn sha512sig0(x: u64) -> u64 {
            let a: u64;
            unsafe { asm!("sha512sig0 {rd}, {rs1}", rd = out(reg) a, rs1 = in(reg) x); }
            a
        }

        fn sha512sig1(x: u64) -> u64 {
            let a: u64;
            unsafe { asm!("sha512sig1 {rd}, {rs1}", rd = out(reg) a, rs1 = in(reg) x); }
            a
        }

    } else {
        #[target_feature(enable = "zknh")]
        fn sha512sum0(x: u64) -> u64 {
            let a: u32;
            unsafe { asm!("sha512sum0r {rd}, {rs1}, {rs2}", rd = out(reg) a, rs1 = in(reg) (x >> 32) as u32, rs2 = in(reg) x as u32); }
            let b: u32;
            unsafe { asm!("sha512sum0r {rd}, {rs1}, {rs2}", rd = out(reg) b, rs1 = in(reg) x as u32, rs2 = in(reg) (x >> 32) as u32); }
            ((a as u64) << 32) | (b as u64)
        }

        #[target_feature(enable = "zknh")]
        fn sha512sum1(x: u64) -> u64 {
            let a: u32;
            unsafe { asm!("sha512sum1r {rd}, {rs1}, {rs2}", rd = out(reg) a, rs1 = in(reg) (x >> 32) as u32, rs2 = in(reg) x as u32); }
            let b: u32;
            unsafe { asm!("sha512sum1r {rd}, {rs1}, {rs2}", rd = out(reg) b, rs1 = in(reg) x as u32, rs2 = in(reg) (x >> 32) as u32); }
            ((a as u64) << 32) | (b as u64)
        }

        #[target_feature(enable = "zknh")]
        fn sha512sig0(x: u64) -> u64 {
            let a: u32;
            unsafe { asm!("sha512sig0h {rd}, {rs1}, {rs2}", rd = out(reg) a, rs1 = in(reg) (x >> 32) as u32, rs2 = in(reg) x as u32); }
            let b: u32;
            unsafe { asm!("sha512sig0l {rd}, {rs1}, {rs2}", rd = out(reg) b, rs1 = in(reg) x as u32, rs2 = in(reg) (x >> 32) as u32); }
            ((a as u64) << 32) | (b as u64)
        }

        #[target_feature(enable = "zknh")]
        fn sha512sig1(x: u64) -> u64 {
            let a: u32;
            unsafe { asm!("sha512sig1h {rd}, {rs1}, {rs2}", rd = out(reg) a, rs1 = in(reg) (x >> 32) as u32, rs2 = in(reg) x as u32); }
            let b: u32;
            unsafe { asm!("sha512sig1l {rd}, {rs1}, {rs2}", rd = out(reg) b, rs1 = in(reg) x as u32, rs2 = in(reg) (x >> 32) as u32); }
            ((a as u64) << 32) | (b as u64)
        }
    }
}
