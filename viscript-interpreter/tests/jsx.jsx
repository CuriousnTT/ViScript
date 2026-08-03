// ============================================================
// JSX LEXER REGRESSION TEST
//
// r = following cases should succeed
// e = following cases should produce lexer errors
// ============================================================
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

// division vs regex vs JSX

const a = x / y / z;

const b = foo(bar) / baz;

const c = (a + b) / (c + d);

const d = obj.value / arr[index];

const re = /abc/;

const re2 = /[a-z]+/gi;

const re3 = /foo\/bar/;

const re4 = /https?:\/\/[^/]+/;

return /abc/.test(x);

throw /error/i;

const x = condition ? /yes/ : /no/;

// JSX regex boundary tests
r

// / after ) is division, not regex
const x = (
  <div />
) / 2;

// regex in JSX expression
const x = (
  <>
    {/abc/.test(value)}
  </>
);

// regex in JSX attribute expression
const x = (
  <div prop={/abc/.exec(str)} />
);

// self-closing tags
r

const x = <A/>;

const y = <A />;

const z = <A a={1} b="2" />;

const nested =
  <Outer>
    <Inner />
  </Outer>;

// closing tags
r

const el =
  <div>
    hello
  </div>;

// nested
const el =
  <div>
    <span>
      text
    </span>
  </div>;

// with fragments
const el =
  <>
    <A />
    <B />
  </>;

// regex immediately after JSX

const value =
  (<Component />)
/abc/.test(str);

let result = (
  <Component />
)
? /yes/
: /no/;

const el =
  <div>
    {`path/${id}`}
  </div>;


// division in template interpolation
const el =
  <div>
    {`${a / b}`}
  </div>;

// regex in template interpolation
const el =
  <div>
    {`${/abc/.source}`}
  </div>;
``

// nested JSX expressions
r

const el =
  <div>
    {
      condition
        ? <A />
        : <B />
    }
  </div>;

r

const el =
  <div>
    {
      list.map(x => (
        <Item key={x.id} />
      ))
    }
  </div>;

// TSX
r

const el =
  <Component<number>
    value={123}
  />;

const el =
  <Component
    fn={(x: number) => x / 2}
  />;

// typed function call (not JSX at all)
const value =
  foo<Bar<Baz>>(arg);

// JSX with slashes in text
const el =
  <div>
    https://example.com/foo/bar
  </div>;


const el =
  <div>
    a/b/c
  </div>;

// intentional errors
e
// unopened tag may be parser responsibility
const x = </div>;

// self-closed empty tag
const x = <div />;

// unterminated regex
const re = /abc;

// inside JSX expression
const x =
  <div>
    {/abc}
  </div>;

// hard mode
r

const el =
  <Component
    matcher={/^foo\/bar$/}
    render={() => (
      <div>
        {`${items.filter(x => /abc/.test(x)).length / total}`}
      </div>
    )}
  />;

// unclosed tag
e
const x = <div>;