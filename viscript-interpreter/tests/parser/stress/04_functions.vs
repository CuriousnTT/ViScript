// ------------------------------------------------------------
// 4. Function declarations and returns
// ------------------------------------------------------------

function add(a: number, b: number): number {
    return a + b;
}

function greet(name: string) {
    return "Hello, " + name;
}

function doNothing() {
    return;
}

function identity(value) {
    return value;
}

function choose(flag: boolean, left: string, right: string) {
    if (flag) {
        return left;
    } else {
        return right;
    }
}