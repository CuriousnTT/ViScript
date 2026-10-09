const arrayThenSet = [[1, 2, 3], [|"a", "b", "c"|]];

const setThenArray = [[|"a", "b", "c"|], [1, 2, 3]];

const arrayContainingSet = [
    [|"a", "b"|],
    [|1, 2|],
];

const setContainingArray = [|[1, 2], [3, 4]|];

const nestedSet = [|[|"a", "b"|], [|"c", "d"|]|];