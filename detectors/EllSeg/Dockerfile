FROM ommatidia:0.1 AS python_environment
COPY env.yaml env.yaml
RUN $MICROMAMBA_INSTALL env.yaml && $MICROMAMBA_CLEAR

FROM python_environment
COPY --chown=$MAMBA_USER:$MAMBA_USER src/detector.py ${PUPIL_DETECTOR_DIR}/detector.py
COPY --chown=$MAMBA_USER:$MAMBA_USER src/implementation ${PUPIL_DETECTOR_DIR}/implementation