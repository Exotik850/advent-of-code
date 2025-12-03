package main

import (
	"bufio"
	"fmt"
	"os"
)

func buildLayout(scanner *bufio.Scanner) [][]byte {
	inputLayout := make([]string, 0)
	for scanner.Scan() {
		line := scanner.Text()
		if len(line) == 0 {
			break
		}
		inputLayout = append(inputLayout, line)
	}
	height := len(inputLayout) - 1
	// reverse the inputLayout
	for i := 0; i < len(inputLayout)/2; i++ {
		inputLayout[i], inputLayout[height-i] = inputLayout[height-i], inputLayout[i]
	}
	width := len(inputLayout[0])
	numStacks := (width + 1) / 4
	layout := make([][]byte, numStacks)
	for i := 1; i < len(inputLayout); i++ {
		for marker := 0; marker < numStacks; marker++ {
			if inputLayout[i][marker*4+1] != ' ' {
				layout[marker] = append(layout[marker], inputLayout[i][marker*4+1])
			}
		}
	}
	return layout
}

func parseCommand(input string) (int, int, int) {
	// move AMT from STACK to STACK
	var from, to, amt int
	fmt.Sscanf(input, "move %d from %d to %d", &amt, &from, &to)
	return from - 1, to - 1, amt
}

func main() {
	file, err := os.Open("procedure.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	layout := buildLayout(scanner)
	fmt.Println(layout)

	for scanner.Scan() {
		line := scanner.Text()
		from, to, amt := parseCommand(line)

		// Part 1 Move each independently
		// for i := 0; i < amt; i++ {
		// 	layout[to] = append(layout[to], layout[from][len(layout[from])-1])
		// 	layout[from] = layout[from][:len(layout[from])-1]
		// }

		// Part 2 Move all at once
		layout[to] = append(layout[to], layout[from][len(layout[from])-amt:]...)
		layout[from] = layout[from][:len(layout[from])-amt]
	}

	for _, stack := range layout {
		fmt.Printf("%c", stack[len(stack)-1])
	}

}
