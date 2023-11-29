#[cfg(test)]
mod types_tests {
    use crate::binary_format::BinaryFormat;

    #[test]
    fn default() {
        let b = BinaryFormat::default();
        assert_eq!(b.format(vec![]).unwrap(), "0x");
        assert_eq!(b.format(vec![2]).unwrap(), "0x02");
        assert_eq!(b.format(vec![2, 4, 6, 8]).unwrap(), "0x02040608");
    }

    #[test]
    fn no_prefix() {
        let b = BinaryFormat::new().no_prefix();
        assert_eq!(b.format(vec![]).unwrap(), "");
        assert_eq!(b.format(vec![2]).unwrap(), "02");
        assert_eq!(b.format(vec![2, 4, 6, 8]).unwrap(), "02040608");
    }

    // alignment tests

    #[test]
    fn prefix_left_align() {
        let b = BinaryFormat::new().min_width(6).left_align();
        assert_eq!(b.format(vec![]).unwrap(), "0x    ");
        assert_eq!(b.format(vec![2]).unwrap(), "0x02  ");
        assert_eq!(b.format(vec![2, 4, 6, 8]).unwrap(), "0x02040608");
    }

    #[test]
    fn prefix_right_align() {
        let b = BinaryFormat::new().min_width(6).right_align();
        assert_eq!(b.format(vec![]).unwrap(), "    0x");
        assert_eq!(b.format(vec![2]).unwrap(), "  0x02");
        assert_eq!(b.format(vec![2, 4, 6, 8]).unwrap(), "0x02040608");
    }

    #[test]
    fn prefix_left_align_zero_padding() {
        let b = BinaryFormat::new().min_width(6).left_align().fill_char('0');
        assert_eq!(b.format(vec![]).unwrap(), "0x0000");
        assert_eq!(b.format(vec![2]).unwrap(), "0x0200");
        assert_eq!(b.format(vec![2, 4, 6, 8]).unwrap(), "0x02040608");
    }

    #[test]
    fn prefix_right_align_zero_padding() {
        let b = BinaryFormat::new()
            .min_width(6)
            .right_align()
            .fill_char('0');
        assert_eq!(b.format(vec![]).unwrap(), "0x0000");
        assert_eq!(b.format(vec![2]).unwrap(), "0x0002");
        assert_eq!(b.format(vec![2, 4, 6, 8]).unwrap(), "0x02040608");
    }

    // no prefix tests

    #[test]
    fn no_prefix_left_align() {
        let b = BinaryFormat::new().min_width(6).left_align().no_prefix();
        assert_eq!(b.format(vec![]).unwrap(), "      ");
        assert_eq!(b.format(vec![2]).unwrap(), "02    ");
        assert_eq!(b.format(vec![2, 4, 6, 8]).unwrap(), "02040608");
    }

    #[test]
    fn no_prefix_right_align() {
        let b = BinaryFormat::new().min_width(6).right_align().no_prefix();
        assert_eq!(b.format(vec![]).unwrap(), "      ");
        assert_eq!(b.format(vec![2]).unwrap(), "    02");
        assert_eq!(b.format(vec![2, 4, 6, 8]).unwrap(), "02040608");
    }

    #[test]
    fn no_prefix_left_align_zero_padding() {
        let b = BinaryFormat::new()
            .min_width(6)
            .left_align()
            .fill_char('0')
            .no_prefix();
        assert_eq!(b.format(vec![]).unwrap(), "000000");
        assert_eq!(b.format(vec![2]).unwrap(), "020000");
        assert_eq!(b.format(vec![2, 4, 6, 8]).unwrap(), "02040608");
    }

    #[test]
    fn no_prefix_right_align_zero_padding() {
        let b = BinaryFormat::new()
            .min_width(6)
            .right_align()
            .fill_char('0')
            .no_prefix();
        assert_eq!(b.format(vec![]).unwrap(), "000000");
        assert_eq!(b.format(vec![2]).unwrap(), "000002");
        assert_eq!(b.format(vec![2, 4, 6, 8]).unwrap(), "02040608");
    }

    // clip tests

    #[test]
    fn prefix_clip() {
        let b = BinaryFormat::new().width(6);
        assert_eq!(b.format(vec![]).unwrap(), "    0x");
        assert_eq!(b.format(vec![2]).unwrap(), "  0x02");
        assert_eq!(b.format(vec![2, 4, 6, 8]).unwrap(), "0x0...");
    }

    #[test]
    fn no_prefix_clip() {
        let b = BinaryFormat::new().width(6).no_prefix();
        assert_eq!(b.format(vec![]).unwrap(), "      ");
        assert_eq!(b.format(vec![2]).unwrap(), "    02");
        assert_eq!(b.format(vec![2, 4, 6, 8]).unwrap(), "020...");
    }
}
