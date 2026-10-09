let r01 = /abc/;
let r02 = /abc/i;
let r03 = /abc/gim;
let r04 = /^\d+$/;
let r05 = /\w+\s+\w+/;
let r06 = /hello|world/;
let r07 = /a*/;
let r08 = /a+?/;
let r09 = /a{3,5}/;
let r10 = /a{3,5}?/;

let r11 = /(abc)/;
let r12 = /(?:abc)/;
let r13 = /(?=abc)/;
let r14 = /(?!abc)/;
let r15 = /(?<=abc)/;
let r16 = /(?<!abc)/;

let r17 = /(?<year>\d{4})-(?<month>\d{2})-(?<day>\d{2})/;
let r18 = /(?<word>\w+)\s+\k<word>/;

let r19 = /[abc]/;
let r20 = /[^abc]/;
let r21 = /[a-z]/;
let r22 = /[a-zA-Z0-9_]/;
let r23 = /[-abc]/;
let r24 = /[abc-]/;
let r25 = /[a\-z]/;
let r26 = /[\]]/;

let r27 = /foo\/bar/;
let r28 = /https?:\/\/[^\/]+\/.*/;

let r29 = /\n\t\r\\\0/;
let r30 = /\x41\x42\x43/;
let r31 = /\u0041\u0042\u0043/;
let r32 = /\u{1F600}/u;

let r33 = /\p{Letter}+/u;
let r34 = /\P{Number}+/u;

let r35 = /[/]/;
let r36 = /[a/b]/;
let r37 = /[/a-z]/;
let r38 = /[a-z/]/;

let r39 = /^((https?|ftp):\/\/)?([^\/\s]+\/?)+$/;

let r40 = /^(?:(?<=\s)|^)\w+(?:(?=\s)|$)/;

let r41 = /multiline
test # also, hi
/x;
let r42 = /ab(?(condition)yes|no)c/;
let r43 = /ab(?R)\g<name>/m;

let e01 = /abc;
let e02 = /abc\/;
let e03 = /[abc/;
let e04 = /abc/q;
let e05 = /abc/gg;
let e06 = /(?<name>abc/;
let e07 = /multiline
fail # also hi
/gm;
let e08 = /abc//;
let e09 = /a{5,3}/;
let e10 = /(?<=a*)b/; // syntactically valid, rejected in JavaScript for lack of fixed-length lookbehind
let e11 = /\u{110000}/u;
let e12 = /[\p{Letter}]/; // lacks necessary unicode mode for property escape