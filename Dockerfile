FROM rustembedded/cross:x86_64-unknown-linux-gnu-0.2.1 AS linux-x86_64

COPY openssl.sh /
RUN bash /openssl.sh linux-x86_64

RUN apt-get update && apt-get install -y \
    pkg-config \
    libudev-dev \
    && rm -rf /var/lib/apt/lists/*

ENV OPENSSL_DIR=/openssl \
    OPENSSL_INCLUDE_DIR=/openssl/include \
    OPENSSL_LIB_DIR=/openssl/lib \
    OPENSSL_STATIC=1 \
    PKG_CONFIG_SYSROOT_DIR=/


FROM rustembedded/cross:aarch64-unknown-linux-gnu-0.2.1 AS linux-aarch64

COPY openssl.sh /
RUN bash /openssl.sh linux-aarch64 aarch64-linux-gnu-

RUN apt-get update && apt-get install -y \
    pkg-config \
    libudev-dev \
    && rm -rf /var/lib/apt/lists/*

ENV OPENSSL_DIR=/openssl \
    OPENSSL_INCLUDE_DIR=/openssl/include \
    OPENSSL_LIB_DIR=/openssl/lib \
    OPENSSL_STATIC=1 \
    PKG_CONFIG_SYSROOT_DIR=/

FROM rustembedded/cross:aarch64-unknown-linux-musl-0.2.1 AS linux-aarch64-musl

COPY openssl.sh /
RUN bash /openssl.sh linux-aarch64 aarch64-linux-musl-

RUN apt-get update && apt-get install -y \
    pkg-config \
    libudev-dev \
    && rm -rf /var/lib/apt/lists/*

ENV OPENSSL_DIR=/openssl \
    OPENSSL_INCLUDE_DIR=/openssl/include \
    OPENSSL_LIB_DIR=/openssl/lib \
    OPENSSL_STATIC=1 \
    PKG_CONFIG_SYSROOT_DIR=/