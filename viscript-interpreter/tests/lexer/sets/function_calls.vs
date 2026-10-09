foo([|"a", "b"|]);

foo([|"a"|], [|"b"|]);

foo([|"a"|], ["b"]);

foo(
    [|"a", "b"|],
    [1, 2, 3],
    ("tuple", 42)
);

const result = foo([|"a", b, c|]);

const nestedResult = foo([|foo(a), bar(b), baz(c)|]);