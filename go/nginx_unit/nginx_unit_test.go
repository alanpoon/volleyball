package nginx_unit

import (
	"encoding/json"
	"fmt"
	"testing"
)

func TestGetConfigs(t *testing.T) {
	if res, err := GetConfig(make(map[string]interface{})); err == nil {
		fmt.Println("res", res)
	} else {
		t.Errorf("err is %s", err.Error())
	}
}
func TestUpdateConfig(t *testing.T) {
	var config = make(map[string]interface{})
	var err error
	if err = json.Unmarshal([]byte(DefaultConfig), &config); err == nil {
		if res, err1 := UpdateConfig(config); err1 == nil {
			if success_val, ok := res["success"]; ok {
				if success_val == "Reconfiguration done." {
					return
				} else {
					t.Errorf("success is %v", ok)
				}
			}

		} else {
			err = err1
		}
	}
	t.Errorf("err is %s", err.Error())

}
func TestAppendServer(t *testing.T) {
	var er error

	if new_config, err := AppendServer("127.0.0.1:8081", "test_zip2"); err == nil {
		if res, err := UpdateConfig(new_config); err == nil {
			fmt.Println("res", res)
			if config, err := GetConfig(make(map[string]interface{})); err == nil {
				if l, ok := config["listeners"]; ok {
					if v, ok := l.(map[string]interface{}); ok {
						if _, ok := v["127.0.0.1:8081"]; ok {
							return
						} else {
							t.Errorf("Cannot get 127.0.0.1:8081")
						}
					} else {

					}
				}
			} else {
				er = err
			}
		} else {
			er = err
		}
	}

	if er != nil {
		t.Errorf("error %s", er.Error())
	}
}
func TestAppendCert(t *testing.T) {
	if res, err := AppendCert(""); err == nil {
		fmt.Println("s", res)
	} else {
		t.Error(err.Error())
	}
}
