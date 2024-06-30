package consistentHash

import (
	"strconv"
	"testing"
)

type IP string

func (i IP) String() string {
	return string(i)
}

func TestNewConsistentHash(t *testing.T) {
	c := NewConsistentHash()
	c.AddNode(IP("1.1.1.1"))
	c.AddNode(IP("1.1.1.2"))
	c.AddNodeWithReplicas(IP("1.1.1.3"), 10)
	c.AddNode(IP("1.1.1.4"))
	c.AddNodeWithReplicas(IP("1.1.1.5"), 50)

	counter := make(map[string]int)
	for i := 0; i < 10000; i++ {
		if n, ok := c.Get(IP(strconv.Itoa(i))); ok {
			counter[n.String()]++
		} else {
			t.Errorf("not found node err: i=%d", i)
		}
	}

	for ip, cnt := range counter {
		t.Logf("ip=%s, cnt=%d", ip, cnt)
	}
}
