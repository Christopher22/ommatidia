# coding: utf-8

from typing import Optional, List

from pydantic import BaseModel, Field
import numpy as np


class Annotation(BaseModel):
    """
    The annotation marking a pixel as pupil.
    """

    x: int = Field(alias="x")
    y: int = Field(alias="y")


class Mask(BaseModel):
    """
    A mask annotating the pupil within a frame.
    """

    type: str = Field("Mask", alias="type", const=True)
    mask: List[Annotation] = Field(alias="mask")
    confidence: Optional[float] = Field(alias="confidence", default=None)

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
