const afterComma = [|"a"|], x = 1;

const afterColon = {
    value: [|"a", "b"|]
};

const afterSemicolon = [|"a"|];

const nestedCall = foo([|"a", "b"|]);

const nestedParens = ([|"a", "b"|]);

const nestedBraces = {
    set: [|"a", "b"|]
};

const nestedBrackets = [[|"a", "b"|]];