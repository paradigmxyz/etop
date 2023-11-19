#[cfg(test)]
mod types_tests {
    // use crate::format_num::types::NumberFormat;
    use crate::format_num::format;
    use crate::format_num::process;

    // #[test]
    // fn initialization() {
    //     let num = NumberFormat::new();
    //     assert_eq!(num.decimal, '.');
    //     assert_eq!(num.group_delimiter, ',');
    // }

    #[test]
    fn significant_digits() {
        assert_eq!(process::get_significant_digits("81"), 2);
        assert_eq!(process::get_significant_digits("26.2"), 3);
        assert_eq!(process::get_significant_digits("0.004"), 1);
        assert_eq!(process::get_significant_digits("5200.38"), 6);
        assert_eq!(process::get_significant_digits("380.0"), 4);
        assert_eq!(process::get_significant_digits("78800"), 3);
        assert_eq!(process::get_significant_digits("78800."), 5);
    }

    #[test]
    fn precision_0_percentage() {
        assert_eq!(format(".0%", 0), "0%");
        assert_eq!(format(".0%", 0.042), "4%");
        assert_eq!(format(".0%", 0.42), "42%");
        assert_eq!(format(".0%", 4.2), "420%");
        assert_eq!(format(".0%", -0.042), "-4%");
        assert_eq!(format(".0%", -0.42), "-42%");
        assert_eq!(format(".0%", -4.2), "-420%");
    }

    #[test]
    fn precision_gt_0_percentage() {
        assert_eq!(format(".1%", 0.234), "23.4%");
        assert_eq!(format(".1%", 0.23456), "23.5%");
        assert_eq!(format(".2%", 0.234), "23.40%");
    }

    #[test]
    fn percentage_forms() {
        assert_eq!(format("020.0%", 12), "0000000000000001200%");
        assert_eq!(format("20.0%", 12), "               1200%");
        assert_eq!(format("^21.0%", 0.12), "         12%         ");
        assert_eq!(format("^21,.0%", 122), "       12,200%       ");
        assert_eq!(format("^21,.0%", -122), "      -12,200%       ");
    }

    #[test]
    fn grouping() {
        assert_eq!(format("01,.0d", 0), "0");
        assert_eq!(format("02,.0d", 0), "00");
        assert_eq!(format("03,.0d", 0), "000");
        assert_eq!(format("04,.0d", 0), "0,000");
        assert_eq!(format("05,.0d", 0), "0,000");
        assert_eq!(format("08,.0d", 0), "0,000,000");
        assert_eq!(format("013,.0d", 0), "0,000,000,000");
        assert_eq!(format("021,.0d", 0), "0,000,000,000,000,000");
        assert_eq!(format("013,.8d", -42000000), "-0,042,000,000");
    }

    #[test]
    fn zeroes() {
        assert_eq!(format(".0f", 0), "0");
        assert_eq!(format(".1f", 0), "0.0");
        assert_eq!(format(".2f", 0), "0.00");
        assert_eq!(format(".3f", 0), "0.000");
        assert_eq!(
            format(".50f", 0),
            "0.00000000000000000000000000000000000000000000000000"
        );
    }

    #[test]
    fn precision_0() {
        // for precision 0, result should never include a .
        assert_eq!(format(".0f", 1.5), "2");
        assert_eq!(format(".0f", 2.5), "2");
        assert_eq!(format(".0f", 3.5), "4");
        assert_eq!(format(".0f", 0.0), "0");
        assert_eq!(format(".0f", 0.1), "0");
        assert_eq!(format(".0f", 0.001), "0");
        assert_eq!(format(".0f", 10.0), "10");
        assert_eq!(format(".0f", 10.1), "10");
        assert_eq!(format(".0f", 10.01), "10");
        assert_eq!(format(".0f", 123.456), "123");
        assert_eq!(format(".0f", 1234.56), "1235");
        assert_eq!(
            format(".0f", 1e49),
            "9999999999999999464902769475481793196872414789632"
        );
        assert_eq!(
            format(".0f", 9.9999999999999987e+49),
            "99999999999999986860582406952576489172979654066176"
        );
        assert_eq!(
            format(".0f", 1e50),
            "100000000000000007629769841091887003294964970946560"
        );
    }

