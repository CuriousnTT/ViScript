let x = 1;

rel stringSetReactive = [|"a", "b", "c"|];

rel numberSetReactive = [|1, 2, x|];

rel nestedReactive = [|x, x + 1, x + 2|];

function effect() {
    console.log([|x, x * 2, x * 3|]);
}

let loggedSet = [|"a", "b"|] &log;

rel loggedReactiveSet = [|x, x + 1|] &log;
