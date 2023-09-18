function reverseString(str) {
    return str.split("").reverse().join("");
}

const str = prompt("Enter a string: ");
console.log(`Reversed string: ${reverseString(str)}`);