    #[test]
    fn precision_1() {
        assert_eq!(format(".1f", 0.0001), "0.0");
        assert_eq!(format(".1f", 0.001), "0.0");
        assert_eq!(format(".1f", 0.01), "0.0");
        assert_eq!(format(".1f", 0.04), "0.0");
        assert_eq!(format(".1f", 0.06), "0.1");
        assert_eq!(format(".1f", 0.25), "0.2");
        assert_eq!(format(".1f", 0.75), "0.8");
        assert_eq!(format(".1f", 1.4), "1.4");
        assert_eq!(format(".1f", 1.5), "1.5");
        assert_eq!(format(".1f", 10.0), "10.0");
        assert_eq!(format(".1f", 1000.03), "1000.0");
        assert_eq!(format(".1f", 1234.5678), "1234.6");
        assert_eq!(format(".1f", 1234.7499), "1234.7");
        assert_eq!(format(".1f", 1234.75), "1234.8");
    }

    #[test]
    fn precision_2() {
        assert_eq!(format(".2f", 0.0001), "0.00");
        assert_eq!(format(".2f", 0.001), "0.00");
        assert_eq!(format(".2f", 0.004999), "0.00");
        assert_eq!(format(".2f", 0.005001), "0.01");
        assert_eq!(format(".2f", 0.01), "0.01");
        assert_eq!(format(".2f", 0.125), "0.12");
        assert_eq!(format(".2f", 0.375), "0.38");
        assert_eq!(format(".2f", 1234500), "1234500.00");
        assert_eq!(format(".2f", 1234560), "1234560.00");
        assert_eq!(format(".2f", 1234567), "1234567.00");
        assert_eq!(format(".2f", 1234567.8), "1234567.80");
        assert_eq!(format(".2f", 1234567.89), "1234567.89");
        assert_eq!(format(".2f", 1234567.891), "1234567.89");
        assert_eq!(format(".2f", 1234567.8912), "1234567.89");
    }

    #[test]
    fn decimal_alternate_form() {
        // alternate form always includes a decimal point.  This only
        // makes a difference when the precision is 0.
        assert_eq!(format("#.0f", 0), "0.");
        assert_eq!(format("#.1f", 0), "0.0");
        assert_eq!(format("#.0f", 1.5), "2.");
        assert_eq!(format("#.0f", 2.5), "2.");
        assert_eq!(format("#.0f", 10.1), "10.");
        assert_eq!(format("#.0f", 1234.56), "1235.");
        assert_eq!(format("#.1f", 1.4), "1.4");
        assert_eq!(format("#.2f", 0.375), "0.38");
    }

    #[test]
    fn default_precision() {
        assert_eq!(format("f", 0), "0.000000");
        assert_eq!(format("f", 1230000), "1230000.000000");
        assert_eq!(format("f", 1234567), "1234567.000000");
        assert_eq!(format("f", 123.4567), "123.456700");
        assert_eq!(format("f", 1.23456789), "1.234568");
        assert_eq!(format("f", 0.00012), "0.000120");
        assert_eq!(format("f", 0.000123), "0.000123");
        assert_eq!(format("f", 0.00012345), "0.000123");
        assert_eq!(format("f", 0.000001), "0.000001");
        assert_eq!(format("f", 0.0000005001), "0.000001");
        assert_eq!(format("f", 0.0000004999), "0.000000");
    }

    // 'e' code formatting with explicit precision (>= 0). Output should always
    // have exactly the number of places after the point that were requested.
    #[test]
    fn zeroes_exp() {
        assert_eq!(format(".0e", 0), "0e+00");
        assert_eq!(format(".1e", 0), "0.0e+00");
        assert_eq!(format(".2e", 0), "0.00e+00");
        assert_eq!(format(".10e", 0), "0.0000000000e+00");
        assert_eq!(
            format(".50e", 0),
            "0.00000000000000000000000000000000000000000000000000e+00"
        );
    }

