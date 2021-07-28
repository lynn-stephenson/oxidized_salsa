fn quarter_round(state: &mut [u32; 16], state_indexes: [usize; 4]) {
    let [a, b, c, d] = state_indexes;

    state[b] ^= state[a].wrapping_add(state[d]).rotate_left(7);
    state[c] ^= state[b].wrapping_add(state[a]).rotate_left(9);
    state[d] ^= state[c].wrapping_add(state[b]).rotate_left(13);
    state[a] ^= state[d].wrapping_add(state[c]).rotate_left(18);
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
}