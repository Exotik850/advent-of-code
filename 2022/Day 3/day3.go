package main

import (
	"bufio"
	"fmt"
	"math/bits"
	"os"
)

func check(err error) {
	if err != nil {
		panic(err)
	}
}

func Priority(c rune) int {
	if c >= 'A' && c <= 'Z' {
		return int(c - 38)
	} else if c >= 'a' && c <= 'z' {
		return int(c - 96)
	}
	return 0
}

func PriorityToChar(p int) rune {
	if p >= 1 && p <= 26 {
		return rune(p + 96)
	}
	if p > 26 && p <= 52 {
		return rune(p + 38)
	}
	return 0
}

func main() {
	file, err := os.Open("rucksacks.txt")
	defer file.Close()
	check(err)
	scanner := bufio.NewScanner(file)
	sum := 0
	masks := make([]uint64, 0)
	for scanner.Scan() {
		sack := scanner.Text()
		// part 1
		// left := uint64(0)
		// right := uint64(0)
		// compLength := len(sack) / 2
		// for _, c := range sack[:compLength] {
		// 	left |= 1 << Priority(c)
		// }
		// for _, c := range sack[compLength:] {
		// 	right |= 1 << Priority(c)
		// }
		// result := left & right
		// if result == 0 {
		// 	fmt.Println("No")
		// } else {
		// sum += bits.TrailingZeros64(result)
		// }

		// part 2
		mask := uint64(0)
		for _, c := range sack {
			mask |= 1 << Priority(c)
		}
		masks = append(masks, mask)
	}

	// Part 2
	for i := 0; i < len(masks); i += 3 {
		// Find the common bits between the 3 masks
		common := masks[i] & masks[i+1] & masks[i+2]
		count := bits.OnesCount64(common)
		if count != 1 {
			fmt.Println("Invalid mask count:", count)
			continue
		}
		// Count the number of bits in the common mask
		sum += bits.TrailingZeros64(common)
	}

	fmt.Println("Total Priority:", sum)

}
