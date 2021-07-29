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
}