#[cfg(test)]
mod tests {
    #[test]
    fn quarter_round_test_vector_1() {
        let mut state = [0_u32; 16];

        quarter_round(&mut state, [0, 1, 2, 3]);

        assert_eq!(state[0..4], [0_u32; 4]);
    }
}