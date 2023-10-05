package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"

	"github.com/alanpoon/bevy_cloud_server/processor"
)

// tar -czf test_zip.tgz test_zip
func TestUploadStaticHandler(t *testing.T) {
	if source_file, er := os.ReadFile("test_zip.tgz"); er == nil {
		reader := bytes.NewReader(source_file)
		req := httptest.NewRequest(http.MethodPost, "/upload_static?project=test_zipv", reader)
		w := httptest.NewRecorder()
		processor.UploadStatic()(w, req)
		res := w.Result()
		defer res.Body.Close()
		if _, err := os.Stat("./dir/test_zip"); os.IsNotExist(err) {
			// path/to/whatever does not exist
			//t.Errorf("./dir/test_zip%v", err.Error())

		}
	} else {
		fmt.Println("er", er)
	}

}
func TestNginxUpdateHandler(t *testing.T) {
	reader := bytes.NewReader([]byte{})
	req := httptest.NewRequest(http.MethodPost, "/nginx_update?project=volleyball&port=8001", reader)
	w := httptest.NewRecorder()
	processor.NginxUpdate()(w, req)
	res := w.Result()
	defer res.Body.Close()
	var er error
	if code, err := ioutil.ReadAll(res.Body); err == nil {
		var obj = make(map[string]interface{})
		err := json.Unmarshal(code, &obj)
		if err != nil {
			er = err
		}
		if rb, ok := obj["response_body"]; ok {
			if rbb, ok := rb.(map[string]interface{}); ok {
				if _, ok := rbb["success"]; ok {
					return
				} else {
					er = fmt.Errorf("No success")
				}
			} else {
				er = fmt.Errorf("not a map")
			}
		} else {
			er = fmt.Errorf("not a response_body")
		}
		fmt.Println("obj", obj)
	} else {
		t.Fail()
	}
	if er != nil {
		t.Error(er)
	}
}
