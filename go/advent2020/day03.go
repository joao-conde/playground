package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	input, _ := os.Open("input/day03")
	defer input.Close()

	scanner := bufio.NewScanner(input)
	scanner.Split(bufio.ScanLines)

	area := make([]string, 0)
	for i := 0; scanner.Scan(); i++ {
		line := scanner.Text()
		area = append(area, line)
	}

	p1 := hits(area, 1, 3)
	p2 := p1 *
		hits(area, 1, 1) *
		hits(area, 1, 5) *
		hits(area, 1, 7) *
		hits(area, 2, 1)

	fmt.Println("Part1:", p1)
	fmt.Println("Part2:", p2)
}

func hits(area []string, slopei int, slopej int) int {
	height, width, hits := len(area), len(area[0]), 0
	for i, j := 0, 0; i < height; i, j = i+slopei, j+slopej {
		j = j % width
		if area[i][j] == '#' {
			hits++
		}
	}
	return hits
}

func maxInt(x int, y int) int {
	if x > y {
		return x
	}
	return y
}
