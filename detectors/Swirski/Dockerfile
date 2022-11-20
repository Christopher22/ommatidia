FROM ommatidia_opencv3416:0.7 AS builder
COPY . /ommatidia/executable

RUN apt update && \
    apt -y install --no-install-recommends libtbb-dev libboost-dev && \
    apt clean && rm -rf /var/lib/apt/lists/*

RUN mkdir /ommatidia/executable/build \
    && cd /ommatidia/executable/build \
    && cmake -DCMAKE_BUILD_TYPE=Release .. \
    && cmake --build . \
    && cd /ommatidia \
    && cp /ommatidia/executable/build/Swirski . \
    && rm -rf /ommatidia/executable
CMD ["/ommatidia/Swirski"]
EXPOSE 8080 