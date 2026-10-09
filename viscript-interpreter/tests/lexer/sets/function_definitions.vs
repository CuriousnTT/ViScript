function getSet() {
    return [|"a", "b", "c"|];
}

function useSet(value) {
    console.log(value);
}

function useBoth(array, set) {
    return [array, set];
}