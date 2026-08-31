const tuple = ("a", 1, true);

const array = ["a", 1, true];

const set = [|"a", 1, true|];

const tupleContainingArray = (["a", "b"], 1);

const tupleContainingSet = ([|"a", "b"|], 1);

const arrayContainingTuple = [("a", "b"), 1];

const arrayContainingSet = [[|"a", "b"|], 1];

const setContainingTuple = [|("a", "b"), 1|];

const setContainingArray = [|["a", "b"], 1|];