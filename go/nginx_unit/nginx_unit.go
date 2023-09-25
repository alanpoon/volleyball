package nginx_unit

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net"
	"net/http"
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
var DefaultListenerConfig = `{
	"127.0.0.1:8080": {
		"pass": "routes/test_zipv"
	}
}`

type Listener struct {
	Pass string `json:"pass"`
}
type UnitConfig struct {
	Listeners map[string]Listener
}

func UpdateConfig(config map[string]interface{}) (map[string]interface{}, error) {
	httpc := &http.Client{
		Transport: &http.Transport{
			DialContext: func(_ context.Context, _, _ string) (net.Conn, error) {
				return net.Dial("unix", "/opt/homebrew/var/run/unit/control.sock")
			},
		},
	}
	var resp *http.Response
	res := make(map[string]interface{})
	var err error
	json_bytes, err := json.Marshal(config)
	if err != nil {
		return res, fmt.Errorf("cannot update config %s", err.Error())
	}
	buf := bytes.NewBuffer(json_bytes)
	req, err := http.NewRequest(http.MethodPut, "http://localhost/config", buf)
	if err != nil {
		return res, fmt.Errorf("cannot update config %s", err.Error())
	}

	if resp, err = httpc.Do(req); err == nil {
		if resp.StatusCode == http.StatusOK {
			defer resp.Body.Close()
			bodyBytes, err := io.ReadAll(resp.Body)
			if err != nil {
				return res, err
			}
			if err := json.Unmarshal(bodyBytes, &res); err == nil {
				return res, nil
			}
		} else {
			return res, fmt.Errorf("bad req %v", resp)
		}
	}
	return res, err
}
func AppendListener(port_number string, project string) (map[string]interface{}, error) {
	httpc := http.Client{
		Transport: &http.Transport{
			DialContext: func(_ context.Context, _, _ string) (net.Conn, error) {
				return net.Dial("unix", "/opt/homebrew/var/run/unit/control.sock")
			},
		},
	}
	var resp *http.Response
	res := make(map[string]interface{})
	data := map[string]interface{}{
		port_number: map[string]interface{}{
			"pass": "routes/" + project,
		},
	}
	if data_, err := json.Marshal(data); err == nil {
		req, err := http.NewRequest(http.MethodPut, "http://localhost/config/listeners", bytes.NewBuffer(data_))
		resp, err = httpc.Do(req)
		if err != nil {
			return res, fmt.Errorf("cannot append listener")
			// handle error
		}
		if resp.StatusCode == http.StatusOK {
			bodyBytes, err := io.ReadAll(resp.Body)
			if err != nil {
				return res, err
			}
			fmt.Println("bodyBytes", string(bodyBytes))
			if err := json.Unmarshal(bodyBytes, &res); err == nil {
				return res, nil
			}
		}
	}
	return res, fmt.Errorf("cannot append listener")
}
func AppendRoute(project string) (map[string]interface{}, error) {
	httpc := http.Client{
		Transport: &http.Transport{
			DialContext: func(_ context.Context, _, _ string) (net.Conn, error) {
				return net.Dial("unix", "/opt/homebrew/var/run/unit/control.sock")
			},
		},
	}
	var resp *http.Response
	res := make(map[string]interface{})
	data := map[string]interface{}{
		project: map[string]interface{}{
			"match": map[string]interface{}{
				"uri": "*",
			},
			"action": map[string]interface{}{
				"share": "/Users/alanpoon/Documents/rust/volleyball/go/dir/" + project + "$uri",
			},
		},
	}
	if data_, err := json.Marshal(data); err == nil {
		req, err := http.NewRequest(http.MethodPut, "http://localhost/config/routes", bytes.NewBuffer(data_))
		resp, err = httpc.Do(req)
		if err != nil {
			// handle error
			return res, err
		}
		if resp.StatusCode == http.StatusOK {
			bodyBytes, err := io.ReadAll(resp.Body)
			if err != nil {
				return res, err
			}
			if err := json.Unmarshal(bodyBytes, &res); err == nil {
				return res, nil
			}
		} else {
			return res, fmt.Errorf("cannot append route %v", resp)
		}
	} else {
		return res, fmt.Errorf("cannot append route %s", err.Error())

	}
	return res, fmt.Errorf("cannot append route")
}
func AppendServer(port_number string, project string) (map[string]interface{}, error) {
	if config, err := GetConfig(); err == nil {
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
									"share": "/Users/alanpoon/Documents/rust/volleyball/go/dir/" + project + "$uri",
								},
							},
						}
					}
				}
			}

		}
		return config, nil
	} else {
		return make(map[string]interface{}), err
	}

}
func GetConfig() (map[string]interface{}, error) {
	fmt.Println("payload")
	httpc := http.Client{
		Transport: &http.Transport{
			DialContext: func(_ context.Context, _, _ string) (net.Conn, error) {
				return net.Dial("unix", "/opt/homebrew/var/run/unit/control.sock")
			},
		},
	}
	var resp *http.Response
	var err error
	res := make(map[string]interface{})

	if resp, err = httpc.Get("http://localhost/config/"); err == nil {
		defer resp.Body.Close()

		if resp.StatusCode == http.StatusOK {
			bodyBytes, err := io.ReadAll(resp.Body)
			if err != nil {
				return res, err
			}
			if err := json.Unmarshal(bodyBytes, &res); err == nil {
				return res, nil
			}
		}
	} else {
		return res, err
	}
	return res, fmt.Errorf("cannot find config")
}
