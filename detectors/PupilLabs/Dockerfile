FROM ommatidia-py3.6.12-cv4.2-np1.19.2:0.1 AS python_environment

# Just create the environment and isntall the default
RUN micromamba install -y -n base --channel anaconda --channel conda-forge $MICROMAMBA_INSTALL_DEFAULT && $MICROMAMBA_CLEAR
SHELL ["micromamba", "run", "-n", "base", "/bin/bash", "-c"]

# Required by scikit-builder
USER root
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive  \
    && apt-get -y install --no-install-recommends build-essential \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/*
USER $MAMBA_USER

# pupil-detectors does not specify any version for its packages. To avoid bugs like https://github.com/opencv/opencv-python/issues/871, we fix that dependency
RUN python -m pip install --no-cache-dir opencv-python==4.4.0.40 pupil-detectors==2.0.2

FROM python_environment
COPY --chown=$MAMBA_USER:$MAMBA_USER src/detector.py ${PUPIL_DETECTOR_DIR}/detector.py