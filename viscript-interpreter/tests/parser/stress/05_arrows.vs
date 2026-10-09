// ------------------------------------------------------------
// 5. Arrow functions
// ------------------------------------------------------------

let increment = (value: number) => value + 1;

let multiply = (left: number, right: number) => {
    return left * right;
};

let getAnswer = () => 42;

let greetUser = (name: string) => {
    let greeting = "Hello, " + name;
    return greeting;
};