package main

import (
	"github.com/alanpoon/bevy_cloud_server/server"
)

type P struct {
}

//func (v P) Start(s []string) {

func main() {
	server.NewServer(P{})
}
