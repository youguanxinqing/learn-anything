package heap

import (
	"testing"
)

func TestHeap(t *testing.T) {
	heapWithInt := New[int](10)
	for _, elem := range []int{0, 8, 1, 2, 8, 3, 9, 6, -1, -10} {
		if err := heapWithInt.Add(elem); err != nil {
			t.Error(err)
		}
	}

	for {
		if elem, err := heapWithInt.Top(); err != nil {
			t.Error(err)
			break
		} else {
			t.Log(elem)
		}
	}
}

func TestNewFromArray(t *testing.T) {
	heap := NewFromArray([]int{0, 8, 1, 2, 8, 3, 9, 6, -1, -10})
	t.Log(heap)
	SortHeap(heap)
	t.Log(heap)
}
