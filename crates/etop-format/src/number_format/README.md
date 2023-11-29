**ADAPTED FROM https://github.com/askanium/format_num/**

Dynamic formatting of numbers into human readable forms.

Did you encounter cases where Rust doesn't represent numbers the way you expect?

```
for i in 1..=10 {
    println!("{}", 0.1 * i as f64);
}
```

You get this:

```text
0.1
0.2
0.30000000000000004
0.4
0.5
0.6000000000000001
0.7000000000000001
0.8
0.9
1
```

That's actually not a Rust issue, but rather [how floats are represented in binary](https://en.wikipedia.org/wiki/Double-precision_floating-point_format).

Yet rounding error is not the only reason to customize number formatting. A table of numbers
should be formatted consistently for comparison; above, 1.0 would be better than 1. Large
numbers may need to have grouped digits (e.g. 42,000) or be in scientific or metric notation
(4.2e+4, 42k). Reported numerical results should be rounded to significant digits (4021 becomes
4000) and so on.

The parser is modeled after Python 3's [format specification mini-language](https://docs.python.org/3/library/string.html#format-specification-mini-language)
[(PEP3101)](https://www.python.org/dev/peps/pep-3101/) with some minor implementation details changes.

The general form of a format specifier is:

```text
[[fill]align][sign][symbol][0][width][,][.precision][type]
```

The _fill_ can be any character. The presence of a fill character is signaled by the align
character following it, which must be one of the following:

`>` - Forces the field to be right-aligned within the available space.

`<` - Forces the field to be left-aligned within the available space.

`^` - Forces the field to be centered within the available space.

`=` - like `>`, but with any sign and symbol to the left of any padding.

The _sign_ can be:

`-` - nothing for zero or positive and a minus sign for negative (default behavior).

`+` - a plus sign for zero or positive and a minus sign for negative.

` ` (space) - a space for zero or positive and a minus sign for negative.

The _symbol_ can be:

The `#` option causes the “alternate form” to be used for the conversion. The alternate
form is defined differently for different types. For integers, when binary (`b`), octal
(`o` or `O`), or hexadecimal (`x` or `X`) output is used, this option adds the prefix
respective "0b", "0o", "0O" or "0x" to the output value. For floats, the alternate form
causes the result of the conversion to always contain a decimal-point character,
even if no digits follow it.

The zero (0) option enables zero-padding; this implicitly sets fill to 0 and align to =.

The _width_ defines the minimum field width; if not specified, then the width will be
determined by the content.

The comma (,) option enables the use of a group separator, such as a comma for thousands.

Depending on the _type_, the _precision_ either indicates the number of digits that follow
the decimal point (types `f` and `%`), or the number of significant digits (types `e`
and `s`). If the precision is not specified, it defaults to 6 for all types. Precision
is ignored for integer formats (types `b`, `o`, `d`, `x` and `X`).

The available _type_ values are:

`e` - exponent notation.

`f` - fixed point notation.

`s` - decimal notation with an SI prefix, rounded to significant digits.

`%` - multiply by 100, and then decimal notation with a percent sign.

`b` - binary notation, rounded to integer.

`o` - octal notation, rounded to integer.

`d` - decimal notation, rounded to integer.

`x` - hexadecimal notation, using lower-case letters, rounded to integer.

`X` - hexadecimal notation, using upper-case letters, rounded to integer.


# Examples

```
use etop_format::NumberFormat;

let num = NumberFormat::new();

assert_eq!(num.format(".1f", 0.06), "0.1");
assert_eq!(num.format("#.0f", 10.1), "10."); // float alternate form (always show a decimal point)
assert_eq!(num.format("+14d", 2_147_483_647), "   +2147483647");
assert_eq!(num.format("#b", 3), "0b11");
assert_eq!(num.format("b", 3), "11");
assert_eq!(num.format("#X", 48879), "0xBEEF");
assert_eq!(num.format(".2s", 42e6), "42M");
assert_eq!(num.format(".^20d", 12), ".........12........."); // dot filled and centered
assert_eq!(num.format("+10.0f", 255), "      +255");
assert_eq!(num.format(".0%", 0.123), "12%");
assert_eq!(num.format("+016,.2s", 42e12), "+000,000,000,042T"); // grouped zero-padded with a mandatory sign, SI-prefixed with 2 significant digits
```

# Note

A current limitation is that the number to be formatted should implement the `Into<f64>`
trait. While this covers a broad range of use cases, for big numbers (>u64::MAX) some
precision will be lost.
