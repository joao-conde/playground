package main

import (
	"fmt"
	"math"
)

const EPSILON = 0.0000001

func Sqrt(x float64) float64 {
	prev, sqrt := x, 1.0

	for math.Abs(prev-sqrt) > EPSILON {
		prev = sqrt
		sqrt = sqrt - (sqrt*sqrt-x)/(2.0*sqrt)
	}

	return sqrt
}

func main() {
	fmt.Println(Sqrt(2), math.Sqrt(2))
}
