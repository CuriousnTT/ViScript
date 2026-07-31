// ============================================================
// TEMPLATE LITERAL LEXER REGRESSION TEST
// Language-limitiations:
// Interpolation can't stop before a valid closing brace or EOF
// A template can't stop before a closing ` or EOF
// This limits testability for variants of these failures
//
// r = following cases should succeed
// e = following cases should produce lexer errors
// ============================================================

r

// basic templates
``
`hello world`

// multiline
`
one
two
three
`

// tagged templates
myTag`hello`
myTag`\ubad tag`

// escapes
`\``
`\\`
`\n\r\t\b\f\v`
`\u1234`
`\u{1F600}`
`\x41`

// dollar handling
`$`
`$$${x}`
`\${x}`
`\\${x}`

// interpolations
`${x}`
`hello ${name}!`
`${a}${b}${c}`

// expressions
`${ { a: 1 } }`
`${"}"}`
`${`nested`}`

// comment in interpolation
`${
    // just for show
    a
}`


// JSX/TSX-like
r

const el = (
  <Component
    data={`${{ id: user.id, label: `User ${user.name}` }}`}
  />
);

function render(items) {
  return (
    <ul>
      {items.map(item => (
        <li key={item.id}>
          {`${item.name}: ${
            item.details
              ? `extra ${item.details.note ?? `fallback-${item.id}`}`
              : 'none'
          }`}
        </li>
      ))}
    </ul>
  );
}

const html = html`<div class="${cls}">${
  show ? html`<span>${text}</span>` : null
}</div>`;

// lexer-vs-parser boundary tests
r

`${if}`
`${return}`
`${123abc}`
`${!!!}`
`${=>}`
`${foo +}`
`${(() => {})}`
`${'}'}`
// stress tests

r

// nested template transitions
`${`${x}`}`
`${`${`${x}`}`}`

// repeated transitions
`a${b}c${d}e${f}g`

// large runs
`aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa`
`${aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa}`

// recoverability
e

// invalid unicode escapes
`\u`
`\u{}`
`\u{xyz}`

// invalid hex escapes
`\x`
`\xGG`

// unknown escape
`\q`

// unterminated interpolation
`${hello`

// unterminated template after interpolation
`hello ${name}

// final recovery cases
`first bad template

`\u{XYZ}

`\xZZ

`${missing

`${{`

\