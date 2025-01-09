FROM rustembedded/cross:armv7-unknown-linux-gnueabihf-0.2.1

RUN apt-get update
RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get install --assume-yes \
    gcc-arm-linux-gnueabihf \
    binutils-arm-linux-gnueabihf \
    build-essential \
    wget \
    curl \
    tar \
    perl \
    zlib1g-dev:armhf \
    libssl-dev:armhf \
    pkg-config:armhf

#ENV PKG_CONFIG_PATH=/usr/lib/arm-linux-gnueabihf/pkgconfig
#ENV PKG_CONFIG_SYSROOT_DIR=/
#ENV OPENSSL_INCLUDE_DIR=/usr/include/arm-linux-gnueabihf
#ENV OPENSSL_LIB_DIR=/usr/lib/arm-linux-gnueabihf
#ENV OPENSSL_DIR=/usr

RUN wget https://www.openssl.org/source/openssl-3.0.12.tar.gz && \
    tar -xvzf openssl-3.0.12.tar.gz && \
    cd openssl-3.0.12 && \
    ./Configure linux-armv4 --cross-compile-prefix=arm-linux-gnueabihf- --prefix=/usr/local/openssl-3.0.12 && \
    make -j$(nproc) && \
    make install && \
    rm -rf /openssl-3.0.12.tar.gz /openssl-3.0.12

ENV OPENSSL_DIR=/usr/local/openssl-3.0.12
ENV OPENSSL_LIB_DIR=/usr/local/openssl-3.0.12/lib
ENV OPENSSL_INCLUDE_DIR=/usr/local/openssl-3.0.12/include
ENV PKG_CONFIG_PATH=/usr/local/openssl-3.0.12/lib/pkgconfig:$PKG_CONFIG_PATH
ENV LD_LIBRARY_PATH=/usr/local/openssl-3.0.12/lib:$LD_LIBRARY_PATH