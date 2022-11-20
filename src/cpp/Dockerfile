# -------------------------------------------
# The image used to download the dependencies
# -------------------------------------------
FROM debian:bullseye-slim as dependencies

ARG OPENCV_VERSION=3.4.2
ARG CROW_VERSION=master

# Install everything to download the dependencies
RUN apt update && \
    apt -y install --no-install-recommends ca-certificates wget unzip git && \
    apt clean && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /ommatidia/dependencies && cd /ommatidia/dependencies \
    && wget -O opencv.zip https://github.com/opencv/opencv/archive/$OPENCV_VERSION.zip && unzip opencv.zip \
    && git clone -b $CROW_VERSION --single-branch https://github.com/CrowCpp/Crow.git crow


# -------------------------------------------
# The "real" image with the library
# -------------------------------------------
FROM debian:bullseye-slim
LABEL maintainer="christopher@gundler.de"
LABEL version="0.7"
LABEL description="Base image installing the ommatidia library and all required toolkits to build C++"

ARG OPENCV_VERSION=3.4.2
ARG COMMERCIAL_USAGE_ALLOWED=OFF

COPY ./include /ommatidia/library/include
COPY ./src /ommatidia/library/src
COPY ./tests /ommatidia/library/tests
COPY ./CMakeLists.txt /ommatidia/library/CMakeLists.txt
COPY --from=dependencies /ommatidia/dependencies /ommatidia/dependencies

WORKDIR /ommatidia

# Install all the requirements for building C++ projects
RUN apt update && \
    apt -y install --no-install-recommends cmake build-essential libasio-dev git ca-certificates && \
    apt clean && rm -rf /var/lib/apt/lists/*

# Install OpenCV with the specific version
RUN cd /ommatidia/dependencies && mkdir -p opencv_build && cd opencv_build \
    && cmake -DCMAKE_BUILD_TYPE=Release -DBUILD_SHARED_LIBS=OFF -DENABLE_PIC=ON -DBUILD_LIST=core,imgproc,imgcodecs -DBUILD_opencv_world=OFF -DBUILD_ITT=OFF -DWITH_1394=OFF -DWITH_PROTOBUF=OFF -DWITH_GSTREAMER=OFF -DWITH_IPP=OFF -DWITH_JASPER=OFF -DWITH_WEBP=OFF -DWITH_OPENEXR=OFF -DWITH_PVAPI=OFF -DWITH_GIGEAPI=OFF -DWITH_QT=OFF -DWITH_TIFF=OFF -DWITH_OPENCL=OFF -DWITH_ADE=OFF -DWITH_CAROTENE=OFF -DWITH_OPENJPEG=OFF -DWITH_IMGCODEC_HDR=OFF -DWITH_IMGCODEC_PFM=OFF -DWITH_IMGCODEC_PXM=OFF -DWITH_IMGCODEC_SUNRASTER=OFF -DWITH_QUIRC=OFF -DBUILD_opencv_apps=OFF -DBUILD_DOCS=OFF -DBUILD_PERF_TESTS=OFF -DBUILD_TESTS=OFF -DBUILD_FAT_JAVA_LIB=OFF -DENABLE_PRECOMPILED_HEADERS=OFF ../opencv-$OPENCV_VERSION \
    && cmake --build . --target install

# Install Crow globally
RUN mkdir -p /ommatidia/dependencies/crow/build && cd /ommatidia/dependencies/crow/build \
    && cmake -DCMAKE_BUILD_TYPE=Release -DCROW_BUILD_EXAMPLES=OFF -DCROW_BUILD_TESTS=OFF .. \
    && cmake --build . --target install

# Clean up dependency artifacts
RUN cd /ommatidia && rm -rf /ommatidia/dependencies

# Build and install the project
RUN mkdir -p /ommatidia/library/lib_build && cd /ommatidia/library/lib_build \
    && cmake -DCMAKE_BUILD_TYPE=Release -DALLOW_COMMERCIAL_USE=$COMMERCIAL_USAGE_ALLOWED .. \
    && cmake --build . --target install \
    && cd /ommatidia && rm -rf /ommatidia/library