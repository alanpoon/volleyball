# capability-provider/game-provider/Makefile
CAPABILITY_ID = wasmcloud:thread
NAME = "Game Provider"
VENDOR = "rustropy"
PROJECT = game-provider
VERSION = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
REVISION = 0

include ../build/makefiles/provider.mk

test:
	cargo test -- --nocapture