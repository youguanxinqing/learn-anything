package heap

import (
	"errors"
)

type Number interface {
	int | int8 | int16 | int32 | int64 |
		float32 | float64 |
		uint | uint8 | uint16 | uint32 | uint64
}

type Heap[T Number] struct {
	box []T
	max int
}

func New[T Number](max int) *Heap[T] {
	// 占用头节点
	var place T
	return &Heap[T]{box: []T{place}, max: max}
}

func (h *Heap[T]) Top() (T, error) {
	var null T
	if len(h.box) < 2 {
		return null, errors.New("heap is empty")
	}

	top := h.box[1]
	// 堆化
	h.box[1] = h.box[len(h.box)-1]
	h.box = h.box[:len(h.box)-1]
	cur, maxPos := 1, 1
	for {
		// 左子节点
		if 2*cur < len(h.box) && h.box[maxPos] < h.box[2*cur] {
			maxPos = 2 * cur
		}
		// 右子节点
		if 2*cur+1 < len(h.box) && h.box[maxPos] < h.box[2*cur+1] {
			maxPos = 2*cur + 1
		}
		// no swap, return directly
		if cur == maxPos {
			break
		}

		// swap
		h.box[cur], h.box[maxPos] = h.box[maxPos], h.box[cur]
		cur = maxPos
	}
	return top, nil
}

func (h *Heap[T]) Add(item T) error {
	if len(h.box)-1 == h.max {
		return errors.New("heap if full")
	}

	h.box = append(h.box, item)
	cur := len(h.box) - 1
	for (cur/2 != 0) && (h.box[cur] > h.box[cur/2]) {
		h.box[cur], h.box[cur/2] = h.box[cur/2], h.box[cur]
		cur = cur / 2
	}
	return nil
}

// NewFromArray 传入一个数组, 原地堆化
func NewFromArray[T Number](array []T) []T {
	// 从有叶子节点的数据开始堆化
	for i := len(array)/2 - 1; i >= 0; i-- {

		cur, maxPos := i, i
		for {
			// 左子节点
			if 2*cur+1 <= len(array)-1 && array[maxPos] < array[2*cur+1] {
				maxPos = 2*cur + 1
			}
			// 右子节点
			if 2*cur+2 <= len(array)-1 && array[maxPos] < array[2*cur+2] {
				maxPos = 2*cur + 2
			}
			if cur == maxPos {
				break
			}

			array[cur], array[maxPos] = array[maxPos], array[cur]
			cur = maxPos
		}
	}
	return array
}

func SortHeap[T Number](array []T) {
	if len(array) == 1 {
		return
	}

	for n := len(array); n > 2; n-- {
		array[0], array[n-1] = array[n-1], array[0]
		heapify(array, n-1)
	}
}

func heapify[T Number](array []T, n int) {
	cur, maxPos := 0, 0
	for {
		// 左子节点
		if cur*2+1 < n && array[maxPos] < array[cur*2+1] {
			maxPos = cur*2 + 1
		}
		// 右子节点
		if cur*2+2 < n && array[maxPos] < array[cur*2+2] {
			maxPos = cur*2 + 2
		}
		if cur == maxPos {
			break
		}

		array[cur], array[maxPos] = array[maxPos], array[cur]
		cur = maxPos
	}
}
