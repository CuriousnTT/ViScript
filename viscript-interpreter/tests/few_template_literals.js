// ============================================================
// TEMPLATE LITERAL LEXER REGRESSION TEST
//
// r = following cases should succeed
// e = following cases should produce lexer errors
// ============================================================

`\u123`
`\u123Z`
`\u{110000}`
`\x4`
`\x4Z`
`\`
