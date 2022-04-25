FROM alpine:3.15.4
LABEL version="0.2"
LABEL description="Base image installing the ommatidia library and all required toolkits to build C++"

COPY . /ommatidia/library
WORKDIR /ommatidia

# Install all the requirements for building C++ projects
RUN apk add --no-cache alpine-sdk cmake ca-certificates wget opencv-dev boost-dev

# Install Crow globally
RUN mkdir /ommatidia/dependencies \
    && cd /ommatidia/dependencies \
    && git clone -b 'v1.0+1' --single-branch https://github.com/CrowCpp/Crow.git crow \
    && mkdir /ommatidia/dependencies/crow/build \
    && cd /ommatidia/dependencies/crow/build \
    && cmake -DCMAKE_BUILD_TYPE=Release -DCROW_BUILD_EXAMPLES=OFF -DCROW_BUILD_TESTS=OFF .. \
    && cmake --build . --target install \
    && cd /ommatidia \
    && rm -rf /ommatidia/dependencies

# Build and install the project
RUN mkdir /ommatidia/library/build \
    && cd /ommatidia/library/build \
    && cmake -DCMAKE_BUILD_TYPE=Release .. \
    && cmake --build . --target install \
    && cd /ommatidia \
    && rm -rf /ommatidia/library