    #[test]
    fn precision_0_exp() {
        // no decimal point in the output
        assert_eq!(format(".0e", 0.01), "1e-02");
        assert_eq!(format(".0e", 0.1), "1e-01");
        assert_eq!(format(".0e", 1), "1e+00");
        assert_eq!(format(".0e", 10), "1e+01");
        assert_eq!(format(".0e", 100), "1e+02");
        assert_eq!(format(".0e", 0.012), "1e-02");
        assert_eq!(format(".0e", 0.12), "1e-01");
        assert_eq!(format(".0e", 1.2), "1e+00");
        assert_eq!(format(".0e", 12), "1e+01");
        assert_eq!(format(".0e", 120), "1e+02");
        assert_eq!(format(".0e", 123.456), "1e+02");
        assert_eq!(format(".0e", 0.000123456), "1e-04");
        assert_eq!(format(".0e", 123456000), "1e+08");
        assert_eq!(format(".0e", 0.5), "5e-01");
        assert_eq!(format(".0e", 1.4), "1e+00");
        assert_eq!(format(".0e", 1.5), "2e+00");
        assert_eq!(format(".0e", 1.6), "2e+00");
        assert_eq!(format(".0e", 2.4999999), "2e+00");
        assert_eq!(format(".0e", 2.5), "2e+00");
        assert_eq!(format(".0e", 2.5000001), "3e+00");
        assert_eq!(format(".0e", 3.499999999999), "3e+00");
        assert_eq!(format(".0e", 3.5), "4e+00");
        assert_eq!(format(".0e", 4.5), "4e+00");
        assert_eq!(format(".0e", 5.5), "6e+00");
        assert_eq!(format(".0e", 6.5), "6e+00");
        assert_eq!(format(".0e", 7.5), "8e+00");
        assert_eq!(format(".0e", 8.5), "8e+00");
        assert_eq!(format(".0e", 9.4999), "9e+00");
        assert_eq!(format(".0e", 9.5), "1e+01");
        assert_eq!(format(".0e", 10.5), "1e+01");
        assert_eq!(format(".0e", 14.999), "1e+01");
        assert_eq!(format(".0e", 15), "2e+01");
    }

    #[test]
    fn precision_1_exp() {
        assert_eq!(format(".1e", 0.0001), "1.0e-04");
        assert_eq!(format(".1e", 0.001), "1.0e-03");
        assert_eq!(format(".1e", 0.01), "1.0e-02");
        assert_eq!(format(".1e", 0.1), "1.0e-01");
        assert_eq!(format(".1e", 1), "1.0e+00");
        assert_eq!(format(".1e", 10), "1.0e+01");
        assert_eq!(format(".1e", 100), "1.0e+02");
        assert_eq!(format(".1e", 120), "1.2e+02");
        assert_eq!(format(".1e", 123), "1.2e+02");
        assert_eq!(format(".1e", 123.4), "1.2e+02");
    }

    #[test]
    fn precision_2_exp() {
        assert_eq!(format(".2e", 0.00013), "1.30e-04");
        assert_eq!(format(".2e", 0.000135), "1.35e-04");
        assert_eq!(format(".2e", 0.0001357), "1.36e-04");
        assert_eq!(format(".2e", 0.0001), "1.00e-04");
        assert_eq!(format(".2e", 0.001), "1.00e-03");
        assert_eq!(format(".2e", 0.01), "1.00e-02");
        assert_eq!(format(".2e", 0.1), "1.00e-01");
        assert_eq!(format(".2e", 1), "1.00e+00");
        assert_eq!(format(".2e", 10), "1.00e+01");
        assert_eq!(format(".2e", 100), "1.00e+02");
        assert_eq!(format(".2e", 1000), "1.00e+03");
        assert_eq!(format(".2e", 1500), "1.50e+03");
        assert_eq!(format(".2e", 1590), "1.59e+03");
        assert_eq!(format(".2e", 1598), "1.60e+03");
        assert_eq!(format(".2e", 1598.7), "1.60e+03");
        assert_eq!(format(".2e", 1598.76), "1.60e+03");
        assert_eq!(format(".2e", 9999), "1.00e+04");
        assert_eq!(format(".2e", 5.4e120), "5.40e+120");
        assert_eq!(format(".2e", 5.4e-120), "5.40e-120");
    }

    #[test]
    fn default_precision_exp() {
        assert_eq!(format("e", 0), "0.000000e+00");
        assert_eq!(format("e", 165), "1.650000e+02");
        assert_eq!(format("e", 1234567), "1.234567e+06");
        assert_eq!(format("e", 12345678), "1.234568e+07");
        assert_eq!(format("e", 1.1), "1.100000e+00");
    }

