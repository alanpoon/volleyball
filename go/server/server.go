package server

import (
	"errors"
	"fmt"
	"html/template"
	"net/http"
	"net/http/httptest"
	"os"

	"github.com/alanpoon/bevy_cloud_server/logger"
	"github.com/alanpoon/bevy_cloud_server/model"
	"github.com/alanpoon/bevy_cloud_server/processor"
	"github.com/alanpoon/bevy_cloud_server/util"
	"github.com/gorilla/mux"
	"github.com/rs/cors"
)

var tmpl template.Template

func NewServer(rpc_able processor.Able) {
	handleHTTP(rpc_able)
}
func RecoverWrap(h http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var err error
		defer func() {
			r := recover()
			if r != nil {
				switch t := r.(type) {
				case string:
					err = errors.New(t)
				case error:
					err = t
				default:
					err = errors.New("Unknown error")
				}
				http.Error(w, err.Error(), http.StatusInternalServerError)
			}
		}()
		h.ServeHTTP(w, r)
	})
}
func handleHTTP(rpc_able processor.Able) {
	logger.Infof("Starting HTTP server...")
	addr := ":20827"
	port := os.Getenv("PORT_client")
	if port != "" {
		addr = ":" + port
	}

	rtr := mux.NewRouter()
	c := cors.New(cors.Options{
		AllowedOrigins:   []string{"*"},
		AllowCredentials: true,
	})
	rtr.HandleFunc("/ping", func(rw http.ResponseWriter, req *http.Request) {
		res := model.CallApiResponse{
			Header: model.Header{
				Message: "pong",
			},
		}
		util.JsonResponse(rw, res)
	})
	rtr.HandleFunc("/upload_static", processor.UploadStatic())
	rtr.HandleFunc("/nginx_update", processor.NginxUpdate())
	handler := c.Handler(rtr)
	logger.Infof("Http Server on: " + addr)
	//fs := http.FileServer(http.Dir("public"))
	if err := http.ListenAndServe(addr, handler); err != nil {
		logger.Errorf("Failed to listen HTTP | err=%s", err.Error())
	}
	return
}

func logHandler(fn http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		rec := httptest.NewRecorder()
		fn(rec, r)
		logger.Infof("url:%v, method:%v, req:%v, resp:%v", r.URL, r.Method, r.Body, rec.Body)
		w.WriteHeader(rec.Code)
		_, err := rec.Body.WriteTo(w)
		if err != nil {
			http.Error(w, fmt.Sprint(err), http.StatusInternalServerError)
			return
		}
	}
}
