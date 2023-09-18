package main

import "fmt"

func main() {
    total := 0
    for i := 2; i <= 100; i += 2 {
        total += i
    }
    fmt.Printf("Sum of even numbers from 2 to 100: %d\n", total)
}