    #[test]
    fn alternate_form_exp() {
        assert_eq!(format("#.0e", 0.01), "1.e-02");
        assert_eq!(format("#.0e", 0.1), "1.e-01");
        assert_eq!(format("#.0e", 1), "1.e+00");
        assert_eq!(format("#.0e", 10), "1.e+01");
        assert_eq!(format("#.0e", 100), "1.e+02");
        assert_eq!(format("#.0e", 0.012), "1.e-02");
        assert_eq!(format("#.0e", 0.12), "1.e-01");
        assert_eq!(format("#.0e", 1.2), "1.e+00");
        assert_eq!(format("#.0e", 12), "1.e+01");
        assert_eq!(format("#.0e", 120), "1.e+02");
        assert_eq!(format("#.0e", 123.456), "1.e+02");
        assert_eq!(format("#.0e", 0.000123456), "1.e-04");
        assert_eq!(format("#.0e", 123456000), "1.e+08");
        assert_eq!(format("#.0e", 0.5), "5.e-01");
        assert_eq!(format("#.0e", 1.4), "1.e+00");
        assert_eq!(format("#.0e", 1.5), "2.e+00");
        assert_eq!(format("#.0e", 1.6), "2.e+00");
        assert_eq!(format("#.0e", 2.4999999), "2.e+00");
        assert_eq!(format("#.0e", 2.5), "2.e+00");
        assert_eq!(format("#.0e", 2.5000001), "3.e+00");
        assert_eq!(format("#.0e", 3.499999999999), "3.e+00");
        assert_eq!(format("#.0e", 3.5), "4.e+00");
        assert_eq!(format("#.0e", 4.5), "4.e+00");
        assert_eq!(format("#.0e", 5.5), "6.e+00");
        assert_eq!(format("#.0e", 6.5), "6.e+00");
        assert_eq!(format("#.0e", 7.5), "8.e+00");
        assert_eq!(format("#.0e", 8.5), "8.e+00");
        assert_eq!(format("#.0e", 9.4999), "9.e+00");
        assert_eq!(format("#.0e", 9.5), "1.e+01");
        assert_eq!(format("#.0e", 10.5), "1.e+01");
        assert_eq!(format("#.0e", 14.999), "1.e+01");
        assert_eq!(format("#.0e", 15), "2.e+01");
        assert_eq!(format("#.1e", 123.4), "1.2e+02");
        assert_eq!(format("#.2e", 0.0001357), "1.36e-04");
    }

    #[test]
    fn decimal() {
        assert_eq!(format("d", 2_147_483_647), "2147483647");
        assert_eq!(format("d", -2_147_483_647), "-2147483647");
        assert_eq!(format("5d", -2_147_483_647), "-2147483647");
        assert_eq!(format("11d", -2_147_483_647), "-2147483647");
        assert_eq!(format("12d", -2_147_483_647), " -2147483647");
        assert_eq!(format("-12d", -2_147_483_647), " -2147483647");
        assert_eq!(format("012d", -2_147_483_647), "-02147483647");
        assert_eq!(format("-012d", -2_147_483_647), "-02147483647");
        assert_eq!(format("014d", -2_147_483_647), "-0002147483647");
        assert_eq!(format("014d", 2_147_483_647), "00002147483647");
        assert_eq!(format("0=+14d", 2_147_483_647), "+0002147483647");
        assert_eq!(format(">+14d", 2_147_483_647), "   +2147483647");
        assert_eq!(format(".^+14d", 2_147_483_647), ".+2147483647..");
        assert_eq!(format("+014d", 2_147_483_647), "+0002147483647");
        assert_eq!(format("+14d", 2_147_483_647), "   +2147483647");
        assert_eq!(format("14d", 2_147_483_647), "    2147483647");
        assert_eq!(format(".2d", 2_147_483_647), "2147483647");
        assert_eq!(format(".10d", 2_147_483_647), "2147483647");
        assert_eq!(format(".11d", 2_147_483_647), "2147483647");
        assert_eq!(format("12.11d", 2_147_483_647), "  2147483647");
    }

    #[test]
    fn bin() {
        assert_eq!(format("#b", 3), "0b11");
        assert_eq!(format("b", 3), "11");
        assert_eq!(format("+020b", 123), "+0000000000001111011");
        assert_eq!(format(" 020b", 123), " 0000000000001111011");
        assert_eq!(format("+#020b", 123), "+0b00000000001111011");
    }

