# Bevy Game Cloud via Wasmcloud

In the dev container, cd templates && make build
## For Http
open browser in http://localhost:4002

## For Https using self-signed certificate
cd certs
mkcert -cert-file server-cert.pem -key-file server-key.pem localhost ::1
mkcert -client -cert-file client-cert.pem -key-file client-key.pem localhost ::1 email@localhost
mkcert -CAROOT
import ./certs/rootCA.pem into browser as a certificate
open browser in https://localhost:4003

## Recompiling Wasmcloud's capabilities providers
- Do it outside devcontainer
- cargo install cross --git https://github.com/cross-rs/cross
- Copy ./templates/keys to ~/.wash/keys when compiling capability_provider
- cd capability_provider/game-provider
- make par-full
Currently par_targets is set to only aarch64-unknown-linux-gnu and aarch64-apple-darwin. Change capability_providers/build/makefiles if you want others

## Caching
Cache is stored at the ../cargo2 folder

