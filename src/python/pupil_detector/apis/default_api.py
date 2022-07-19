# coding: utf-8

from fastapi import APIRouter, Response

from ..detector import Detector, Config
from ..models.meta_data import MetaData


router = APIRouter()


@router.get(
    "/config/",
    responses={
        200: {
            "description": "Configuration specified as an JSON Schema.",
        },
    },
    tags=["default"],
    summary="Define the configurations for the pupil detection algorithm as an JSON Schema.",
    response_model_by_alias=True,
)
async def get_config_schema() -> Response:
    """Yield the config schema for the pupil detection algorithm."""
    return Response(content=Config.schema_json(indent=2), media_type="application/json")


@router.get(
    "/",
    responses={
        200: {
            "model": MetaData,
            "description": "Meta data regarding the pupil detection algorithm",
        },
    },
    tags=["default"],
    summary="Get the meta data of the detector",
    response_model_by_alias=True,
)
async def get_meta_data() -> MetaData:
    """Yield the meta data for the pupil detection algorithm."""
    return Detector.metadata()
