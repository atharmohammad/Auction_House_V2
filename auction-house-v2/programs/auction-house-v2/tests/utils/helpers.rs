use auction_house_v2::ID;
use mpl_bubblegum;
use solana_program_test::*;

pub fn auction_house_program_test() -> ProgramTest {
    let mut program = ProgramTest::new("auction_house_v2", ID, None);
    program.add_program("mpl_bubblegum", mpl_bubblegum::ID, None);
    program.set_compute_max_units(u64::MAX);
    return program;
}
