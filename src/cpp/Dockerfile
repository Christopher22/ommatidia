FROM alpine:3.15.4
LABEL maintainer="christopher@gundler.de"
LABEL version="0.4"
LABEL description="Base image installing the ommatidia library and all required toolkits to build C++"

ARG OPENCV_VERSION=4.6.0
ARG CROW_VERSION=v1.0+4
ARG COMMERCIAL_USAGE_ALLOWED=OFF

COPY ./include /ommatidia/library/include
COPY ./src /ommatidia/library/src
COPY ./tests /ommatidia/library/tests
COPY ./CMakeLists.txt /ommatidia/library/CMakeLists.txt
WORKDIR /ommatidia

# Install all the requirements for building C++ projects
RUN apk add --no-cache alpine-sdk cmake ca-certificates wget boost-dev unzip

# Install OpenCV with the specific version
RUN mkdir /ommatidia/dependencies && cd /ommatidia/dependencies \
    && wget -O opencv.zip https://github.com/opencv/opencv/archive/$OPENCV_VERSION.zip && unzip opencv.zip \
    && mkdir -p opencv_build && cd opencv_build \
    && cmake -DCMAKE_BUILD_TYPE=Release -DBUILD_SHARED_LIBS=OFF -DENABLE_PIC=ON -DBUILD_LIST=core,imgproc,imgcodecs -DBUILD_opencv_world=OFF -DBUILD_ITT=OFF -DWITH_1394=OFF -DWITH_PROTOBUF=OFF -DWITH_GSTREAMER=OFF -DWITH_IPP=OFF -DWITH_JASPER=OFF -DWITH_WEBP=OFF -DWITH_OPENEXR=OFF -DWITH_PVAPI=OFF -DWITH_GIGEAPI=OFF -DWITH_QT=OFF -DWITH_TIFF=OFF -DWITH_OPENCL=OFF -DWITH_ADE=OFF -DWITH_CAROTENE=OFF -DWITH_OPENJPEG=OFF -DWITH_IMGCODEC_HDR=OFF -DWITH_IMGCODEC_PFM=OFF -DWITH_IMGCODEC_PXM=OFF -DWITH_IMGCODEC_SUNRASTER=OFF -DWITH_QUIRC=OFF -DBUILD_opencv_apps=OFF -DBUILD_DOCS=OFF -DBUILD_PERF_TESTS=OFF -DBUILD_TESTS=OFF -DBUILD_FAT_JAVA_LIB=OFF -DENABLE_PRECOMPILED_HEADERS=OFF ../opencv-$OPENCV_VERSION \
    && cmake --build . --target install \
    && cd /ommatidia && rm -rf /ommatidia/dependencies

# Install Crow globally
RUN mkdir /ommatidia/dependencies && cd /ommatidia/dependencies \
    && git clone -b $CROW_VERSION --single-branch https://github.com/CrowCpp/Crow.git crow \
    && mkdir /ommatidia/dependencies/crow/build && cd /ommatidia/dependencies/crow/build \
    && cmake -DCMAKE_BUILD_TYPE=Release -DCROW_BUILD_EXAMPLES=OFF -DCROW_BUILD_TESTS=OFF .. \
    && cmake --build . --target install \
    && cd /ommatidia && rm -rf /ommatidia/dependencies

# Build and install the project
RUN mkdir /ommatidia/library/lib_build && cd /ommatidia/library/lib_build \
    && cmake -DCMAKE_BUILD_TYPE=Release -DALLOW_COMMERCIAL_USE=$COMMERCIAL_USAGE_ALLOWED .. \
    && cmake --build . --target install \
    && cd /ommatidia && rm -rf /ommatidia/library