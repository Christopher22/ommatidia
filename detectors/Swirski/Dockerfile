FROM ommatidia:0.3 AS builder
COPY . /ommatidia/executable
RUN mkdir /ommatidia/executable/build \
    && cd /ommatidia/executable/build \
    && cmake -DCMAKE_BUILD_TYPE=Release .. \
    && cmake --build . \
    && cd /ommatidia \
    && cp /ommatidia/executable/build/Swirski . \
    && rm -rf /ommatidia/executable
CMD ["/ommatidia/Swirski"]
EXPOSE 8080 