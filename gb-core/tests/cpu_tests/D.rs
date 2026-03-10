use super::*;
test_ret!(test_0xd0_ret_nc_jump, 0xD0, FLAG_C, false, true);
test_ret!(test_0xd0_ret_nc_no_jump, 0xD0, FLAG_C, true, false);
test_pop!(
    #[ignore]
    test_0xd1_pop_de,
    0xD1,
    de,
    0x5678
);

test_jp!(
    #[ignore]
    test_0xd2_jp_nc_jump,
    0xD2,
    FLAG_C,
    false,
    0x5000,
    true
);

test_jp!(
    #[ignore]
    test_0xd2_jp_nc_no_jump,
    0xD2,
    FLAG_C,
    true,
    0x5000,
    false
);

test_call!(
    #[ignore]
    test_0xd4_call_nc_jump,
    0xD4,
    FLAG_C,
    false,
    0x9ABC,
    true
);

test_call!(
    #[ignore]
    test_0xd4_call_nc_no_jump,
    0xD4,
    FLAG_C,
    true,
    0x9ABC,
    false
);

test_push!(
    #[ignore]
    test_0xd5_push_de,
    0xD5,
    de,
    0x5678
);

test_jp!(
    #[ignore]
    test_0xda_jp_c_jump,
    0xDA,
    FLAG_C,
    true,
    0x6000,
    true
);

test_jp!(
    #[ignore]
    test_0xda_jp_c_no_jump,
    0xDA,
    FLAG_C,
    false,
    0x6000,
    false
);

test_call!(
    #[ignore]
    test_0xdc_call_c_jump,
    0xDC,
    FLAG_C,
    true,
    0xDEF0,
    true
);

test_call!(
    #[ignore]
    test_0xdc_call_c_no_jump,
    0xDC,
    FLAG_C,
    false,
    0xDEF0,
    false
);
