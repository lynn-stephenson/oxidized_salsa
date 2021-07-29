fn quarter_round(state: &mut [u32; 16], state_indexes: [usize; 4]) {
    let [a, b, c, d] = state_indexes;

    state[b] ^= state[a].wrapping_add(state[d]).rotate_left(7);
    state[c] ^= state[b].wrapping_add(state[a]).rotate_left(9);
    state[d] ^= state[c].wrapping_add(state[b]).rotate_left(13);
    state[a] ^= state[d].wrapping_add(state[c]).rotate_left(18);
}

fn row_round(mut state: &mut [u32; 16]) {
    quarter_round(&mut state, [0, 1, 2, 3]);
    quarter_round(&mut state, [5, 6, 7, 4]);
    quarter_round(&mut state, [10, 11, 8, 9]);
    quarter_round(&mut state, [15, 12, 13, 14]);
}

fn column_round(mut state: &mut [u32; 16]) {
    quarter_round(&mut state, [0, 4, 8, 12]);
    quarter_round(&mut state, [5, 9, 13, 1]);
    quarter_round(&mut state, [10, 14, 2, 6]);
    quarter_round(&mut state, [15, 3, 7, 11]);
}

fn double_round(mut state: &mut [u32; 16]) {
    column_round(&mut state);
    row_round(&mut state);
}

fn hash(key_stream: &mut [u8; 64]) {
    let mut state = [0_u32; 16];

    for (index, state_bytes) in key_stream.chunks_exact(4).enumerate() {
        state[index] = u32::from_le_bytes(
            std::convert::TryInto::try_into(state_bytes).unwrap()
        );
    }

    let original_state = state;

    for _rounds in 0..10 {
        double_round(&mut state);
    }

    for (index, state_bytes) in key_stream.chunks_exact_mut(4).enumerate() {
        state_bytes.copy_from_slice(
            &(state[index].wrapping_add(original_state[index])).to_le_bytes()
        );
    }
}

