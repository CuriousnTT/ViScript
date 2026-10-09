const key = "values";

const object = {
    [key]: [|1, 2, 3|]
};

const indexed = object[key];

const setValue = object[key][0];

const setFromComputedProperty = object[[|"values"|]];