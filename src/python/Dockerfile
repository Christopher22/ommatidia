ARG MICROMAMBA_TAG=focal
FROM mambaorg/micromamba:$MICROMAMBA_TAG as builder

# Enable usage of mamba within the Dockerfile
ARG MAMBA_DOCKERFILE_ACTIVATE=1

# The "base implementation" requires Python, OpenCV, and Numpy.
# Consequently, we enforce their install while the versions might be changed.
ARG VERSION_PYTHON=3.7
ARG VERSION_OPENCV=3.4
ARG VERSION_NUMPY=1.19.2
# Those are likely not influenced by the dependent image
# While they are not up-to-date, they allow usage with Python 3.6
ARG VERSION_FASTAPI=0.70.0
ARG VERSION_REQUESTS=2.25.1
ARG VERSION_UVICORN=0.15.0
# For some reason, fastapi requires contextlib2 but does not install it
ARG VERSION_CONTEXTLIB2=21.6.0

# Copy the pupil detector source directory
ENV PUPIL_DETECTOR_DIR=pupil_detector
COPY --chown=$MAMBA_USER:$MAMBA_USER pupil_detector ${PUPIL_DETECTOR_DIR}

# Install LibGL for Conda
USER root
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive  \
    && apt-get -y install --no-install-recommends libgl1 \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/*
USER $MAMBA_USER

# Those commands must be used within the child Dockerfiles
ENV MICROMAMBA_INSTALL='micromamba install -y -n base --channel nvidia --channel anaconda --channel conda-forge --channel pytorch --channel bioconda -f '
ENV MICROMAMBA_INSTALL_DEFAULT=" python=${VERSION_PYTHON} fastapi=${VERSION_FASTAPI} opencv=${VERSION_OPENCV} numpy=${VERSION_NUMPY} requests=${VERSION_REQUESTS} uvicorn=${VERSION_UVICORN} contextlib2=${VERSION_CONTEXTLIB2}"
ENV MICROMAMBA_CLEAR='micromamba clean --all --yes '

CMD ["uvicorn", "pupil_detector.main:app", "--host", "0.0.0.0", "--port", "8080"]
EXPOSE 8080
