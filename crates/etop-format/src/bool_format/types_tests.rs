#[cfg(test)]
mod types_tests {
    use crate::bool_format::BoolFormat;

    #[test]
    fn default() {
        let b = BoolFormat::default();
        assert_eq!(b.format(true).unwrap(), "true");
        assert_eq!(b.format(false).unwrap(), "false");
    }

    // alignment tests

    #[test]
    fn prefix_left_align() {
        let b = BoolFormat::new().min_width(6).left_align();
        assert_eq!(b.format(true).unwrap(), "true  ");
        assert_eq!(b.format(false).unwrap(), "false ");
    }

    #[test]
    fn prefix_right_align() {
        let b = BoolFormat::new().min_width(6).right_align();
        assert_eq!(b.format(true).unwrap(), "  true");
        assert_eq!(b.format(false).unwrap(), " false");
    }

    #[test]
    fn prefix_left_align_zero_padding() {
        let b = BoolFormat::new().min_width(6).left_align().fill_char('0');
        assert_eq!(b.format(true).unwrap(), "true00");
        assert_eq!(b.format(false).unwrap(), "false0");
    }

    #[test]
    fn prefix_right_align_zero_padding() {
        let b = BoolFormat::new().min_width(6).right_align().fill_char('0');
        assert_eq!(b.format(true).unwrap(), "00true");
        assert_eq!(b.format(false).unwrap(), "0false");
    }

    // clip tests

    #[test]
    fn prefix_clip() {
        let b = BoolFormat::new().width(6);
        assert_eq!(b.format(true).unwrap(), "  true");
        assert_eq!(b.format(false).unwrap(), " false");
    }

    // alternative text

    #[test]
    fn alternative_text() {
        let b = BoolFormat::new()
            .true_text("y".into())
            .false_text("NO".into());
        assert_eq!(b.format(true).unwrap(), "y");
        assert_eq!(b.format(false).unwrap(), "NO");
    }
}
