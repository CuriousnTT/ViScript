// should succeed
// ordinary decimal integers
0
1
9
10
42
999999999
1234567890
// decimal floating-point
0.0
1.0
3.14
123.456
999999.999999
.0
.1
.5
.123
0.
1.
42.
999.
// exponent notation
1e0
1e1
1e+1
1e-1
1E1
1E+1
1E-1
1.2e3
1.2E3
.5e2
5.e2
0.5e-2
999.999e+999
0e0
0e+0
0e-0
1e308
1e309
1e999999999
1e-999999999
1e309        // Infinity
1e-999999999 // 0
// hexadecimal literals
0x0
0x1
0xA
0xa
0xF
0xff
0xFF
0x123456789abcdef
0xAbCdEf
// binary literals
0b0
0b1
0b1010
0b11111111
0B101010
// octal literals (modern)
0o0
0o7
0o10
0o777
0O777
// octal literals (legacy, Annex B); Fails when strict
00
01
07
0123
0777
// nmeric separators
1_000
1_000_000
123_456_789
0_1
1_234.567_890
0.000_001
123_456.789_012
1e1_0
1.2e3_4
1e+1_0
1e-1_0
0xFF_FF
0xAB_CD_EF
0b1010_1010
0B1111_0000
0o777_777
0O123_456
// BigInt literals
0n
1n
42n
999999999999999999999999999999999999n
0x1n
0xFFn
0xabcdefn
0b1010n
0B1111n
0o777n
0O123n
1_000n
0xFF_FFn
0b1010_1010n
0o777_777n
123_456_789n
// weird but valid
0.
.0
5.e2
.5e2
0e+0
0e-0
0x0
0b0
0o0
0n
0x0n
0b0n
0o0n
1_2_3
1_2_3.4_5_6
1_2e3_4
1_2n

// surprising but valid - tokenizer challenges
// member access after integer
1..toString() // should tokenize as 1.   .   toString
42..valueOf()
// decimal point ambiguity
1.toString() // should fail as a syntax error
(1).toString()
1 .toString()
// Unary operators should tokenize separately
-1 // should tokenize as - 1
+1
-0
+0
-1n // should tokenize as - 1n
// not literals but identifiers
Infinity
NaN

// should fail - almost valid
// Decimal
01n
00n
1__2
1_.2
1._2
1e_2
1e+_2
1e-_2
1e2_
1_
_1
// Decimal point
.
..
1..
1...
1.e
1.e+
1.e-
.e1
// Exponents
1e
1E
1e+
1e-
1e++
1e--
1ee1
1e1e1
// hexadecimal
0x
0xg
0x_
0x_1
0x1_
0x1__2
0x1.2
0x1e2
0x1e2.3
// binary
0b
0b2
0b102
0b_
0b_1
0b1_
0b1__0
// octal
0o
0o8
0o9
0o78
0o_
0o_1
0o1_
// BigInt
1.n
1.0n
1e2n
1e+2n
1e-2n
1_n
1._0n
1.0_n
0xn
0x1.n
0x1en
0b1.0n
0o7.0n
