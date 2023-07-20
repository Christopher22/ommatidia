# coding: utf-8

"""
    Ommatidia

    Unified and simplistic API to evaluate pupil detection algorithms for video-based eye tracking.

    The version of the OpenAPI document: 0.2.0
    Contact: christopher@gundler.de
"""


from fastapi import FastAPI
from fastapi.responses import JSONResponse

from .apis.detections_api import router as DetectionsApiRouter
from .apis.default_api import router as DefaultApiRouter

app = FastAPI(
    title="Ommatidia",
    description="Unified and simplistic API to evaluate pupil detection algorithms for video-based eye tracking.",
    version="0.2.0",
)


@app.exception_handler(Exception)
async def validation_exception_handler(request, err):
    # We need more details
    base_error_message = f"Failed to execute: {request.method}: {request.url}"
    return JSONResponse(
        status_code=500, content={"message": f"{base_error_message}. Detail: {err}"}
    )


app.include_router(DetectionsApiRouter)
app.include_router(DefaultApiRouter)

app.state.detectors = []
