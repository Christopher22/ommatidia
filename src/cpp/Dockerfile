FROM debian:bullseye-slim

LABEL version="0.1"
LABEL description="Base image installing the ommatidia library and all required toolkits to build C++"

COPY . /ommatidia/library
WORKDIR /ommatidia

RUN apt-get update \
    && apt-get -y install --no-install-recommends cmake build-essential wget git ca-certificates libboost-all-dev libopencv-dev \
    && mkdir /ommatidia/library/build \
    && cd /ommatidia/library/build \
    && cmake .. \
    && cmake --build . --target install \
    && cd /ommatidia \
    && rm -rf library