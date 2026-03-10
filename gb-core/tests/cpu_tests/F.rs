use super::*;

test_pop!(
    #[ignore]
    test_0xf1_pop_af,
    0xF1,
    af,
    0x42F0
);

test_push!(
    #[ignore]
    test_0xf5_push_af,
    0xF5,
    af,
    0x42F0
);
