# Use the [Choice] comment to indicate option arguments that should appear in VS Code UX. Use a comma separated list.
#
# [Choice] Debian OS version: bullseye, buster
ARG VARIANT="bullseye"
ARG WASM_BINDGEN_VERSION="0.2.83"
# FROM buildpack-deps:${VARIANT}-curl
FROM mcr.microsoft.com/devcontainers/rust:1-${VARIANT}
# Use the [Option] comment to specify true/false arguments that should appear in VS Code UX
#
# [Option] Install zsh
ARG INSTALL_ZSH="true"
# [Option] Upgrade OS packages to their latest versions
ARG UPGRADE_PACKAGES="false"
# Install needed packages and setup non-root user. Use a separate RUN statement to add your own dependencies.
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID
RUN apt-get install libcurl4-openssl-dev \
    && curl -s https://packagecloud.io/install/repositories/wasmcloud/core/script.deb.sh | bash && apt install wash \
    && which wash
RUN echo $WASM_BINDGEN_VERSION
RUN cargo install -f wasm-bindgen-cli --vers 0.2.83
RUN cargo install -f wasm-snip
RUN cargo install -f cargo-make
RUN rustup target add wasm32-unknown-unknown
RUN apt-get install iputils-ping -y
RUN wget "https://github.com/nats-io/nats-rest-config-proxy/releases/download/v0.7.0/nats-rest-config-proxy-v0.7.0-linux-$(uname -m | grep -q '^aarch64' && echo 'arm64' || echo 'amd64' ).deb" && dpkg -i "nats-rest-config-proxy-v0.7.0-linux-$(uname -m | grep -q '^aarch64' && echo 'arm64' || echo 'amd64' ).deb"
RUN cargo install cross --git https://github.com/cross-rs/cross
RUN apt-get update \
    && apt-get install -y apt-transport-https ca-certificates curl gnupg2 lsb-release \
    && curl -fsSL https://download.docker.com/linux/$(lsb_release -is | tr '[:upper:]' '[:lower:]')/gpg | apt-key add - 2>/dev/null \
    && echo "deb [arch=$(dpkg --print-architecture)] https://download.docker.com/linux/$(lsb_release -is | tr '[:upper:]' '[:lower:]') $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list \
    && apt-get update \
    && apt-get install -y docker-ce-cli
RUN sudo mkdir -p /home/vscode/.wash && sudo chmod 777 /home/vscode/.wash