package pkg

import (
	"hash/fnv"
	"strings"
)

func CompatibilityHash(name, query string) uint32 {
	h := fnv.New32a()
	if strings.Compare(name, query) > 0 {
		name, query = query, name
	}
	h.Write([]byte(name + query))
	return h.Sum32()
}
