package consistent_hash

import (
	"fmt"
	"github.com/spaolacci/murmur3"
	"sort"
	"strconv"
	"sync"
)

func hash(data []byte) uint64 {
	return murmur3.Sum64(data)
}

type Node interface {
	String() string
}

const (
	// 最小放大因子
	minReplicas = 100
	prime       = 16777619
)

type Func func(data []byte) uint64

type ConsistentHash struct {
	hashFunc Func

	replicas int
	// 存放虚拟节点
	keys []uint64
	// 虚拟节点-物理节点 映射关系
	ring map[uint64][]Node
	// 用于快速判断物理节点是否存在
	nodes map[Node]bool
	lock  sync.RWMutex
}

func NewConsistentHash() *ConsistentHash {
	return &ConsistentHash{
		hashFunc: hash,
		replicas: minReplicas,
		ring:     map[uint64][]Node{},
		nodes:    map[Node]bool{},
	}
}

func (h *ConsistentHash) AddNode(node Node) {
	h.AddNodeWithReplicas(node, h.replicas)
}

func (h *ConsistentHash) AddNodeWithReplicas(node Node, replicas int) {
	// 移除 node, 以此允许重复添加
	h.RemoveNode(node)

	if replicas > h.replicas {
		replicas = h.replicas
	}

	h.lock.Lock()
	defer h.lock.Unlock()

	for i := 0; i < replicas; i++ {
		hashValue := h.hashFunc([]byte(node.String() + strconv.Itoa(i)))
		// 添加虚拟节点
		h.keys = append(h.keys, hashValue)
		// 添加 虚拟节点-物理节点 映射关系
		h.ring[hashValue] = append(h.ring[hashValue], node)
	}
	// 添加真实的物理节点
	h.addNode(node)

	// 对虚拟节点排序, 方便以后查找
	sort.Slice(h.keys, func(i, j int) bool {
		return h.keys[i] < h.keys[j]
	})
}

func (h *ConsistentHash) Get(node Node) (Node, bool) {
	h.lock.RLock()
	defer h.lock.RUnlock()

	// 安全检查
	if len(h.nodes) == 0 {
		return nil, false
	}

	hashValue := h.hashFunc([]byte(node.String()))
	// 寻找顺时针第一个节点
	index := sort.Search(len(h.keys), func(i int) bool {
		return h.keys[i] >= hashValue
	}) % len(h.keys)

	nodes := h.ring[h.keys[index]]
	switch len(nodes) {
	// 如果不存在
	case 0:
		return nil, false
	// 存在 && 没有哈希冲突
	case 1:
		return nodes[0], true
	// 存在 && 哈希冲突
	default:
		innerHashValue := h.hashFunc([]byte(fmt.Sprintf("%d:%s", prime, node)))
		innerIndex := int(innerHashValue % uint64(len(nodes)))
		return nodes[innerIndex], true
	}
}

func (h *ConsistentHash) RemoveNode(node Node) {
	h.lock.Lock()
	defer h.lock.Unlock()

	if !h.containNode(node) {
		return
	}

	for i := 0; i < h.replicas; i++ {
		hashValue := h.hashFunc([]byte(node.String() + strconv.Itoa(i)))
		index := sort.Search(len(h.keys), func(i int) bool {
			return h.keys[i] >= hashValue
		})
		// 删除虚拟节点
		if index < len(h.keys) && h.keys[index] == hashValue {
			h.keys = append(h.keys[:index], h.keys[index+1:]...)
		}
		h.removeRingNode(hashValue, node)
	}
	h.removeNode(node)
}

func (h *ConsistentHash) addNode(node Node) {
	h.nodes[node] = true
}

func (h *ConsistentHash) removeNode(node Node) {
	delete(h.nodes, node)
}

func (h *ConsistentHash) removeRingNode(hashValue uint64, node Node) {
	if nodes, ok := h.ring[hashValue]; ok {
		var newNodes []Node
		for _, x := range nodes {
			if x.String() != node.String() {
				newNodes = append(newNodes, x)
			}
		}

		if len(newNodes) > 0 {
			h.ring[hashValue] = newNodes
		} else {
			delete(h.ring, hashValue)
		}
	}
}

func (h *ConsistentHash) containNode(node Node) bool {
	return h.nodes[node]
}