fn expand16(
    key: [u8; 16],
    number_used_once: [u8; 8],
    position: [u8; 8]
) -> [u8; 64] {
    let constant = [
        b"expa",
        b"nd 1",
        b"6-by",
        b"te k"
    ];

    let mut key_stream = [0_u8; 64];

    key_stream[0] = constant[0][0];
    key_stream[1] = constant[0][1];
    key_stream[2] = constant[0][2];
    key_stream[3] = constant[0][3];

    key_stream[4] = key[0];
    key_stream[5] = key[1];
    key_stream[6] = key[2];
    key_stream[7] = key[3];

    key_stream[8] = key[4];
    key_stream[9] = key[5];
    key_stream[10] = key[6];
    key_stream[11] = key[7];

    key_stream[12] = key[8];
    key_stream[13] = key[9];
    key_stream[14] = key[10];
    key_stream[15] = key[11];

    key_stream[16] = key[12];
    key_stream[17] = key[13];
    key_stream[18] = key[14];
    key_stream[19] = key[15];

    key_stream[20] = constant[1][0];
    key_stream[21] = constant[1][1];
    key_stream[22] = constant[1][2];
    key_stream[23] = constant[1][3];

    key_stream[24] = number_used_once[0];
    key_stream[25] = number_used_once[1];
    key_stream[26] = number_used_once[2];
    key_stream[27] = number_used_once[3];

    key_stream[28] = number_used_once[4];
    key_stream[29] = number_used_once[5];
    key_stream[30] = number_used_once[6];
    key_stream[31] = number_used_once[7];

    key_stream[32] = position[0];
    key_stream[33] = position[1];
    key_stream[34] = position[2];
    key_stream[35] = position[3];

    key_stream[36] = position[4];
    key_stream[37] = position[5];
    key_stream[38] = position[6];
    key_stream[39] = position[7];

    key_stream[40] = constant[2][0];
    key_stream[41] = constant[2][1];
    key_stream[42] = constant[2][2];
    key_stream[43] = constant[2][3];

    key_stream[44] = key[0];
    key_stream[45] = key[1];
    key_stream[46] = key[2];
    key_stream[47] = key[3];

    key_stream[48] = key[4];
    key_stream[49] = key[5];
    key_stream[50] = key[6];
    key_stream[51] = key[7];

    key_stream[52] = key[8];
    key_stream[53] = key[9];
    key_stream[54] = key[10];
    key_stream[55] = key[11];

    key_stream[56] = key[12];
    key_stream[57] = key[13];
    key_stream[58] = key[14];
    key_stream[59] = key[15];

    key_stream[60] = constant[3][0];
    key_stream[61] = constant[3][1];
    key_stream[62] = constant[3][2];
    key_stream[63] = constant[3][3];

    key_stream
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quarter_round_test_vector_1() {
        let mut state = [0_u32; 16];

        quarter_round(&mut state, [0, 1, 2, 3]);

        assert_eq!(state[0..4], [0_u32; 4]);
    }

    #[test]
    fn quarter_round_test_vector_2() {
        let mut state = [0_u32; 16];
        state[0] = 1;

        quarter_round(&mut state, [0, 1, 2, 3]);

        assert_eq!(
            state[0..4],
            [0x08008145, 0x00000080, 0x00010200, 0x20500000]
        );
    }

    #[test]
    fn quarter_round_test_vector_3() {
        let mut state = [0_u32; 16];
        state[1] = 1;

        quarter_round(&mut state, [0, 1, 2, 3]);

        assert_eq!(
            state[0..4],
            [0x88000100, 0x00000001, 0x00000200, 0x00402000]
        );
    }

    #[test]
    fn quarter_round_test_vector_4() {
        let mut state = [0_u32; 16];
        state[2] = 1;

        quarter_round(&mut state, [0, 1, 2, 3]);

        assert_eq!(
            state[0..4],
            [0x80040000, 0x00000000, 0x00000001, 0x00002000]
        );
    }

    #[test]
    fn quarter_round_test_vector_5() {
        let mut state = [0_u32; 16];
        state[3] = 1;

        quarter_round(&mut state, [0, 1, 2, 3]);

        assert_eq!(
            state[0..4],
            [0x00048044, 0x00000080, 0x00010000, 0x20100001]
        );
    }

    #[test]
    fn quarter_round_test_vector_6() {
        let mut state = [0_u32; 16];
        state[0] = 0xe7e8c006;
        state[1] = 0xc4f9417d;
        state[2] = 0x6479b4b2;
        state[3] = 0x68c67137;

        quarter_round(&mut state, [0, 1, 2, 3]);

        assert_eq!(
            state[0..4],
            [0xe876d72b, 0x9361dfd5, 0xf1460244, 0x948541a3]
        );
    }

    #[test]
    fn quarter_round_test_vector_7() {
        let mut state = [0_u32; 16];
        state[0] = 0xd3917c5b;
        state[1] = 0x55f1c407;
        state[2] = 0x52a58a7a;
        state[3] = 0x8f887a3b;

        quarter_round(&mut state, [0, 1, 2, 3]);

        assert_eq!(
            state[0..4],
            [0x3e2f308c, 0xd90a8f36, 0x6ab2a923, 0x2883524c]
        );
    }

    #[test]
    fn row_round_test_vector_1() {
        let mut state = [
            0x00000001, 0x00000000, 0x00000000, 0x00000000,
            0x00000001, 0x00000000, 0x00000000, 0x00000000,
            0x00000001, 0x00000000, 0x00000000, 0x00000000,
            0x00000001, 0x00000000, 0x00000000, 0x00000000
        ];

        row_round(&mut state);

        assert_eq!(state,
            [
                0x08008145, 0x00000080, 0x00010200, 0x20500000,
                0x20100001, 0x00048044, 0x00000080, 0x00010000,
                0x00000001, 0x00002000, 0x80040000, 0x00000000,
                0x00000001, 0x00000200, 0x00402000, 0x88000100
            ]
        );
    }

    #[test]
    fn row_round_test_vector_2() {
        let mut state = [
            0x08521bd6, 0x1fe88837, 0xbb2aa576, 0x3aa26365,
            0xc54c6a5b, 0x2fc74c2f, 0x6dd39cc3, 0xda0a64f6,
            0x90a2f23d, 0x067f95a6, 0x06b35f61, 0x41e4732e,
            0xe859c100, 0xea4d84b7, 0x0f619bff, 0xbc6e965a
        ];

        row_round(&mut state);

        assert_eq!(
            state,
            [
                0xa890d39d, 0x65d71596, 0xe9487daa, 0xc8ca6a86,
                0x949d2192, 0x764b7754, 0xe408d9b9, 0x7a41b4d1,
                0x3402e183, 0x3c3af432, 0x50669f96, 0xd89ef0a8,
                0x0040ede5, 0xb545fbce, 0xd257ed4f, 0x1818882d
            ]
        );
    }

    #[test]
    fn column_round_test_vector_1() {
        let mut state = [
            0x00000001, 0x00000000, 0x00000000, 0x00000000,
            0x00000001, 0x00000000, 0x00000000, 0x00000000,
            0x00000001, 0x00000000, 0x00000000, 0x00000000,
            0x00000001, 0x00000000, 0x00000000, 0x00000000
        ];

        column_round(&mut state);

        assert_eq!(
            state,
            [
                0x10090288, 0x00000000, 0x00000000, 0x00000000,
                0x00000101, 0x00000000, 0x00000000, 0x00000000,
                0x00020401, 0x00000000, 0x00000000, 0x00000000,
                0x40a04001, 0x00000000, 0x00000000, 0x00000000
            ]
        );
    }

    #[test]
    fn column_round_test_vector_2() {
        let mut state = [
            0x08521bd6, 0x1fe88837, 0xbb2aa576, 0x3aa26365,
            0xc54c6a5b, 0x2fc74c2f, 0x6dd39cc3, 0xda0a64f6,
            0x90a2f23d, 0x067f95a6, 0x06b35f61, 0x41e4732e,
            0xe859c100, 0xea4d84b7, 0x0f619bff, 0xbc6e965a
        ];

        column_round(&mut state);

        assert_eq!(
            state,
            [
                0x8c9d190a, 0xce8e4c90, 0x1ef8e9d3, 0x1326a71a,
                0x90a20123, 0xead3c4f3, 0x63a091a0, 0xf0708d69,
                0x789b010c, 0xd195a681, 0xeb7d5504, 0xa774135c,
                0x481c2027, 0x53a8e4b5, 0x4c1f89c5, 0x3f78c9c8
            ]
        );
    }

    #[test]
    fn double_round_test_vector_1() {
        let mut state = [
            0x00000001, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000
        ];

        double_round(&mut state);

        assert_eq!(
            state,
            [
                0x8186a22d, 0x0040a284, 0x82479210, 0x06929051,
                0x08000090, 0x02402200, 0x00004000, 0x00800000,
                0x00010200, 0x20400000, 0x08008104, 0x00000000,
                0x20500000, 0xa0000040, 0x0008180a, 0x612a8020
            ]
        );
    }

    #[test]
    fn double_round_test_vector_2() {
        let mut state = [
            0xde501066, 0x6f9eb8f7, 0xe4fbbd9b, 0x454e3f57,
            0xb75540d3, 0x43e93a4c, 0x3a6f2aa0, 0x726d6b36,
            0x9243f484, 0x9145d1e8, 0x4fa9d247, 0xdc8dee11,
            0x054bf545, 0x254dd653, 0xd9421b6d, 0x67b276c1
        ];

        double_round(&mut state);

        assert_eq!(
            state,
            [
                0xccaaf672, 0x23d960f7, 0x9153e63a, 0xcd9a60d0,
                0x50440492, 0xf07cad19, 0xae344aa0, 0xdf4cfdfc,
                0xca531c29, 0x8e7943db, 0xac1680cd, 0xd503ca00,
                0xa74b2ad6, 0xbc331c5c, 0x1dda24c7, 0xee928277
            ]
        );
    }

    #[test]
    fn hash_test_vector_1() {
        let mut key_stream = [0_u8; 64];

        hash(&mut key_stream);

        assert_eq!(key_stream, [0_u8; 64]);
    }

    #[test]
    fn hash_test_vector_2() {
        let mut key_stream = [
            211,159, 13,115, 76, 55, 82,183, 3,117,222, 37,191,187,234,136,
            49,237,179, 48, 1,106,178,219,175,199,166, 48, 86, 16,179,207,
            31,240, 32, 63, 15, 83, 93,161,116,147, 48,113,238, 55,204, 36,
            79,201,235, 79, 3, 81,156, 47,203, 26,244,243, 88,118,104, 54
        ];

        hash(&mut key_stream);

        assert_eq!(
            key_stream,
            [
                109, 42,178,168,156,240,248,238,168,196,190,203, 26,110,170,154,
                29, 29,150, 26,150, 30,235,249,190,163,251, 48, 69,144, 51, 57,
                118, 40,152,157,180, 57, 27, 94,107, 42,236, 35, 27,111,114,114,
                219,236,232,135,111,155,110, 18, 24,232, 95,158,179, 19, 48,202
            ]
        );
    }

    #[test]
    fn hash_test_vector_3() {
        let mut key_stream = [
            88,118,104, 54, 79,201,235, 79, 3, 81,156, 47,203, 26,244,243,
            191,187,234,136,211,159, 13,115, 76, 55, 82,183, 3,117,222, 37,
            86, 16,179,207, 49,237,179, 48, 1,106,178,219,175,199,166, 48,
            238, 55,204, 36, 31,240, 32, 63, 15, 83, 93,161,116,147, 48,113
        ];

        hash(&mut key_stream);

        assert_eq!(
            key_stream,
            [
                179, 19, 48,202,219,236,232,135,111,155,110, 18, 24,232, 95,158,
                26,110,170,154,109, 42,178,168,156,240,248,238,168,196,190,203,
                69,144, 51, 57, 29, 29,150, 26,150, 30,235,249,190,163,251, 48,
                27,111,114,114,118, 40,152,157,180, 57, 27, 94,107, 42,236, 35
            ]
        );
    }

    #[test]
    fn hash_test_vector_4() {
        let mut key_stream = [
            6,124, 83,146, 38,191, 9, 50, 4,161, 47,222,122,182,223,185,
            75, 27, 0,216, 16,122, 7, 89,162,104,101,147,213, 21, 54, 95,
            225,253,139,176,105,132, 23,116, 76, 41,176,207,221, 34,157,108,
            94, 94, 99, 52, 90,117, 91,220,146,190,239,143,196,176,130,186
        ];

        for _j in 0..1_000_000 {
            hash(&mut key_stream);
        }

        assert_eq!(
            key_stream,
            [
                8, 18, 38,199,119, 76,215, 67,173,127,144,162,103,212,176,217,
                192, 19,233, 33,159,197,154,160,128,243,219, 65,171,136,135,225,
                123, 11, 68, 86,237, 82, 20,155,133,189, 9, 83,167,116,194, 78,
                122,127,195,185,185,204,188, 90,245, 9,183,248,226, 85,245,104
            ]
        );
    }

    #[test]
    fn expand16_test_vector_1() {
        assert_eq!(
            expand16(
                [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
                [101, 102, 103, 104, 105, 106, 107, 108],
                [109, 110, 111, 112, 113, 114, 115, 116]
            ),
            [
                101,120,112, 97, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
                13, 14, 15, 16,110,100, 32, 49,101,102,103,104,105,106,107,108,
                109,110,111,112,113,114,115,116, 54, 45, 98,121, 1, 2, 3, 4,
                5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,116,101, 32,107
            ]
        );
    }

    #[test]
    fn expand16_test_vector_2() {
        let mut key_stream = expand16(
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            [101, 102, 103, 104, 105, 106, 107, 108],
            [109, 110, 111, 112, 113, 114, 115, 116]
        );

        hash(&mut key_stream);

        assert_eq!(
            key_stream,
            [
                39,173, 46,248, 30,200, 82, 17, 48, 67,254,239, 37, 18, 13,247,
                241,200, 61,144, 10, 55, 50,185, 6, 47,246,253,143, 86,187,225,
                134, 85,110,246,161,163, 43,235,231, 94,171, 51,145,214,112, 29,
                14,232, 5, 16,151,140,183,141,171, 9,122,181,104,182,177,193
            ]
        );
    }

    #[test]
    fn expand32_test_vector_1() {
        assert_eq!(
            expand32(
                [
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                    201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211,
                    212, 213, 214, 215, 216
                ],
                [101, 102, 103, 104, 105, 106, 107, 108],
                [109, 110, 111, 112, 113, 114, 115, 116]
            ),
            [
                101,120,112, 97, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
                13, 14, 15, 16,110,100, 32, 51,101,102,103,104,105,106,107,108,
                109,110,111,112,113,114,115,116, 50, 45, 98,121,201,202,203,204,
                205,206,207,208,209,210,211,212,213,214,215,216,116,101, 32,107
            ]
        );
    }

    #[test]
    fn expand32_test_vector_2() {
        let mut key_stream = expand32(
            [
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211,
                212, 213, 214, 215, 216
            ],
            [101, 102, 103, 104, 105, 106, 107, 108],
            [109, 110, 111, 112, 113, 114, 115, 116]
        );

        hash(&mut key_stream);

        assert_eq!(
            key_stream,
            [
                69, 37, 68, 39, 41, 15,107,193,255,139,122, 6,170,233,217, 98,
                89,144,182,106, 21, 51,200, 65,239, 49,222, 34,215,114, 40,126,
                104,197, 7,225,197,153, 31, 2,102, 78, 76,176, 84,245,246,184,
                177,160,133,130, 6, 72,149,119,192,195,132,236,234,103,246, 74
            ]
        );
    }
}