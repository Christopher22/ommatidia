# coding: utf-8

from typing import List, Union, Optional

from fastapi import (
    APIRouter,
    HTTPException,
    Path,
    Request,
)
from fastapi.responses import JSONResponse
import cv2 as cv
import numpy as np

from ..models.point import Point
from ..models.ellipse import Ellipse
from ..models.mask import Mask
from ..models.sample import Sample
from ..detector import Detector, Config

router = APIRouter()


@router.post(
    "/detections/",
    responses={
        200: {"description": "Instance successfully created."},
        400: {"model": str, "description": "The given configuration is invalid."},
    },
    tags=["Detections"],
    summary="Initialize a new pupil detection algorithm with specific configuration.",
    response_model_by_alias=True,
)
async def create_detector(request: Request, config: Optional[Config] = None) -> int:
    """
    Initialize a new pupil detection algorithm with specific configuration.
    """
    detectors = request.app.state.detectors
    detector_id = len(detectors)
    detectors.append(Detector(config if config is not None else Config()))
    return detector_id


@router.get(
    "/detections/{detector_id}/",
    responses={
        200: {"description": "The specified configuration on creation."},
        404: {"model": str, "description": "The selected ID was not found"},
    },
    tags=["Detections"],
    summary="Query an pupil detection algorithm and its configuration.",
    response_model_by_alias=True,
)
async def get_detector(
    request: Request,
    detector_id: int = Path(
        None,
        description="Identifier for the running instance of pupil detection algorithm.",
        ge=0,
    ),
) -> object:
    """
    Query an pupil detection algorithm and its configuration.
    """
    detectors = request.app.state.detectors
    if detector_id >= len(detectors) or detectors[detector_id] is None:
        return JSONResponse(content="The selected ID was not found", status_code=404)
    return {}


@router.delete(
    "/detections/{detector_id}/",
    responses={
        200: {"description": "The detection was successfully deleted."},
        404: {"model": str, "description": "The selected ID was not found"},
    },
    tags=["Detections"],
    summary="Delete the instance of the pupil detection algorithm and remove all associated resources.",
    response_model_by_alias=True,
)
async def delete_detector(
    request: Request,
    detector_id: int = Path(
        None,
        description="Identifier for the running instance of pupil detection algorithm.",
        ge=0,
    ),
) -> None:
    """
    Delete the instance of the pupil detection algorithm and remove all associated resources.
    """
    detectors = request.app.state.detectors
    if detector_id >= len(detectors) or detectors[detector_id] is None:
        raise HTTPException(status_code=404, detail="The selected ID was not found")

    detectors[detector_id] = None


@router.post(
    "/detections/{detector_id}/",
    responses={
        200: {"description": "The estimated pupil center."},
        404: {"description": "The selected ID was not found"},
    },
    tags=["Detections"],
    summary="Evaluate a given image with the configured pupil detection algorihm.",
    response_model_by_alias=True,
)
async def detect(
    request: Request,
    detector_id: int = Path(
        None,
        description="Identifier for the running instance of pupil detection algorithm.",
        ge=0,
    ),
) -> Union[Point, Ellipse, Mask]:
    """
    Evaluate a given image with the configured pupil detection algorihm.
    """
    detectors = request.app.state.detectors
    detector = None if detector_id >= len(detectors) else detectors[detector_id]
    if detector is None:
        raise HTTPException(status_code=404, detail="The selected ID was not found")

    # Try to decode the image from bytes
    image_data: bytes = await request.body()
    image = cv.imdecode(np.frombuffer(image_data, dtype=np.uint8), cv.IMREAD_GRAYSCALE)
    if image is None:
        raise HTTPException(status_code=400, detail="The provided sample is not valid")

    # Run the detector
    prediction = detector.detect(image)

    # Enrich the result with the information regarding the sample
    prediction.sample = Sample(width=image.shape[1], height=image.shape[0])
    return prediction

@router.get(
    "/detections/",
    responses={
        200: {
            "model": List[int],
            "description": 'A list of "detectionId"s for running pupil detection algorithms.',
        },
    },
    tags=["Detections"],
    summary="Returns a list of running pupil detection algorithms.",
    response_model_by_alias=True,
)
async def get_detectors(
    request: Request,
) -> List[int]:
    """
    Returns a list of running pupil detection algorithms.
    """
    return [
        i
        for i, detector in enumerate(request.app.state.detectors)
        if detector is not None
    ]
