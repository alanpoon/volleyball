package processor

import (
	"archive/tar"
	"compress/gzip"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"

	"github.com/alanpoon/bevy_cloud_server/model"
	"github.com/gorilla/websocket"
)

func init() {

}

type Able interface {
}

var upgrader = websocket.Upgrader{}

type RAmessage struct {
	RA_ID string `json:"ra_id"`
}

func ExtractTarGz(gzipStream io.Reader, project string) {
	uncompressedStream, err := gzip.NewReader(gzipStream)
	if err != nil {
		log.Fatal("ExtractTarGz: NewReader failed")
	}

	tarReader := tar.NewReader(uncompressedStream)
	if _, err := os.Stat("dir/" + project); os.IsNotExist(err) {
	} else {
		os.RemoveAll("dir/" + project)
	}
	var dir_name = ""
	for true {
		header, err := tarReader.Next()
		fmt.Println("header", header)
		if err == io.EOF {
			break
		}

		if err != nil {
			log.Fatalf("ExtractTarGz: Next() failed: %s", err.Error())
		}

		switch header.Typeflag {
		case tar.TypeDir:
			if dir_name == "" {
				dir_name = header.Name
				if _, err := os.Stat("dir/" + dir_name); os.IsNotExist(err) {
				} else {
					os.RemoveAll("dir/" + dir_name)
				}
			}
			if err := os.Mkdir("dir/"+header.Name, 0755); err != nil {
				log.Fatalf("ExtractTarGz: Mkdir() failed: %s", err.Error())
			}
		case tar.TypeReg:
			outFile, err := os.Create("dir/" + header.Name)
			if err != nil {
				log.Fatalf("ExtractTarGz: Create() failed: %s", err.Error())
			}
			if _, err := io.Copy(outFile, tarReader); err != nil {
				log.Fatalf("ExtractTarGz: Copy() failed: %s", err.Error())
			}
			outFile.Close()

		default:
			log.Fatalf(
				"ExtractTarGz: uknown type: %s in %s",
				header.Typeflag,
				project)
		}

	}
	if err := os.Rename("dir/"+dir_name, "dir/"+project); err == nil {

	}
}
func UploadStatic() http.HandlerFunc {
	return func(rw http.ResponseWriter, req *http.Request) {
		//code, err := ioutil.ReadAll(req.Body)
		res := model.CallApiResponse{
			Header: model.Header{},
		}
		//fmt.Println("code", len(code))

		dstArr := req.URL.Query()["project"]
		var dst string = ""
		if len(dstArr) > 0 {
			dst = dstArr[0]
		}
		ExtractTarGz(req.Body, dst)

		//_ = code
		//_ = err
		_ = res

	}

}
func NginxUpdate() http.HandlerFunc {
	return func(rw http.ResponseWriter, req *http.Request) {
		//code, err := ioutil.ReadAll(req.Body)
		res := model.CallApiResponse{
			Header: model.Header{},
		}
		_ = res
	}
}
