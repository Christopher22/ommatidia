FROM ommatidia-py3.6.12-cv4.2-np1.19.2:0.1 AS python_environment

# We need to install an binary version of OpenCV compatible with the Python version
USER root
ARG OPENCV_VERSION=4.2.0
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive  \
    && apt-get -y install --no-install-recommends build-essential cmake libeigen3-dev wget unzip ca-certificates \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/*
RUN mkdir -p /ommatidia/dependencies && cd /ommatidia/dependencies \
    && wget -O opencv.zip https://github.com/opencv/opencv/archive/$OPENCV_VERSION.zip && unzip opencv.zip \
    && mkdir -p opencv_build && cd opencv_build \
    && cmake -DCMAKE_BUILD_TYPE=Release -DBUILD_SHARED_LIBS=OFF -DENABLE_PIC=ON -DBUILD_LIST=core,imgproc,imgcodecs -DBUILD_opencv_world=OFF -DBUILD_ITT=OFF -DWITH_1394=OFF -DWITH_PROTOBUF=OFF -DWITH_GSTREAMER=OFF -DWITH_IPP=OFF -DWITH_JASPER=OFF -DWITH_WEBP=OFF -DWITH_OPENEXR=OFF -DWITH_PVAPI=OFF -DWITH_GIGEAPI=OFF -DWITH_QT=OFF -DWITH_TIFF=OFF -DWITH_OPENCL=OFF -DWITH_ADE=OFF -DWITH_CAROTENE=OFF -DWITH_OPENJPEG=OFF -DWITH_IMGCODEC_HDR=OFF -DWITH_IMGCODEC_PFM=OFF -DWITH_IMGCODEC_PXM=OFF -DWITH_IMGCODEC_SUNRASTER=OFF -DWITH_QUIRC=OFF -DBUILD_opencv_apps=OFF -DBUILD_DOCS=OFF -DBUILD_PERF_TESTS=OFF -DBUILD_TESTS=OFF -DBUILD_FAT_JAVA_LIB=OFF -DENABLE_PRECOMPILED_HEADERS=OFF ../opencv-$OPENCV_VERSION \
    && cmake --build . --target install \
    && cd /ommatidia && rm -rf /ommatidia/dependencies
USER $MAMBA_USER

ARG MAMBA_DOCKERFILE_ACTIVATE=1
COPY requirements_conda.txt requirements_conda.txt
COPY requirements_pip.txt requirements_pip.txt
RUN $MICROMAMBA_INSTALL requirements_conda.txt $MICROMAMBA_INSTALL_DEFAULT && $MICROMAMBA_CLEAR
RUN pip install --no-cache-dir -r requirements_pip.txt

FROM python_environment
COPY --chown=$MAMBA_USER:$MAMBA_USER src/detector.py ${PUPIL_DETECTOR_DIR}/detector.py