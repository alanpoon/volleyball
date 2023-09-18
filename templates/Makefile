.PHONY: build start
init:
	nats-rest-config-proxy -d /etc/nats-rest-config-proxy-data
	docker exec -it volleyball_devcontainer-wasmcloud-1 sh -c "sudo apt-get install "
	sudo chmod 777 /var/run/docker.sock
cross:
	# sed "s|BUILD_DIR|${FOLDER_DIR}|g" capability_providers/game-provider/Cross_template.toml > capability_providers/game-provider/Cross.toml
	# sed "s|BUILD_DIR|${FOLDER_DIR}|g" capability_providers/nats/Cross_template.toml > capability_providers/nats/Cross.toml
	sudo mkdir -p ${FOLDER_DIR}
	sudo chmod 777 ${FOLDER_DIR}
	cp -r /workspace/templates/capability_providers ${FOLDER_DIR}/templates
	cp -r /workspace/templates/interfaces ${FOLDER_DIR}/templates
	ln /workspace/templates ${FOLDER_DIR}/templates
	(cd ${FOLDER_DIR}/templates/capability_providers/game-provider && make par-full)
	(cd ${FOLDER_DIR}/templates/capability_providers/nats && make par-full)
build:
	wash reg push --insecure registry:5000/v2/game-provider:0.1.0 ../test/cap/game-provider.par.gz
	wash reg push --insecure registry:5000/v2/nats-messaging:0.11.5 ../test/cap/nats_messaging.par.gz
	# wash reg push registry:443/v2/game-provider:0.1.0 ../test/cap/game-provider.par.gz
	# wash reg push registry:443/v2/nats-messaging:0.11.5 ../test/cap/nats_messaging.par.gz
	#(cd actors/game_logic && make clean_wasm && make push)
	(cd actors/game_logic && make push)
	wash ctl apply NDP4B6DPQICPNEA3UJ7FOG4KR66Y56JCKNASH6UXWZECNNYFTVYJ4ROS configs/manifest.yaml
serve:
	(cd www && cargo make wasm-serve2)