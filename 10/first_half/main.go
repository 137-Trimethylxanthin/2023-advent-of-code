package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

var pipes = map[rune][][]int{
	'|': {{0, -1}, {0, 1}},
	'-': {{-1, 0}, {1, 0}},
	'L': {{0, -1}, {1, 0}},
	'J': {{0, -1}, {-1, 0}},
	'7': {{0, 1}, {-1, 0}},
	'F': {{0, 1}, {1, 0}},
}

var adjs = [][]int{{0, -1}, {0, 1}, {-1, 0}, {1, 0}}

func pointingTo(pipeType rune, pipe, target []int) bool {
	for _, direction := range pipes[pipeType] {
		if pipe[0]+direction[0] == target[0] && pipe[1]+direction[1] == target[1] {
			return true
		}
	}
	return false
}

func findLoop(matrix []string, start []int) [][]int {
	path := [][]int{start}

	for _, adj := range adjs {
		x, y := start[0]+adj[0], start[1]+adj[1]
		if strings.ContainsRune("|-LJ7F", rune(matrix[y][x])) && pointingTo(rune(matrix[y][x]), []int{x, y}, start) {
			path = append(path, []int{x, y})
			break
		}
	}

	cx, cy := path[len(path)-1][0], path[len(path)-1][1]
	for cx != start[0] || cy != start[1] {
		for _, direction := range pipes[rune(matrix[cy][cx])] {
			if cx+direction[0] != path[len(path)-2][0] || cy+direction[1] != path[len(path)-2][1] {
				path = append(path, []int{cx + direction[0], cy + direction[1]})
				break
			}
		}
		cx, cy = path[len(path)-1][0], path[len(path)-1][1]
	}

	return path[:len(path)-1]
}

func readFile(filePath string) ([]string, error) {

	var lines []string

	file, err := os.Open(filePath)
	if err != nil {
		return lines, err
	}
	defer func(file *os.File) {
		err := file.Close()
		if err != nil {

		}
	}(file)

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	return lines, nil
}

func main() {
	lines, _ := readFile("input.txt")
	matrix := make([]string, len(lines))
	start := []int{-1, -1}
	for y, line := range lines {
		if strings.Contains(line, "S") {
			start = []int{strings.Index(line, "S"), y}
		}
		matrix[y] = line
	}

	path := findLoop(matrix, start)
	solution := len(path) / 2

	fmt.Println("Solution:", solution)
}
