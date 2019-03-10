FROM ubuntu:bionic as build

RUN apt-get -y update && apt-get -y install curl g++ libssl-dev pkg-config musl-tools
ENV RUST_VERSION 1.32.0
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain ${RUST_VERSION}
ENV PATH $PATH:/root/.cargo/bin
RUN rustup target add x86_64-unknown-linux-musl
RUN mkdir source && mkdir .cargo && echo "[target.x86_64-unknown-linux-musl]\n" > .cargo/config

ENV SSL_VER 1.0.2o
ENV CC musl-gcc
ENV PREFIX /usr/local
ENV PATH /usr/local/bin:$PATH
ENV PKG_CONFIG_PATH /usr/local/lib/pkgconfig

RUN curl -sL http://www.openssl.org/source/openssl-$SSL_VER.tar.gz | tar xz && \
        cd openssl-$SSL_VER && \
        ./Configure no-shared --prefix=$PREFIX --openssldir=$PREFIX/ssl no-zlib linux-x86_64 -fPIC && \
        make -j$(nproc) && make install && cd .. && rm -rf openssl-$SSL_VER

ENV SSL_CERT_FILE /etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR /etc/ssl/certs
ENV OPENSSL_LIB_DIR $PREFIX/lib
ENV OPENSSL_INCLUDE_DIR $PREFIX/include
ENV OPENSSL_DIR $PREFIX
ENV OPENSSL_STATIC 1
ENV PKG_CONFIG_ALLOW_CROSS 1

WORKDIR /source
COPY ./ ./

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

RUN cargo build --target x86_64-unknown-linux-musl --release
RUN mkdir -p /build-out
RUN cp target/x86_64-unknown-linux-musl/release/weather_report /build-out/

###############################################################################

FROM alpine:3.9

RUN apk --no-cache add gettext

RUN mkdir /app
WORKDIR /app

COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=build /build-out/weather_report /app

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs

EXPOSE 3000

COPY ./config.json.template /app
COPY ./entrypoint.sh /app
ENTRYPOINT ["/app/entrypoint.sh"]
