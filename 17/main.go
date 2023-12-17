package main

import (
	"bufio"
	"container/heap"
	"fmt"
	"log"
	"os"
	"strconv"
)

type Item struct {
    value    int
    priority int
    index    int
}

type PriorityQueue []*Item

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool {
    return pq[i].priority < pq[j].priority
}

func (pq PriorityQueue) Swap(i, j int) {
    pq[i], pq[j] = pq[j], pq[i]
    pq[i].index = i
    pq[j].index = j
}

func (pq *PriorityQueue) Push(x interface{}) {
    n := len(*pq)
    item := x.(*Item)
    item.index = n
    *pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() interface{} {
    old := *pq
    n := len(old)
    item := old[n-1]
    old[n-1] = nil
    item.index = -1
    *pq = old[0 : n-1]
    return item
}

var (
    R, C int
    m    [][]int
    D    []int
    K    = [4][2]int{{0, 1}, {-1, 0}, {0, -1}, {1, 0}}
    Z    = [4][2]int{{1, 3}, {0, 2}, {1, 3}, {0, 2}}
)

func dijkstra(lb, ub int) int {
    D = make([]int, 4*R*C)
    for i := range D {
        D[i] = 1e6
    }
    pq := make(PriorityQueue, 4)
    for s := 0; s < 4; s++ {
        D[s] = 0
        pq[s] = &Item{value: s, priority: 0, index: s}
    }
    heap.Init(&pq)
    for len(pq) > 0 {
        item := heap.Pop(&pq).(*Item)
        dd, tt := item.priority, item.value
        r, c, z := tt/(4*C), tt/4%C, tt%4
        if dd != D[tt] {
            continue
        }
        for _, i := range Z[z] {
            rr, cc := r, c
            dr, dc := K[i][0], K[i][1]
            inc := 0
            for k := 1; k <= ub; k++ {
                rr += dr
                cc += dc
                if 0 <= rr && rr < R && 0 <= cc && cc < C {
                    inc += m[rr][cc]
                    if lb <= k {
                        t := 4*C*rr + 4*cc + i
                        nn := dd + inc
                        if D[t] > nn {
                            D[t] = nn
                            heap.Push(&pq, &Item{value: t, priority: nn})
                        }
                    }
                } else {
                    break
                }
            }
        }
    }
    min := D[4*R*C-1]
    for i := 2; i <= 4; i++ {
        if D[4*R*C-i] < min {
            min = D[4*R*C-i]
        }
    }
    return min
}

func main() {
    file, err := os.Open("input.txt") // replace with your file name
    if err != nil {
        log.Fatal(err)
    }
    defer file.Close()

    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        line := scanner.Text()
        row := make([]int, len(line))
        for i, ch := range line {
            row[i], _ = strconv.Atoi(string(ch))
        }
        m = append(m, row)
    }
    if err := scanner.Err(); err != nil {
        log.Fatal(err)
    }

    R, C = len(m), len(m[0])
    fmt.Println("Part 1:", dijkstra(1, 3))
    fmt.Println("Part 2:", dijkstra(4, 10))
}