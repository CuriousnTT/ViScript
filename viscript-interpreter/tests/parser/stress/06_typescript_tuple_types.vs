// ------------------------------------------------------------
// 6. Tuple types and parameter-list syntax
// These are not Tuples to ViScript, but a special case of Array type syntax
// ------------------------------------------------------------

// Unlabeled tuple elements.
let pair: [number, string];

// Labeled tuple elements.
let labeledPair: [x: number, y: string];

// Optional elements.
let optionalPair: [x: number, y?: string];

// Function parameter declarations.
function describePoint(point: [x: number, y: number]) {
    return point;
}

// Contextual interpretation of tuple-like parameter syntax.
let sumPair = (values: [left: number, right: number]) => {
    return values;
};