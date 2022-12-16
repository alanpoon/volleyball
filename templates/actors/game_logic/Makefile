# examples/actor/echo-messaging
.PHONY:
PROJECT  = game_logic
VERSION  = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
REVISION = 0
# list of all contract claims for actor signing (space-separated)
CLAIMS   = wasmcloud:messaging wasmcloud:builtin:logging wasmcloud:thread wasmcloud:builtin:numbergen
#CLAIMS   = wasmcloud:thread wasmcloud:messaging
# registry url for our actor
REG_URL  = $(shell if [  -z "$(REMOTE)" ]; then echo registry:5000/v2/$(PROJECT):$(VERSION); else echo $(REMOTE):5000/v2/$(PROJECT):$(VERSION);fi )
DEFAULT_REG_URL  = registry:5000/v2/$(PROJECT):$(VERSION)

#REG_URL  = registry:5000/v2/$(PROJECT):$(VERSION)
# command to upload to registry (without last wasm parameter)
PUSH_REG_CMD = wash reg push --insecure $(REG_URL)

include ../../makefiles/actor.mk

ACTOR_ID=$(shell make --silent actor_id )

CALL_ALIAS = game_logic
# messaging provider id from wasmcloud.azurecr.io/nats
# If you built it from source, get the provider id with `make inspect` in the source folder
MESSAGING_PROVIDER_ID = VADNMSIML2XGO2X4TPIONTIC55R2UUQGPPDZPAVSC2QD7E76CR77SPW7
THREAD_PROVIDER_ID = VBYISBTQPDQOMMEWQN3Q6HMYNAIEBF6HAJQOTTCV5IU4KAQRXDVLTJFZ

start-thread:
	@wash call -o json $(ACTOR_ID) --cluster-seed SCALV3N2M5JRUOCSLTRTAIQYJSBKNZ3XMGME2XI3L3OSR2RRSYUTMHAN7E Thread.StartThread \
		'{"game_id":"hello","sleep_interval":30}'