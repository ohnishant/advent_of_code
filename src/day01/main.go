package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func main() {
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
