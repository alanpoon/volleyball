package processor

import (
	"archive/tar"
	"compress/gzip"
	"io"
	"log"
	"net/http"
	"os"

	"github.com/alanpoon/bevy_cloud_server/nginx_unit"
	"github.com/alanpoon/bevy_cloud_server/util"

	"github.com/alanpoon/bevy_cloud_server/model"
	"github.com/gorilla/websocket"
)

func init() {

}

type Able interface {
}

var upgrader = websocket.Upgrader{}

func ExtractTarGz(gzipStream io.Reader, project string) error {
	uncompressedStream, err := gzip.NewReader(gzipStream)
	if err != nil {
		log.Fatal("ExtractTarGz: NewReader failed")
		return err
	}

	tarReader := tar.NewReader(uncompressedStream)
	if _, err := os.Stat(nginx_unit.DIR + "/" + project); os.IsNotExist(err) {
	} else {
		os.RemoveAll(nginx_unit.DIR + "/" + project)
	}
	var dir_name = ""
	for true {
		header, err := tarReader.Next()
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
				if _, err := os.Stat(nginx_unit.DIR + "/" + dir_name); os.IsNotExist(err) {
				} else {
					os.RemoveAll(nginx_unit.DIR + "/" + dir_name)
				}
			}
			if err := os.Mkdir(nginx_unit.DIR+"/"+header.Name, 0755); err != nil {
				log.Fatalf("ExtractTarGz: Mkdir() failed: %s", err.Error())
				return err
			}
		case tar.TypeReg:
			outFile, err := os.Create(nginx_unit.DIR + "/" + header.Name)
			if err != nil {
				log.Fatalf("ExtractTarGz: Create() failed: %s", err.Error())
				return err
			}
			if _, err := io.Copy(outFile, tarReader); err != nil {
				log.Fatalf("ExtractTarGz: Copy() failed: %s", err.Error())
				return err
			}
			outFile.Close()

		default:
			log.Fatalf(
				"ExtractTarGz: uknown type: %v in %v",
				header.Typeflag,
				project)
		}

	}
	if err := os.Rename(nginx_unit.DIR+"/"+dir_name, nginx_unit.DIR+"/"+project); err == nil {

	}
	return err
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
		util.JsonResponse(rw, res)

	}

}
func NginxUpdate() http.HandlerFunc {
	return func(rw http.ResponseWriter, req *http.Request) {
		//code, err := ioutil.ReadAll(req.Body)
		res := model.CallApiResponse{
			Header:       model.Header{},
			ResponseBody: make(map[string]interface{}),
		}
		dstArr := req.URL.Query()["project"]
		portArr := req.URL.Query()["port"]
		var dst string = ""
		var port string = ""
		if len(dstArr) > 0 {
			dst = dstArr[0]
		}
		if len(portArr) > 0 {
			port = portArr[0]
		}
		if new_config, er := nginx_unit.AppendServer("127.0.0.1:"+port, dst); er == nil {
			if resUpdate, er := util.DockerSingletonInstance.Run(nginx_unit.UpdateConfig, new_config); er == nil {
				res.ResponseBody = resUpdate
			} else {
				res.ResponseBody["error"] = er.Error()
			}
		} else {
			res.ResponseBody["error"] = er.Error()
		}
		util.JsonResponse(rw, res)
	}
}
