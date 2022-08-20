# coding: utf-8

from typing import List

from pydantic import BaseModel, Field


class MetaData(BaseModel):
    """
    Associated metadata with a pupil detector.
    """

    name: str = Field(alias="name")
    additional_information: str = Field(alias="additional_information")
    authors: List[str] = Field(alias="authors")
    license: str = Field(alias="license")
    prediction: str = Field(alias="prediction")


MetaData.update_forward_refs()
