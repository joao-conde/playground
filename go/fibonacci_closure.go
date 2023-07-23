package main

import "fmt"

// fibonacci is a function that returns
// a function that returns an int
// (function closures)
func fibonacci() func() int {
	fib1, fib2 := -1, -1
	return func() int {
		if fib1 == -1 {
			fib1 = 0
			return fib1
		} else if fib2 == -1 {
			fib2 = 1
			return fib2
		}

		fib := fib1 + fib2
		fib1 = fib2
		fib2 = fib
		return fib
	}
}

func main() {
	f := fibonacci()
	for i := 0; i < 10; i++ {
		fmt.Println(f())
	}
}
