let price = 19.99;
let quantity = 2;
let taxRate = 0.15;
let discount = 5;

function subtotal() { // Runs immediately after being defined in ViScript
    console.log(`Subtotal: $${(price * quantity).toFixed(2)}`);
}

function tax() {
    console.log(`Tax: $${(price * quantity * taxRate).toFixed(2)}`);
}

function total() {
    const beforeTax = price * quantity - discount; /* could be defined with 'rel' in place of 'const',
    outside the function, if this were ViScript. It would behave the same. */
    const afterTax = beforeTax * (1 + taxRate); // The same is true of afterTax

    console.log(`Total: $${afterTax.toFixed(2)}`);
}

console.log("Initial values:");
subtotal();
tax();
total();

console.log("\nChanging quantity..."); // This does nothing else in JavaScript
quantity = 4;

console.log("\nChanging discount..."); // This does a bit more in ViScript
discount = 10;