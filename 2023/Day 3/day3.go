package main

import (
	"bufio"
	"os"
	"strconv"
	"strings"
	"unicode"
)

type MaybePart struct {
	num   int
	width int
	x     int
	y     int
}

type Symbol struct {
	icon rune
	x    int
	y    int
}

func getParts(line string, y int) []MaybePart {
	makingDigit := false
	cursor := 0
	parts := []MaybePart{}
	for i, c := range line {
		if c >= '0' && c <= '9' {
			if !makingDigit {
				makingDigit = true
				cursor = i
			}
			continue
		} else if makingDigit {
			numStr := line[cursor:i]
			width := i - cursor
			num, err := strconv.Atoi(numStr)
			if err != nil {
				panic(err)
			}

			makingDigit = false
			parts = append(parts, MaybePart{num, width, cursor, y})
			cursor = i
		}
	}
	return parts
}

func getSymbols(line string, y int) []Symbol {
	symbols := []Symbol{}
	for i, c := range line {
		if c != '.' && !unicode.IsDigit(c) {
			symbols = append(symbols, Symbol{c, i, y})
		}
	}
	return symbols
}

func main() {
	file, err := os.Open("./schematic.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	parts := []MaybePart{}
	symbols := []Symbol{}
	y := 0
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		parts = append(parts, getParts(line, y)...)
		symbols = append(symbols, getSymbols(line, y)...)
		y++
	}
	if err := scanner.Err(); err != nil {
		panic(err)
	}

	sum := 0
	for _, parts := range parts {
		// Check if there is a symbol that touches this part, even diagonally
		for _, symbol := range symbols {
			if symbol.x >= parts.x-1 && symbol.x <= parts.x+parts.width && symbol.y >= parts.y-1 && symbol.y <= parts.y+1 {
				println("Part", parts.num, "touches", string(symbol.icon))
				// Print pos
				println("Part", parts.num, "is at", parts.x, parts.y)
				sum += parts.num
			}
		}
	}

	// Write a blocked out map of the parts to schema_blocked.txt
	file, err = os.Create("./schema_blocked.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	cursor := 0
	for i, part := range parts {

		// If the part is behind the cursor, it is the next line
		if cursor > part.x {
			_, err := file.WriteString("\n")
			if err != nil {
				panic(err)
			}
			cursor = 0
		}

		// If the part is ahead of the cursor, print out the empty space
		if cursor < part.x {
			_, err := file.WriteString(strings.Repeat(" ", part.x-cursor))
			if err != nil {
				panic(err)
			}
		}
		// Print the part
		_, err := file.WriteString(strings.Repeat("#", part.width))
		if err != nil {
			panic(err)
		}
		cursor = part.x + part.width
		// If this is the last part, print out the empty space
		if i == len(parts)-1 {
			_, err := file.WriteString(strings.Repeat(" ", 140-cursor))
			if err != nil {
				panic(err)
			}
		}
	}

	println("Sum of parts that touch symbols is", sum)
}
