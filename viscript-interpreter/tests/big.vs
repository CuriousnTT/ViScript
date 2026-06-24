let x = 1&log;
rel y = 3 + x;
const z = { value: x };
function myEffect() {
    if (y > 5) {
        x = 1;
    }
}