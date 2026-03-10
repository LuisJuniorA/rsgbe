use super::*;

test_pop!(
    #[ignore]
    test_0xe1_pop_hl,
    0xE1,
    hl,
    0x9ABC
);

test_push!(
    #[ignore]
    test_0xe5_push_hl,
    0xE5,
    hl,
    0x9ABC
);
