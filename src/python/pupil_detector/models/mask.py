# coding: utf-8

from typing import Optional, List

from pydantic import BaseModel, Field, validator
import numpy as np

from .sample import Sample

class Annotation(BaseModel):
    """
    The annotation marking a pixel as pupil.
    """

    x: int = Field(alias="x")
    y: int = Field(alias="y")

    @validator("x")
    def x_min(cls, value):
        assert value >= 0
        return value

    @validator("y")
    def y_min(cls, value):
        assert value >= 0
        return value

class Mask(BaseModel):
    """
    A mask annotating the pupil within a frame.
    """

    type: str = Field("Mask", alias="type", const=True)
    mask: List[Annotation] = Field(alias="mask")
    confidence: Optional[float] = Field(alias="confidence", default=None)
    sample: Optional[Sample] = Field(alias="sample", default=None)
    
    @staticmethod
    def from_numpy(mask: np.ndarray) -> "Mask":
        """
        Create a mask from a numpy array.
        """
        x_values, y_values = np.where(mask != 0)
        return Mask(
            type="Mask", mask=[Annotation(x=x, y=y) for x, y in zip(x_values, y_values)]
        )


Mask.update_forward_refs()
