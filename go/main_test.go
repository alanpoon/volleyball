package main

import (
	"bytes"
	"fmt"
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
