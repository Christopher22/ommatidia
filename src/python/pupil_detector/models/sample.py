# coding: utf-8

from pydantic import BaseModel, Field, validator

class Sample(BaseModel):
    width: int = Field(alias="width")
    height: int = Field(alias="height")
    
    @validator("width")
    def x_min(cls, value):
        assert value >= 0
        return value

    @validator("height")
    def y_min(cls, value):
        assert value >= 0
        return value