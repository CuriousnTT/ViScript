
const tightSet = [|1,2,3|];

const tightEmptySet = [||];

const setFollowedByIndex = [|1, 2, 3|][0];

const setFollowedByCall = [|1, 2, 3|]();

const setFollowedByProperty = [|1, 2, 3|].size;

const arrayFollowedBySet = [1, 2, 3][|"not an index"|];

const setFollowedByArray = [|"a"|][1, 2, 3];