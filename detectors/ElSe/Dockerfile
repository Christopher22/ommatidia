FROM ommatidia:0.2 AS builder
COPY . /ommatidia/executable
RUN mkdir /ommatidia/executable/build \
    && cd /ommatidia/executable/build \
    && cmake -DCMAKE_BUILD_TYPE=Release .. \
    && cmake --build . \
    && cd /ommatidia \
    && cp /ommatidia/executable/build/ElSe . \
    && rm -rf /ommatidia/executable
CMD ["/ommatidia/ElSe"]
EXPOSE 8080 