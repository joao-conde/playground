package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type policyFn func(char string, min int, max int, password string) bool

func main() {
	fmt.Println("Part1:", countValidPws(policy1))
	fmt.Println("Part2:", countValidPws(policy2))
}

func countValidPws(validPw policyFn) int {
	input, _ := os.Open("input/day02")
	defer input.Close()

	scanner := bufio.NewScanner(input)

	valid := 0
	for scanner.Scan() {
		line := scanner.Text()

		splits := strings.Split(line, ":")
		rule := strings.TrimSpace(splits[0])
		password := strings.TrimSpace(splits[1])

		splits = strings.Split(rule, " ")
		char := splits[1]

		splits = strings.Split(splits[0], "-")
		min, _ := strconv.Atoi(splits[0])
		max, _ := strconv.Atoi(splits[1])

		if validPw(char, min, max, password) {
			valid++
		}
	}

	return valid
}

func policy1(char string, min int, max int, password string) bool {
	count := 0
	for _, c := range password {
		if string(c) == char {
			count += 1
		}
	}
	return count >= min && count <= max
}

func policy2(char string, i int, j int, password string) bool {
	return (string(password[i-1]) == char) != (string(password[j-1]) == char)
}
