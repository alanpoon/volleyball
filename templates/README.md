https://docs.docker.com/registry/insecure/
openssl req \
  -newkey rsa:4096 -nodes -sha256 -keyout certs/domain.key \
  -addext "subjectAltName = DNS:registry" \
  -x509 -days 365 -out certs/domain.crt

//sudo chown -R vscode:rustlang /usr/local/cargo

docker logs -n 100 35ee0cbd06e1 2>&1 | grep "PUB"

docker logs -n 1000 35ee0cbd06e1 2>&1 | grep "SUB"

docker logs -n 6000 b2c6a34c034d 2>&1 | grep "welcome"

docker logs -n 1000 35ee0cbd06e1 2>&1 | grep "AAA"

docker logs -n 6000 983e09e41e0d 2>&1 | grep "help"

docker logs -n 200 2de6af269e67

docker stop $(docker ps -a -q) && docker rm $(docker ps -a -q)

wash 0.11.0
nats-server --signal reload=1


nats-rest-config-proxy -c /etc/nats-server.conf
curl -X GET http://localhost:4567/v1/auth/perms

curl -X PUT http://localhost:4567/v1/auth/idents/sample-user2 -d '{
  "username": "sample-user2",
  "password": "secret",
  "permissions": "sample-user2"
}'
curl http://localhost:4567/v1/auth/idents/sample-user
curl -X PUT http://127.0.0.1:4567/v1/auth/accounts/sample-account -d '{}'

curl -X PUT http://127.0.0.1:4567/v2/auth/accounts/sample-account2 -d '{}'
curl -X POST http://localhost:4567/v1/auth/snapshot?name=snap1
curl -X POST http://127.0.0.1:4567/v1/auth/publish?name=snap1

docker exec -it bevy_wasmcloud_codegen2_devcontainer-nats-1 nats-server --signal reload

docker logs -n 100 bevy_wasmcloud_codegen2_devcontainer-nats-1

bevy_wasmcloud_codegen2_devcontainer-app-1

docker exec -it bevy_wasmcloud_codegen2_devcontainer-app-1 /bin/bash

make init
make build
make start
make serve

docker exec -it bevy_wasmcloud_codegen2_devcontainer-wadm-1 update-locale LC_ALL=en_US.UTF-8
