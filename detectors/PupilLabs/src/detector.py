import math

import numpy as np
from pydantic import BaseModel
from pupil_detectors import Detector2D

from . import AbstractDetector
from .models.ellipse import Ellipse
from .models.meta_data import MetaData

class Config(BaseModel):
    """An empty base model used for config."""

class Detector(AbstractDetector):
    def __init__(self, _: Config = Config()):
        self._detector = Detector2D()

    def detect(self, frame: np.ndarray) -> Ellipse:
        result = self._detector.detect(frame)["ellipse"]
        return Ellipse(
            type="Ellipse",
            x=result["center"][0],
            y=result["center"][1],
            major=result["axes"][0],
            minor=result["axes"][1],
            rotation=result["angle"] * (math.pi / 180.0)
        )
    
    @classmethod
    def metadata(cls) -> MetaData:
        return MetaData(
            name="Pupil: An Open Source Platform for Pervasive Eye Tracking and Mobile Gaze-based Interaction",
            additional_information="http://doi.acm.org/10.1145/2638728.2641695",
            authors=["M. Kassner", "W. Patera", "A. Bulling"],
            license="GPT",
            prediction="Ellipse",
        )