function factorial(n) {
    if (n === 0 || n === 1) {
        return 1;
    }
    return n * factorial(n - 1);
}

const num = prompt("Enter a number: ");
console.log(`Factorial of ${num} is ${factorial(num)}`);
