package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
	"time"
)

func main() {
	file, err := os.Open("elf_food.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	start := time.Now()
	scanner := bufio.NewScanner(file)
	max := -1
	current := 0
	elves := make([]int, 0)
	for scanner.Scan() {
		input := scanner.Text()
		if len(input) == 0 {
			elves = append(elves, current)
			current = 0
			continue
		}
		num, err := strconv.Atoi(strings.TrimSpace(input))
		if err != nil {
			panic(err)
		}
		current += num
		if current > max {
			max = current
		}

	}
	if err := scanner.Err(); err != nil {
		panic(err)
	}
	fmt.Println("Found max:", max)
	sort.Sort(sort.Reverse(sort.IntSlice(elves)))
	fmt.Println("Top three elves:", elves[0], elves[1], elves[2])
	fmt.Println("Sum of top three elves:", elves[0]+elves[1]+elves[2])
	fmt.Println("Execution time:", time.Since(start))
}
