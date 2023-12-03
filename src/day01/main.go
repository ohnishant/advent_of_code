package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	if len(os.Args) != 2 {
		log.Fatal("Usage: go run main.go [1|2]")
	}
	switch os.Args[1] {
	case "1":
		one()
	case "2":
		two()
	default:
		log.Fatal("Usage: go run main.go [1|2]")
	}
}

func one() {
	f, err := os.Open("./src/day01/day01.txt")
	if err != nil {
		log.Fatalf("[Error]: failed to open file: %v", err)
	}
	scanner := bufio.NewScanner(f)

	sum := 0
	for scanner.Scan() {
		line := scanner.Text()
		sum += firstDigit(line)*10 + lastDigit(line)
	}

	fmt.Println(sum)
}

func two() {
	f, err := os.Open("./src/day01/day01.txt")
	if err != nil {
		log.Fatalf("[Error]: failed to open file: %v", err)
	}
	scanner := bufio.NewScanner(f)

	sum := 0
	for scanner.Scan() {
		line := scanner.Text()
		sum += firstNumber(line)*10 + lastNumber(line)
	}

	fmt.Println(sum)
}

func firstDigit(s string) int {
	for i := 0; i < len(s); i++ {
		if s[i] >= '0' && s[i] <= '9' {
			return int(s[i] - '0')
		}
	}
	log.Fatal("not found")
	return -1
}

func lastDigit(s string) int {
	for i := len(s) - 1; i >= 0; i-- {
		if s[i] >= '0' && s[i] <= '9' {
			return int(s[i] - '0')
		}
	}
	log.Fatal("not found")
	return -1
}

// SECOND PROBLEM

var numbers = []string{"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"}

func firstNumber(s string) int {
	temp := ""
	for i := 0; i < len(s); i++ {
		if s[i] >= '0' && s[i] <= '9' {
			return int(s[i] - '0')
		}
		temp += string(s[i])

		for idx, num := range numbers {
			if strings.HasSuffix(temp, num) {
				return idx + 1
			}
		}
	}
	log.Fatal("not found")
	return -1
}

func lastNumber(s string) int {
	temp := ""
	for i := len(s) - 1; i >= 0; i-- {
		if s[i] >= '0' && s[i] <= '9' {
			return int(s[i] - '0')
		}
		temp = string(s[i]) + temp

		for idx, num := range numbers {
			if strings.HasPrefix(temp, num) {
				return idx + 1
			}
		}
	}
	log.Fatal("not found")
	return -1
}
