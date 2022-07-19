# coding: utf-8

"""
    Ommatidia

    Unified and simplistic API to evaluate pupil detection algorithms for video-based eye tracking.

    The version of the OpenAPI document: 0.2.0
    Contact: christopher@gundler.de
"""


from fastapi import FastAPI

from .apis.detections_api import router as DetectionsApiRouter
from .apis.default_api import router as DefaultApiRouter

app = FastAPI(
    title="Ommatidia",
    description="Unified and simplistic API to evaluate pupil detection algorithms for video-based eye tracking.",
    version="0.2.0",
)

app.include_router(DetectionsApiRouter)
app.include_router(DefaultApiRouter)

app.state.detectors = []
