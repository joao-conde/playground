package main

import "fmt"

func main() {
	slice := make([]int, 0)
	fmt.Println(len(slice), cap(slice))

	for i := 0; i < 10; i++ {
		slice = append(slice, i)
	}
	fmt.Println(len(slice), cap(slice), float64(cap(slice))/float64(len(slice)))

	for i := 0; i < 100; i++ {
		slice = append(slice, i)
	}
	fmt.Println(len(slice), cap(slice), float64(cap(slice))/float64(len(slice)))

	for i := 0; i < 1000; i++ {
		slice = append(slice, i)
	}
	fmt.Println(len(slice), cap(slice), float64(cap(slice))/float64(len(slice)))
}
