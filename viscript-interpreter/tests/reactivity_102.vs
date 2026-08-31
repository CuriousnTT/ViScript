let price = 19.99;
let quantity = 2;
let taxRate = 0.15;
let discount = 5;

export function subtotal() { // no longer runs automatically
    console.log(`Subtotal: $${(price * quantity).toFixed(2)}`); // becomes a scoped snapshot
}

function tax(taxRate) { // no longer runs automatically
    console.log(`Tax: $${(price * quantity * taxRate).toFixed(2)}`); // price & quantity are scoped snapshots
}

rel beforeTax = price * quantity - discount;
rel afterTax = beforeTax * (1 + taxRate);

function total() { // no longer runs automatically
    console.log(`Total: $${afterTax.toFixed(2)}`); // scoped snapshot
    return afterTax.toFixed(2);
}

console.log("Initial values:");
subtotal();
tax(taxRate);
total();

console.log("\nChanging quantity..."); // does not affect subtotal or tax log, does not activate any effect (there are none)
quantity = 4;

console.log("\nChanging discount..."); // changes rel values but does not activate any effect
discount = 10;