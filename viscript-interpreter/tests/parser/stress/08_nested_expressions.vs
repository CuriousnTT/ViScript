// ------------------------------------------------------------
// 8. Nested expressions and operator precedence
// ------------------------------------------------------------

let nestedArithmetic = (1 + 2) * (3 + 4);

let nestedCalls = outer(inner(1), inner(2));

let nestedObjects = {
    first: {
        second: {
            value: 123
        }
    }
};

let combined = add(1, 2) * increment(3) + 4;