package main

import (
	"cmp"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"

	"github.com/Knetic/govaluate"
)

func check(err error) {
	if err != nil {
		panic(err)
	}
}

type monkey struct {
	items        []uint64
	operation    func(uint64) uint64
	test         uint64
	true_monkey  *monkey
	false_monkey *monkey
	observations uint64
}

func (m *monkey) Eval() {
	m.observations += uint64(len(m.items))
	for len(m.items) != 0 {
		oldItem := m.items[0]
		m.items = m.items[1:]
		newItem := m.operation(oldItem)
		if m.Test(newItem) {
			m.true_monkey.AddItem(newItem)
		} else {
			m.false_monkey.AddItem(newItem)
		}
	}
}

const MAX_MOD = 43

func (m *monkey) AddItem(item uint64) {
	m.items = append(m.items, item%m.test)
}

func (m *monkey) Test(item uint64) bool {
	return m.test%item == 0
}

func ParseMonkey(inp string) ([]monkey, error) {
	lines := strings.Split(inp, "\n")

	m := make([]monkey, 8)
	index := 0

	for _, line := range lines {
		parts := strings.Split(strings.TrimSpace(line), ": ")
		switch parts[0] {
		case "Monkey":
			newIndex, err := strconv.Atoi(parts[1])
			check(err)
			index = newIndex
		case "Starting items":
			for _, s := range strings.Split(parts[1], ", ") {
				i, err := strconv.Atoi(s)
				check(err)
				m[index].AddItem(uint64(i))
			}
		case "Operation":
			opExpr := strings.Split(parts[1], "=")
			right := strings.TrimSpace(opExpr[1])
			expr, err := govaluate.NewEvaluableExpression(right)
			check(err)
			op := func(old uint64) uint64 {
				parameters := make(map[string]interface{}, 1)
				parameters["old"] = float64(old)
				res, err := expr.Evaluate(parameters)
				check(err)
				return uint64(res.(float64))
			}
			m[index].operation = op

		case "Test":
			div, err := strconv.Atoi(parts[1][len("divisible by "):])
			check(err)
			m[index].test = uint64(div)
		case "If true":
			monkeyIndex, err := strconv.Atoi(parts[1][len(parts[1])-1:])
			check(err)
			m[index].true_monkey = &m[monkeyIndex]
		case "If false":
			monkeyIndex, err := strconv.Atoi(parts[1][len(parts[1])-1:])
			check(err)
			m[index].false_monkey = &m[monkeyIndex]
		}
	}

	return m, nil
}

func main() {
	bytes, err := os.ReadFile("./input.txt")
	check(err)
	monkeyInput := string(bytes)
	monkeys, err := ParseMonkey(monkeyInput)
	check(err)

	for i := 0; i < 10000; i++ {
		// fmt.Printf("Round: %v\n", i)
		for i := range monkeys {
			monkeys[i].Eval()
		}
	}

	slices.SortFunc(monkeys, func(a monkey, b monkey) int {
		return cmp.Compare[uint64](b.observations, a.observations)
	})

	for _, monkey := range monkeys {
		fmt.Printf("monkey: %v\n", monkey)
	}

	result := monkeys[0].observations * monkeys[1].observations
	fmt.Printf("\nresult: %v\n", result)
}
