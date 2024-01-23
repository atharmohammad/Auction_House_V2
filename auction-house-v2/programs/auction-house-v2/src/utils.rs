use anchor_lang::solana_program::program_memory::sol_memcmp;

pub fn cmp_bytes(a: &[u8], b: &[u8], size: usize) -> bool {
    sol_memcmp(a, b, size) == 0
}
