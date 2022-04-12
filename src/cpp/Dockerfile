FROM debian:bullseye-slim

LABEL version="0.1"
LABEL description="Base image installing the ommatidia library and all required toolkits to build C++"

COPY . /ommatidia/library
WORKDIR /ommatidia

RUN apt-get update \
    && apt-get -y install --no-install-recommends cmake build-essential wget git ca-certificates libboost-all-dev libopencv-dev \
    && wget https://github.com/CrowCpp/Crow/releases/download/v1.0%2B1/crow-v1.0+1.deb \
    && apt install ./crow-v1.0+1.deb \
    && mkdir /ommatidia/library/build \
    && cd /ommatidia/library/build \
    && cmake .. \
    && cmake --build . --target install \
    && cd /ommatidia \
    && rm -rf library