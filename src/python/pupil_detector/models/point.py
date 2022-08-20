# coding: utf-8

from typing import Optional

from pydantic import BaseModel, Field, validator


class Point(BaseModel):
    """
    The center of the pupil detected within a sample.
    """

    type: str = Field("Point", alias="type", const=True)
    x: float = Field(alias="x")
    y: float = Field(alias="y")
    confidence: Optional[float] = Field(alias="confidence", default=None)

    @validator("x")
    def x_max(cls, value):
        assert value <= 1
        return value

    @validator("x")
    def x_min(cls, value):
        assert value >= 0
        return value

    @validator("y")
    def y_max(cls, value):
        assert value <= 1
        return value

    @validator("y")
    def y_min(cls, value):
        assert value >= 0
        return value


Point.update_forward_refs()
