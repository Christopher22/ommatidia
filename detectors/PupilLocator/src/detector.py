import numpy as np
import tensorflow.compat.v1 as tf
from pydantic import BaseModel

from . import AbstractDetector
from .models.point import Point
from .models.meta_data import MetaData
from .implementation.inferno import rescale, gray_normalizer, change_channel, load_model, upscale_preds
from .implementation.logger import Logger
from .implementation.config import config

# We need to put it here as we only want one global session
MODEL_TYPE = "INC"
MODEL_NAME = "3A4Bh-Ref25"
session = tf.Session().__enter__()
logger = Logger(MODEL_TYPE, MODEL_NAME, "", config, dir="implementation/models/")
model = load_model(session, MODEL_TYPE, MODEL_NAME, logger)

class Config(BaseModel):
    """An empty base model used for config."""

class Detector(AbstractDetector):
    def __init__(self, _: Config = Config()):
        pass

    def detect(self, frame: np.ndarray) -> Point:
        f_shape = frame.shape
        if frame.shape[0] != 192:
            frame = rescale(frame)

        image = gray_normalizer(frame)
        image = change_channel(image, config["input_channel"])
        [p] = model.predict(session, [image])
    
        x, y, _ = upscale_preds(p, f_shape)
        return Point(x=x, y=y)
    
    @classmethod
    def metadata(cls) -> MetaData:
        return MetaData(
            name="Improving real-time CNN-based pupil detection through domain-specific data augmentation",
            additional_information="https://doi.org/10.1145/3314111.3319914",
            authors=["S. Eivazi", "T. Santini", "A. Keshavarzi", "T. Kübler", "A. Mazzei"],
            license="MIT",
            prediction="Point",
        )