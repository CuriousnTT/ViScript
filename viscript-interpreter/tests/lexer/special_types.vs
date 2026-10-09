type MyTuple: (string, number, boolean);
type MyNamedTuple: (name: string, age: number, married: boolean);

type StringSet: string{};
type MultiTypedSet: (string | boolean | number){};

let base = 1 &log;
rel computed = base + 2;

const tuple: MyTuple = ("Warhammer", 40000, false);
const named: MyNamedTuple = ("Elizabeth", 20, false);

console.log(tuple[0]);
console.log(named.name);

function trigger() {
    if (computed > 5) {
        base = 1;
    }
}