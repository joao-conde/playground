package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"

	"github.com/emirpasic/gods/sets/hashset"
)

func main() {
	input, _ := os.Open("input/day01")
	defer input.Close()

	scanner := bufio.NewScanner(input)

	nums := hashset.New()
	for scanner.Scan() {
		num, _ := strconv.Atoi(scanner.Text())
		nums.Add(num)
	}

	fmt.Println("Part1:", p1(nums))
	fmt.Println("Part2:", p2(nums))
}

func p1(nums *hashset.Set) int {
	x, y, _ := findPair(nums, 2020)
	return x * y
}

func p2(nums *hashset.Set) int {
	for _, num := range nums.Values() {
		target := 2020 - num.(int)
		x, y, found := findPair(nums, target)
		if found {
			return num.(int) * x * y
		}
	}
	panic("no three nums amount to the desired goal")
}

func findPair(nums *hashset.Set, target int) (int, int, bool) {
	for _, num := range nums.Values() {
		pair := target - num.(int)
		if nums.Contains(pair) {
			return num.(int), pair, true
		}
	}
	return 0, 0, false
}
