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
# Install needed packages and setup non-root user. Use a separate RUN statement to add your own dependencies.
#COPY library-scripts/*.sh library-scripts/*.env /tmp/library-scripts/
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && /bin/bash /tmp/library-scripts/common-debian.sh "${INSTALL_ZSH}" "${USERNAME}" "${USER_UID}" "${USER_GID}" "${UPGRADE_PACKAGES}" "true" "true" \
#     #
    # ****************************************************************************
    # * TODO: Add any additional OS packages you want included in the definition *
    # * here. We want to do this before cleanup to keep the "layer" small.       *
    # ****************************************************************************
    # && apt-get -y install --no-install-recommends <your-package-list-here> \
    #
    #&& apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/* /tmp/library-scripts
# RUN apt install glibc-source -y
RUN apt-get install libcurl4-openssl-dev \
    && curl -s https://packagecloud.io/install/repositories/wasmcloud/core/script.deb.sh | bash && apt install wash \
    && which wash
RUN echo $WASM_BINDGEN_VERSION
RUN cargo install -f wasm-bindgen-cli --vers 0.2.83
RUN cargo install -f wasm-snip
RUN cargo install -f cargo-make
RUN rustup target add wasm32-unknown-unknown
RUN apt-get install iputils-ping -y
RUN cargo install -f basic-http-server

RUN wget https://github.com/nats-io/nats-rest-config-proxy/releases/download/v0.7.0/nats-rest-config-proxy-v0.7.0-linux-amd64.deb && dpkg -i nats-rest-config-proxy-v0.7.0-linux-amd64.deb
#ENTRYPOINT ["nohup","nats-rest-config-proxy","&"]

#CMD ["nats-rest-config-proxy"]
# sudo nohup nats-rest-config-proxy -d /etc/nats-rest-config-proxy-data > t2.out > /dev/null 2>&1 &
#RUN nohup nats-rest-config-proxy -d /etc/nats-rest-config-proxy-data > nats-rest.out 2>&1 < /dev/null &

#  [Optional] Uncomment this section to install additional OS packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>
# RUN wget -c https://ftp.gnu.org/gnu/glibc/glibc-2.29.tar.gz \
#     && tar -zxvf glibc-2.29.tar.gz \
#     && cd glibc-2.29 \
#     && ./configure --prefix=/opt/glibc \
#     && make \
#     && make install