    #[test]
    fn hex() {
        assert_eq!(format("x", 0xf12abcd), "f12abcd");
        assert_eq!(format("x", -0xf12abcd), "-f12abcd");
        assert_eq!(format("5x", -0xf12abcd), "-f12abcd");
        assert_eq!(format("8x", -0xf12abcd), "-f12abcd");
        assert_eq!(format("9x", -0xf12abcd), " -f12abcd");
        assert_eq!(format("-9x", -0xf12abcd), " -f12abcd");
        assert_eq!(format("09x", -0xf12abcd), "-0f12abcd");
        assert_eq!(format("-09x", -0xf12abcd), "-0f12abcd");
        assert_eq!(format("011x", -0xf12abcd), "-000f12abcd");
        assert_eq!(format("011x", 0xf12abcd), "0000f12abcd");
        assert_eq!(format("0=+11x", 0xf12abcd), "+000f12abcd");
        assert_eq!(format("0>+11x", 0xf12abcd), "000+f12abcd");
        assert_eq!(format("+11x", 0xf12abcd), "   +f12abcd");
        assert_eq!(format("11x", 0xf12abcd), "    f12abcd");
        assert_eq!(format(".2x", 0xf12abcd), "f12abcd");
        assert_eq!(format(".7x", 0xf12abcd), "f12abcd");
        assert_eq!(format(".8x", 0xf12abcd), "f12abcd");
        assert_eq!(format("9.8x", 0xf12abcd), "  f12abcd");
        assert_eq!(format("X", 0xf12abcd), "F12ABCD");
        assert_eq!(format("#X", 0xf12abcd), "0xF12ABCD");
        assert_eq!(format("#x", 0xf12abcd), "0xf12abcd");
        assert_eq!(format("#x", -0xf12abcd), "-0xf12abcd");
        assert_eq!(format("#13x", 0xf12abcd), "    0xf12abcd");
        assert_eq!(format("<#13x", 0xf12abcd), "0xf12abcd    ");
        assert_eq!(format("#013x", 0xf12abcd), "0x0000f12abcd");
        assert_eq!(format("#.9x", 0xf12abcd), "0xf12abcd");
        assert_eq!(format("#.9x", -0xf12abcd), "-0xf12abcd");
        assert_eq!(format("#13.9x", 0xf12abcd), "    0xf12abcd");
        assert_eq!(format("#013.9x", 0xf12abcd), "0x0000f12abcd");
        assert_eq!(format("+#.9x", 0xf12abcd), "+0xf12abcd");
        assert_eq!(format(" #.9x", 0xf12abcd), " 0xf12abcd");
        assert_eq!(format("+#.9X", 0xf12abcd), "+0xF12ABCD");
    }

    #[test]
    fn oct() {
        assert_eq!(format("o", 1234567890), "11145401322");
        assert_eq!(format("o", -1234567890), "-11145401322");
        assert_eq!(format("5o", -1234567890), "-11145401322");
        assert_eq!(format("8o", -1234567890), "-11145401322");
        assert_eq!(format("13o", -1234567890), " -11145401322");
        assert_eq!(format("-13o", -1234567890), " -11145401322");
        assert_eq!(format("013o", -1234567890), "-011145401322");
        assert_eq!(format("-013o", -1234567890), "-011145401322");
        assert_eq!(format("015o", -1234567890), "-00011145401322");
        assert_eq!(format("015o", 1234567890), "000011145401322");
        assert_eq!(format("0=+15o", 1234567890), "+00011145401322");
        assert_eq!(format("0>+15o", 1234567890), "000+11145401322");
        assert_eq!(format("+15o", 1234567890), "   +11145401322");
        assert_eq!(format("15o", 1234567890), "    11145401322");
        assert_eq!(format(".2o", 1234567890), "11145401322");
        assert_eq!(format(".7o", 1234567890), "11145401322");
        assert_eq!(format(".13o", 1234567890), "11145401322");
        assert_eq!(format("13.12o", 1234567890), "  11145401322");
        assert_eq!(format("O", 1234567890), "11145401322");
        assert_eq!(format("#O", 1234567890), "0O11145401322");
        assert_eq!(format("#o", 1234567890), "0o11145401322");
        assert_eq!(format("#o", -1234567890), "-0o11145401322");
        assert_eq!(format("#17o", 1234567890), "    0o11145401322");
        assert_eq!(format("<#17o", 1234567890), "0o11145401322    ");
        assert_eq!(format("#017o", 1234567890), "0o000011145401322");
        assert_eq!(format("#.13o", 1234567890), "0o11145401322");
        assert_eq!(format("#.13o", -1234567890), "-0o11145401322");
        assert_eq!(format("#17.13o", 1234567890), "    0o11145401322");
        assert_eq!(format("#017.13o", 1234567890), "0o000011145401322");
        assert_eq!(format("+#.13o", 1234567890), "+0o11145401322");
        assert_eq!(format(" #.13o", 1234567890), " 0o11145401322");
        assert_eq!(format("+#.13O", 1234567890), "+0O11145401322");
    }

