.PHONY: build start
init:
	nats-rest-config-proxy -d /etc/nats-rest-config-proxy-data
build:
	wash reg push --insecure registry:5000/v2/game-provider:0.1.0 ../test/cap/game-provider.par.gz
	#wash reg push --insecure registry:5000/v2/nats-messaging:0.11.5 ../test/cap/nats_messaging.par.gz
	# wash reg push registry:443/v2/game-provider:0.1.0 ../test/cap/game-provider.par.gz
	# wash reg push registry:443/v2/nats-messaging:0.11.5 ../test/cap/nats_messaging.par.gz
	(cd actors/game_logic && make clean_wasm && make push)
	wash ctl apply NDP4B6DPQICPNEA3UJ7FOG4KR66Y56JCKNASH6UXWZECNNYFTVYJ4ROS configs/manifest.yaml
	sleep 15
start:
	(cd actors/game_logic && make start-thread)
serve:
	(cd www && cargo make wasm-serve2)