package main

import (
	"fmt"
	"os"
)

func main() {
	bytes, err := os.ReadFile("transmission.txt")
	if err != nil {
		panic(err)
	}

	const length = 14

	// Sliding window over the bytes
	for i := length - 1; i < len(bytes); i++ {
		window := bytes[i-length+1 : i+1]

		// Check if all characters in the window are unique
		if isUnique(window) {
			fmt.Println(string(window), i+1)
			break
		}
	}
}

func isUnique(window []byte) bool {
	seen := make(map[byte]bool)

	for _, b := range window {
		if seen[b] {
			return false
		}
		seen[b] = true
	}

	return true
}