    #[test]
    fn small_ints() {
        assert_eq!(format("d", 42), "42");
        assert_eq!(format("d", -42), "-42");
        assert_eq!(format("d", 42.0), "42");
        assert_eq!(format("#x", 1), "0x1");
        assert_eq!(format("#X", 1), "0x1");
        assert_eq!(format("#o", 1), "0o1");
        assert_eq!(format("#o", 0), "0o0");
        assert_eq!(format("o", 0), "0");
        assert_eq!(format("d", 0), "0");
        assert_eq!(format("#x", 0), "0x0");
        assert_eq!(format("#X", 0), "0x0");
        assert_eq!(format("x", 0x42), "42");
        assert_eq!(format("x", -0x42), "-42");
        assert_eq!(format("o", 0o42), "42");
        assert_eq!(format("o", -0o42), "-42");
    }

    #[test]
    fn si_prefix_default_precision() {
        assert_eq!(format("s", 0), "0.00000");
        assert_eq!(format("s", 1), "1.00000");
        assert_eq!(format("s", 10), "10.0000");
        assert_eq!(format("s", 100), "100.000");
        assert_eq!(format("s", 999.5), "999.500");
        assert_eq!(format("s", 999500), "999.500k");
        assert_eq!(format("s", 1000), "1.00000k");
        assert_eq!(format("s", 100), "100.000");
        assert_eq!(format("s", 1400), "1.40000k");
        assert_eq!(format("s", 1500.5), "1.50050k");
        assert_eq!(format("s", 0.00001), "10.0000µ");
        assert_eq!(format("s", 0.000001), "1.00000µ");
    }

    #[test]
    fn si_prefix_custom_precision() {
        assert_eq!(format(".3s", 0), "0.00");
        assert_eq!(format(".3s", 1), "1.00");
        assert_eq!(format(".3s", 10), "10.0");
        assert_eq!(format(".3s", 100), "100");
        assert_eq!(format(".3s", 999.5), "1.00k");
        assert_eq!(format(".3s", 999500), "1.00M");
        assert_eq!(format(".3s", 1000), "1.00k");
        assert_eq!(format(".3s", 1500.5), "1.50k");
        assert_eq!(format(".3s", 42e6), "42.0M");
        assert_eq!(format(".3s", 145500000), "146M");
        assert_eq!(format(".3s", 145999999.999999347), "146M");
        assert_eq!(format(".3s", 1e26), "100Y");
        assert_eq!(format(".3s", 0.000001), "1.00µ");
        assert_eq!(format(".3s", 0.009995), "10.0m");
        assert_eq!(format(".4s", 999.5), "999.5");
        assert_eq!(format(".4s", 999500), "999.5k");
        assert_eq!(format(".4s", 0.009995), "9.995m");
    }

    #[test]
    fn si_prefix_numbers_smaller_than_one_yocto() {
        assert_eq!(format(".8s", 1.29e-30), "0.0000013y"); // Note: rounded!
        assert_eq!(format(".8s", 1.29e-29), "0.0000129y");
        assert_eq!(format(".8s", 1.29e-28), "0.0001290y");
        assert_eq!(format(".8s", 1.29e-27), "0.0012900y");
        assert_eq!(format(".8s", 1.29e-26), "0.0129000y");
        assert_eq!(format(".8s", 1.29e-25), "0.1290000y");
        assert_eq!(format(".8s", 1.29e-24), "1.2900000y");
        assert_eq!(format(".8s", 1.29e-23), "12.900000y");
        assert_eq!(format(".8s", 1.29e-22), "129.00000y");
        assert_eq!(format(".8s", 1.29e-21), "1.2900000z");
        assert_eq!(format(".8s", -1.29e-30), "-0.0000013y"); // Note: rounded!
        assert_eq!(format(".8s", -1.29e-29), "-0.0000129y");
        assert_eq!(format(".8s", -1.29e-28), "-0.0001290y");
        assert_eq!(format(".8s", -1.29e-27), "-0.0012900y");
        assert_eq!(format(".8s", -1.29e-26), "-0.0129000y");
        assert_eq!(format(".8s", -1.29e-25), "-0.1290000y");
        assert_eq!(format(".8s", -1.29e-24), "-1.2900000y");
        assert_eq!(format(".8s", -1.29e-23), "-12.900000y");
        assert_eq!(format(".8s", -1.29e-22), "-129.00000y");
        assert_eq!(format(".8s", -1.29e-21), "-1.2900000z");
    }

