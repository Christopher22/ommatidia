FROM ommatidia-py3.6.12-cv4.2-np1.19.2:0.1 AS python_environment
ARG MAMBA_DOCKERFILE_ACTIVATE=1
COPY requirements_conda.txt requirements_conda.txt
COPY requirements_pip.txt requirements_pip.txt
RUN $MICROMAMBA_INSTALL requirements_conda.txt $MICROMAMBA_INSTALL_DEFAULT && $MICROMAMBA_CLEAR
RUN pip install --no-cache-dir -r requirements_pip.txt

FROM python_environment
COPY --chown=$MAMBA_USER:$MAMBA_USER src/detector.py ${PUPIL_DETECTOR_DIR}/detector.py
COPY --chown=$MAMBA_USER:$MAMBA_USER src/implementation ${PUPIL_DETECTOR_DIR}/implementation