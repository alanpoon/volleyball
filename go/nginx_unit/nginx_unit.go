package nginx_unit

import (
	"context"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"os"
	"strings"

	"github.com/alanpoon/bevy_cloud_server/util"

	"github.com/docker/docker/api/types"
	"github.com/docker/docker/client"
)

var DefaultConfig = `{
    "listeners": {
        "127.0.0.1:8080": {
            "pass": "routes/test_zipv"
        }
    },

    "routes": {
		"test_zipv":[{
            "match": {
                "uri": "*"
            },
            "action": {
                "share": "/Users/alanpoon/Documents/rust/volleyball/go/dir/test_zipv$uri"
            }
        }]
    }
}`
var unix_socket = os.Getenv("UNIX_SOCKET")
var DIR = os.Getenv("DIR")
var MOUNT_DIR = os.Getenv("MOUNT_DIR")

func init() {
	if DIR == "" {
		DIR = "/www"
	}
}

type Listener struct {
	Pass string `json:"pass"`
}
type UnitConfig struct {
	Listeners map[string]Listener
}

func AppendServer(port_number string, project string) (map[string]interface{}, error) {
	var res = make(map[string]interface{})
	if config, err := util.DockerSingletonInstance.Run(GetConfig, make(map[string]interface{})); err == nil {
		old_config_str, err := json.Marshal(config)
		if err != nil {
			return res, err
		}
		if l, ok := config["listeners"]; ok {
			if v, ok := l.(map[string]interface{}); ok {
				v[port_number] = map[string]interface{}{
					"pass": "routes/" + project,
				}
				if l, ok := config["routes"]; ok {
					if v, ok := l.(map[string]interface{}); ok {
						v[project] = []map[string]interface{}{
							map[string]interface{}{
								"match": map[string]interface{}{
									"uri": "*",
								},
								"action": map[string]interface{}{
									//"share": "/Users/alanpoon/Documents/rust/volleyball/go/dir/" + project + "$uri",
									"share": DIR + "/" + project + "$uri",
								},
							},
						}
					}
				}
			}

		}
		new_config_str, err := json.Marshal(config)
		if err != nil {
			return res, err
		}
		if string(old_config_str) != string(new_config_str) {
			return config, nil
		} else {
			return res, fmt.Errorf("config is identical, no update")
		}

	} else {
		return res, err
	}
}
func GetConfig(dummy map[string]interface{}) (map[string]interface{}, error) {

	return DockerNginxConfigure([]byte{}, "curl -s --unix-socket /var/run/control.unit.sock http://localhost/config")
}
func DockerNginxConfigure(config []byte, command string) (map[string]interface{}, error) {
	res := make(map[string]interface{})
	abs_path := os.Getenv("ABSOLUTE_PATH")
	var err error
	is_post := false
	if len(config) > 0 {
		is_post = true
	}
	if is_post {
		abs_path := os.Getenv("ABSOLUTE_PATH")

		err = ioutil.WriteFile(abs_path+"/../configs/nginx_configs_holder/snippet.json", config, 0644)
		if err != nil {
			return res, err
		}
	}

	cli, err := client.NewClientWithOpts(client.FromEnv)
	defer cli.Close()
	if err != nil {
		println("er", err.Error())
	}
	nginx_res, err := cli.ContainerExecCreate(context.Background(), "nginx_unit", types.ExecConfig{
		AttachStdin:  true,
		AttachStdout: true,
		AttachStderr: true,
		Env:          []string{},
		Cmd:          []string{"sh", "-c", command},
		//Cmd: []string{"sh", "-c", fmt.Sprintf("ls")},
	})
	h, err := cli.ContainerExecAttach(context.Background(), nginx_res.ID, types.ExecStartCheck{})
	if err != nil {
		return res, fmt.Errorf("ContainerExecStart %s", err.Error())
	}
	s := ""
	for {
		l, _, e := h.Reader.ReadLine()
		l_s := string(l)
		if e != nil {
			break
		}
		if s == "" {
			if strings.Contains(l_s, "{") {
				s += "{"
				s += "\n"
				continue
			}
		}
		s += string(l)
		s += "\n"
	}
	err = json.Unmarshal([]byte(s), &res)
	if is_post {
		os.Remove(abs_path + "/../configs/nginx_configs_holder/snippet.json")
	}
	return res, err
}
func UpdateConfig(config map[string]interface{}) (map[string]interface{}, error) {
	if json_str, er := json.Marshal(config); er != nil {
		return make(map[string]interface{}), nil
	} else {
		return DockerNginxConfigure(json_str, "curl -X PUT -s --data-binary @/nginx_configs_holder/snippet.json --unix-socket /var/run/control.unit.sock http://localhost/config")
	}
}

//	func AppendCert(path string) (map[string]interface{}, error) {
//		return DockerNginxConfigure([]byte{}, "curl -X PUT -s --data-binary @/certs/wasmmock_xyz/wasmmock_xyz.ca-bundle --unix-socket /var/run/control.unit.sock http://localhost/certificates/bundle")
//	}
func AppendCert(path string) (map[string]interface{}, error) {
	return DockerNginxConfigure([]byte{}, "curl -X PUT -s --data-binary @/certs/bundle.pem --unix-socket /var/run/control.unit.sock http://localhost/certificates/bundle")
}
