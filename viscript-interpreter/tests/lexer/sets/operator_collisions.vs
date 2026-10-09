
const bitwiseOr = a | b;

const logicalOr = a || b;

const logicalOrAssignment = a ||= b;

const bitwiseOrAssignment = a |= b;

const setAfterOr = a | [|"x", "y"|];

const setBeforeOr = [|"x", "y"|] | b;

const setAroundOr = [|"x"|] | [|"y"|];

const setAroundLogicalOr = [|"x"|] || [|"y"|];

const setAroundAssignments = (
    [|"x"|] |= value
);

const setExpression = [|a | b, c || d|];

const finalArray = ["this array is at the end of a line"];