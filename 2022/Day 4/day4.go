package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func check(err error) {
	if err != nil {
		panic(err)
	}
}

func parseLine(line string) (int, int, int, int) {
	left, right, _ := strings.Cut(line, ",")
	a, b, _ := strings.Cut(left, "-")
	c, d, _ := strings.Cut(right, "-")
	ia, _ := strconv.Atoi(a)
	ib, _ := strconv.Atoi(b)
	ic, _ := strconv.Atoi(c)
	id, _ := strconv.Atoi(d)
	return ia, ib, ic, id
}

func main() {
	file, err := os.Open("pairs.txt")
	check(err)
	defer file.Close()
	scanner := bufio.NewScanner(file)
	total := 0
	for scanner.Scan() {
		line := scanner.Text()
		fmt.Println(line)
		a, b, c, d := parseLine(line)
		fmt.Println(a, b, c, d)
		// Part 1
		// if (a <= c && b >= d) || (c <= a && d >= b) {
		// 	total++
		// }

		// Part 2
		if (a >= c && a <= d) || (b >= c && b <= d) || (c >= a && c <= b) || (d >= a && d <= b) {
			total++
		}
	}
	fmt.Println(total)
}
