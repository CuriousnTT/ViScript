// should *not* become VSX
function f<T>(x: T) {}
const g = <T,>(x: T) => x;
const h = <T extends string>(x: T) => x;
foo<Bar>();
new Foo<Bar>();
type X = Array<string>;

// should become VSX
const el = <div />;
const el2 = <Foo bar={1}>;
const el3 = <>fragment</>;
const el4 = <Foo<string> />;   // generic component - still VSX
return <Bar />;