FROM ommatidia-py3.6.12-cv3.4-np1.19.2:0.1 AS python_environment
COPY requirements.txt requirements.txt
RUN $MICROMAMBA_INSTALL requirements.txt $MICROMAMBA_INSTALL_DEFAULT && $MICROMAMBA_CLEAR

FROM python_environment
COPY --chown=$MAMBA_USER:$MAMBA_USER src/detector.py ${PUPIL_DETECTOR_DIR}/detector.py
COPY --chown=$MAMBA_USER:$MAMBA_USER src/implementation ${PUPIL_DETECTOR_DIR}/implementation