package util

import (
	"sync"
)

var DockerSingletonInstance = DockerSingleton{}

type DockerSingleton struct {
	mu sync.Mutex
}
type FN func(map[string]interface{}) (map[string]interface{}, error)

func (d *DockerSingleton) Run(closure FN, parameter map[string]interface{}) (map[string]interface{}, error) {
	d.mu.Lock()
	defer d.mu.Unlock()
	return closure(parameter)
}
