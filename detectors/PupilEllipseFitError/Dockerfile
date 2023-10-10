FROM ommatidia:0.1 AS python_environment
COPY --chown=$MAMBA_USER:$MAMBA_USER env.yaml env.yaml

# Fix "libgthread-2.0.so.0: cannot open shared object file: No such file or directory"
USER root
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive  \
    && apt-get -y install --no-install-recommends libglib2.0-0 \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/*
USER $MAMBA_USER

RUN $MICROMAMBA_INSTALL env.yaml && $MICROMAMBA_CLEAR

FROM python_environment
COPY --chown=$MAMBA_USER:$MAMBA_USER src/detector.py ${PUPIL_DETECTOR_DIR}/detector.py
COPY --chown=$MAMBA_USER:$MAMBA_USER src/implementation ${PUPIL_DETECTOR_DIR}/implementation