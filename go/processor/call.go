package processor

import (
	"net/http"

	"github.com/alanpoon/bevy_cloud_server/util"

	"github.com/alanpoon/bevy_cloud_server/model"
)

func Call() http.HandlerFunc {
	return func(rw http.ResponseWriter, req *http.Request) {
		res := model.CallApiResponse{
			Header: model.Header{},
		}
		// dstArr := req.URL.Query()["dst"]
		// var dst string = ""
		// if len(dstArr) > 0 {
		// 	dst = dstArr[0]
		// }
		defer req.Body.Close()

		util.JsonResponse(rw, res)

	}
}
