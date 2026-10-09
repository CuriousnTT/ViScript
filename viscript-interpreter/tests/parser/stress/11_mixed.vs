// ------------------------------------------------------------
// 11. Mixed constructs
// ------------------------------------------------------------

function processUser(user: [name: string, age: number]) {
    let label = `User: ${user}`;

    if (user) {
        let result = {
            label: label,
            age: user.age
        };

        return result;
    }

    return null;
}

let pipeline = (value: number) => {
    let intermediate = value * 2;

    if (intermediate > 10) {
        return intermediate + 1;
    } else {
        return intermediate - 1;
    }
};