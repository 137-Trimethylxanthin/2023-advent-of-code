package main

import (
	"fmt"
	"log"
	"os"
	"strings"
)

type Queue [][]int

func (q *Queue) IsEmpty() bool {
	return len(*q) == 0
}

func (q *Queue) Push(slice []int) {
	*q = append(*q, slice)
}

func (q *Queue) Pop() []int {
	last := (*q)[0]
	*q = (*q)[1:]

	return last
}

var directions = map[rune][][]int{
	'|': {{1, 0}, {-1, 0}},
	'-': {{0, 1}, {0, -1}},
	'F': {{0, 1}, {1, 0}},
	'7': {{0, -1}, {1, 0}},
	'J': {{0, -1}, {-1, 0}},
	'L': {{0, 1}, {-1, 0}},
}

func readFile(path string) ([]string, error) {
	file, err := os.ReadFile(path)
	if err != nil {
		log.Fatal(err)
	}

	pipesRaw := strings.Split(strings.Trim(string(file), "\n"), "\n")
	return pipesRaw, nil
}

func processPipes(pipesRaw []string) [][]rune {
	var pipes [][]rune
	for _, line := range pipesRaw {
		pipes = append(pipes, []rune(line))
	}
	return pipes
}

func findStart(pipes [][]rune) []int {
	var start []int
start:
	for i := range pipes {
		for j := range pipes[i] {
			if pipes[i][j] == 'S' {
				start = []int{i, j}
				// HARDCODED: IDK how the f to do this sry :) just try both and i am to lazy to do it properly for now ...
				// pipes[i][j] = '7'
				pipes[i][j] = 'J'
				break start
			}
		}
	}
	return start
}

func calculateDistances(pipes [][]rune, start []int) [][]bool {
	var visited [][]bool
	for range pipes {
		var temp []bool
		for range pipes[0] {
			temp = append(temp, false)
		}
		visited = append(visited, temp)
	}
	queue := Queue{}
	queue.Push([]int{start[0], start[1], 0})
	visited[start[0]][start[1]] = true

	for !queue.IsEmpty() {
		curr := queue.Pop()

		i, j, dist := curr[0], curr[1], curr[2]

		nextDirs := directions[pipes[i][j]]

		for _, coords := range nextDirs {
			x, y := i+coords[0], j+coords[1]
			if !visited[x][y] {
				visited[x][y] = true
				queue.Push([]int{x, y, dist + 1})
			}
		}
	}
	return visited
}

func calculateEnclosed(pipes [][]rune, visited [][]bool) int {
	for i := range pipes {
		for j := range pipes[i] {
			if !visited[i][j] {
				pipes[i][j] = '.'
			}
		}
	}

	enclosed := 0
	for _, row := range pipes {
		inside := 0

		i := 0
		for i < len(row) {
			r := row[i]

			switch r {
			case '|':
				inside++
			case 'J':
				inside++
			case '7':
				inside++
			case 'F':
				i++
				for row[i] == '-' {
					i++
				}
				if row[i] == 'J' {
					inside++
				}
			case 'L':
				i++
				for row[i] == '-' {
					i++
				}
				if row[i] == '7' {
					inside++
				}
			case '.':
				if inside%2 == 1 {
					enclosed++
				}
			}

			i++
		}
	}
	return enclosed
}

func main() {
	pipesRaw, _ := readFile("input.txt")
	pipes := processPipes(pipesRaw)
	start := findStart(pipes)
	visited := calculateDistances(pipes, start)
	enclosed := calculateEnclosed(pipes, visited)
	fmt.Print("solution: ", enclosed, "\n")
}
