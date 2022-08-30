# coding: utf-8

from typing import Optional  # noqa: F401

from pydantic import BaseModel, Field, validator  # noqa: F401


class Ellipse(BaseModel):
    """
    The pupil within an sample represented as ellipse.

    type: The type of this ellipse.
    x: The x position of this ellipse.
    y: The y position of this ellipse.
    major: The major axis of this ellipse.
    minor: The minor axis of this ellipse.
    rotation: The rotation of this ellipse in Radians.
    confidence: The optimal confidence of this ellipse.
    """

    type: str = Field("Ellipse", alias="type", const=True)
    x: float = Field(alias="x")
    y: float = Field(alias="y")
    major: float = Field(alias="major")
    minor: float = Field(alias="minor")
    rotation: float = Field(alias="rotation")
    confidence: Optional[float] = Field(alias="confidence", default=None)

    @validator("x")
    def x_min(cls, value):
        assert value >= 0
        return value

    @validator("y")
    def y_min(cls, value):
        assert value >= 0
        return value

    @validator("rotation")
    def rotation_max(cls, value):
        assert value <= 3.14159265359
        return value

    @validator("rotation")
    def rotation_min(cls, value):
        assert value >= -3.14159265359
        return value


Ellipse.update_forward_refs()
