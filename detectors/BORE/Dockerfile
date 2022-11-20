FROM ommatidia_opencv3416:0.7 AS builder
COPY . /ommatidia/executable
RUN mkdir /ommatidia/executable/build \
    && cd /ommatidia/executable/build \
    && cmake -DCMAKE_BUILD_TYPE=Release .. \
    && cmake --build . \
    && cd /ommatidia \
    && cp /ommatidia/executable/build/BORE . \
    && rm -rf /ommatidia/executable
CMD ["/ommatidia/BORE"]
EXPOSE 8080 