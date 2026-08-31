let price;
let quantity = 15;
let taxRate = 0.15;
let discount = 5;

function subtotal() { // Runs immediately after being defined in ViScript, but does nothing because price is undefined
    console.log(`Subtotal: $${(price * quantity).toFixed(2)}`);
}

function tax() {
    console.log(`Tax: $${(price * quantity * taxRate).toFixed(2)}`);
}

function total() {
    const beforeTax = price * quantity - discount;
    const afterTax = beforeTax * (1 + taxRate);


    console.log(`Total: $${afterTax.toFixed(2)}`);
}

console.log("Initial values:");
// The effects above have already run, but did nothing because price was undefined

function announce() { // runs immediately
    if (quantity > 10) { // replaces implicit if-check with explicit if-check
        console.log("Bulk order!");
    }
}

console.log("\nSetting price...");
price = 19.99; // This activates the first three effects, and as all dependencies are now defined, they will do something

console.log("\nChanging quantity...");
quantity = 10; // Activates all effects again

console.log("\nChanging discount...");
discount = 10; // Activates only total()

console.log("\nChanging quantity...");
quantity = undefined; /* re-runs the first three effects with no consequence.
Announce also, though a different if-check could have led to error */