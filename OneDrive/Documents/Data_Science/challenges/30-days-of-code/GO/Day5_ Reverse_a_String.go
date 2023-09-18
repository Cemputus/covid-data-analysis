package main

import "fmt"

func reverseString(str string) string {
    runes := []rune(str)
    for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
        runes[i], runes[j] = runes[j], runes[i]
    }
    return string(runes)
}

func main() {
    var str string
    fmt.Print("Enter a string: ")
    fmt.Scan(&str)

    reversed := reverseString(str)
    fmt.Printf("Reversed string: %s\n", reversed)
}
