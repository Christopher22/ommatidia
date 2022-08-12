FROM ommatidia:0.5 AS builder
COPY . /ommatidia/executable

# We need to download old versions of TBB and Boost
RUN printf "\ndeb http://deb.debian.org/debian/ buster main" >> /etc/apt/sources.list \
    && apt update && apt -y install --no-install-recommends libtbb-dev=2018~U6-4 libboost-dev=1.67.0.1 \
    && apt clean && rm -rf /var/lib/apt/lists/*

# We need to modify CMAKE_MODULE_PATH manually
RUN mkdir /ommatidia/executable/build \
    && cd /ommatidia/executable/build \
    && cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_MODULE_PATH=/usr/share/cmake/Modules .. \
    && cmake --build . \
    && cd /ommatidia \
    && cp /ommatidia/executable/build/CPRD . \
    && rm -rf /ommatidia/executable

CMD ["/ommatidia/CPRD"]
EXPOSE 8080 