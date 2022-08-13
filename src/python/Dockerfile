FROM mambaorg/micromamba:latest as builder

ARG MAMBA_DOCKERFILE_ACTIVATE=1

# Those envs could be used in the "child" files
ENV PUPIL_DETECTOR_DIR=pupil_detector
ENV MICROMAMBA_INSTALL='micromamba install -y -n base -f '
ENV MICROMAMBA_CLEAR='micromamba clean --all --yes '

# Define dependencies always required
ARG VERSION_PYTHON=3.7
ARG VERSION_OPENCV=3.4
ARG VERSION_NUMPY=1.19.2
# Those are likely not influenced by the dependent image
ARG VERSION_FASTAPI=0.78.0
ARG VERSION_REQUESTS=2.25.1
ARG VERSION_UVICORN=0.18.2

COPY --chown=$MAMBA_USER:$MAMBA_USER pupil_detector ${PUPIL_DETECTOR_DIR}

# Install LibGL for Conda
USER root
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive  \
    && apt-get -y install --no-install-recommends libgl1 \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/*

USER $MAMBA_USER
RUN micromamba install -y -n base --channel conda-forge python=${VERSION_PYTHON} fastapi=${VERSION_FASTAPI} opencv=${VERSION_OPENCV} numpy=${VERSION_NUMPY} requests=${VERSION_REQUESTS} uvicorn=${VERSION_UVICORN} && $MICROMAMBA_CLEAR

CMD ["uvicorn", "pupil_detector.main:app", "--host", "0.0.0.0", "--port", "8080"]
EXPOSE 8080
