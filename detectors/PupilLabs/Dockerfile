FROM ommatidia:0.1 AS python_environment

# Required by scikit-builder
USER root
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive  \
    && apt-get -y install --no-install-recommends build-essential \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/*
USER $MAMBA_USER

# pupil-detectors does not specify any version for its packages. To avoid bugs like https://github.com/opencv/opencv-python/issues/871, we fix that dependency
COPY --chown=$MAMBA_USER:$MAMBA_USER env.yaml env.yaml
RUN $MICROMAMBA_INSTALL env.yaml && $MICROMAMBA_CLEAR

FROM python_environment
COPY --chown=$MAMBA_USER:$MAMBA_USER src/detector.py ${PUPIL_DETECTOR_DIR}/detector.py