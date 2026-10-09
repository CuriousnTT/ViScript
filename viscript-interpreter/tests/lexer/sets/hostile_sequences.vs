const exactSetEnd = [|1|];

const exactBitwiseOrThenBracket = value | array[0];

const setThenIndex = [|1|][0];

const setThenBitwiseOr = [|1|] | value;

const bitwiseOrThenArray = value | [0];

const logicalOrThenArray = value || [0];

const arrayThenBitwiseOr = [0] | value;

const arrayThenLogicalOr = [0] || value;