    #[test]
    fn si_prefix_numbers_bigger_than_one_yotta() {
        assert_eq!(format(".8s", 1.23e+21), "1.2300000Z");
        assert_eq!(format(".8s", 1.23e+22), "12.300000Z");
        assert_eq!(format(".8s", 1.23e+23), "123.00000Z");
        assert_eq!(format(".8s", 1.23e+24), "1.2300000Y");
        assert_eq!(format(".8s", 1.23e+25), "12.300000Y");
        assert_eq!(format(".8s", 1.23e+26), "123.00000Y");
        assert_eq!(format(".8s", 1.23e+27), "1230.0000Y");
        assert_eq!(format(".8s", 1.23e+28), "12300.000Y");
        assert_eq!(format(".8s", 1.23e+29), "123000.00Y");
        assert_eq!(format(".8s", 1.23e+30), "1230000.0Y");
        assert_eq!(format(".8s", -1.23e+21), "-1.2300000Z");
        assert_eq!(format(".8s", -1.23e+22), "-12.300000Z");
        assert_eq!(format(".8s", -1.23e+23), "-123.00000Z");
        assert_eq!(format(".8s", -1.23e+24), "-1.2300000Y");
        assert_eq!(format(".8s", -1.23e+25), "-12.300000Y");
        assert_eq!(format(".8s", -1.23e+26), "-123.00000Y");
        assert_eq!(format(".8s", -1.23e+27), "-1230.0000Y");
        assert_eq!(format(".8s", -1.23e+28), "-12300.000Y");
        assert_eq!(format(".8s", -1.23e+29), "-123000.00Y");
        assert_eq!(format(".8s", -1.23e+30), "-1230000.0Y");
    }

    #[test]
    fn si_prefix_consistent_for_small_and_big_numbers() {
        assert_eq!(format(".0s", 1e-5), "10µ");
        assert_eq!(format(".0s", 1e-4), "100µ");
        assert_eq!(format(".0s", 1e-3), "1m");
        assert_eq!(format(".0s", 1e-2), "10m");
        assert_eq!(format(".0s", 1e-1), "100m");
        assert_eq!(format(".0s", 1e+0), "1");
        assert_eq!(format(".0s", 1e+1), "10");
        assert_eq!(format(".0s", 1e+2), "100");
        assert_eq!(format(".0s", 1e+3), "1k");
        assert_eq!(format(".0s", 1e+4), "10k");
        assert_eq!(format(".0s", 1e+5), "100k");
        assert_eq!(format(".4s", 1e-5), "10.00µ");
        assert_eq!(format(".4s", 1e-4), "100.0µ");
        assert_eq!(format(".4s", 1e-3), "1.000m");
        assert_eq!(format(".4s", 1e-2), "10.00m");
        assert_eq!(format(".4s", 1e-1), "100.0m");
        assert_eq!(format(".4s", 1e+0), "1.000");
        assert_eq!(format(".4s", 1e+1), "10.00");
        assert_eq!(format(".4s", 1e+2), "100.0");
        assert_eq!(format(".4s", 1e+3), "1.000k");
        assert_eq!(format(".4s", 1e+4), "10.00k");
        assert_eq!(format(".4s", 1e+5), "100.0k");
    }

    #[test]
    fn si_prefix_grouping() {
        assert_eq!(format("020,s", 42), "000,000,000,042.0000");
        assert_eq!(format("020,s", 42e12), "00,000,000,042.0000T");
        assert_eq!(format(",s", 42e30), "42,000,000Y");
    }

    #[test]
    fn negative_zero_correct_formatting() {
        assert_eq!(format("f", -1e-12), "0.000000");
        assert_eq!(format("+f", -0.0), "-0.000000");
        assert_eq!(format("+f", 0), "+0.000000");
        assert_eq!(format("+f", -1e-12), "-0.000000");
        assert_eq!(format("+f", 1e-12), "+0.000000");
    }
}
