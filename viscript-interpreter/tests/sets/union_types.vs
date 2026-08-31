type StringOrSet: string | string[|];

type SetOrNumber: string[|] | number;

type MultiTypedSet2: (string | boolean | number)[|];

type TupleWithUnion: (string | number, boolean);

type SetOfUnion: (string | number | boolean)[|];