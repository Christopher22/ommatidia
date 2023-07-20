from pathlib import Path

import torch
import numpy as np
import cv2
from pydantic import BaseModel

from . import AbstractDetector
from .models.point import Point
from .models.meta_data import MetaData
from .implementation.unet import MyUNet


class Config(BaseModel):
    """An empty base model used for config."""


class Detector(AbstractDetector):
    def __init__(self, _: Config = Config()):
        model_file_name = (
            Path(__file__).parent / "implementation/efe-Unet-trained-model-640x480.pt"
        )  # This is the model for ellipse fit error

        self.model = MyUNet(32)
        self.model.load_state_dict(
            torch.load(model_file_name, map_location=torch.device("cpu"))
        )
        self.model.eval()

    def detect(self, frame_raw: np.ndarray) -> np.ndarray:
        if len(frame_raw.shape) != 2:
            raise ValueError("Expecting grayscale image")

        SIZE_X, SIZE_Y = 640, 480
        frame = cv2.resize(frame_raw, (SIZE_X, SIZE_Y), interpolation=cv2.INTER_CUBIC)
        frame = frame[np.newaxis, np.newaxis, :]
        frame = torch.from_numpy(frame.astype(np.float32) / 255)

        output = self.model(frame)
        output_bk = output[:, 0].clone().detach().cpu().numpy()
        ttt = output_bk
        ttt[ttt < 0.5] = 0
        ttt[ttt >= 0.5] = 1
        if np.count_nonzero(ttt) == 0:
            output_bk[output_bk < 0.25] = 0
            output_bk[output_bk >= 0.25] = 1
        else:
            output_bk[output_bk < 0.5] = 0
            output_bk[output_bk >= 0.5] = 1

        ## Connected Component Analysis
        if np.count_nonzero(output_bk) != 0:
            _, _, stats, center = cv2.connectedComponentsWithStats(
                output_bk[0, :, :].astype(np.uint8)
            )

            stats = stats[1:, :]
            pupil_candidate = np.argmax(stats[:, 4]) + 1

            x = float(center[pupil_candidate][0])
            y = float(center[pupil_candidate][1])

            # This scales is missing in the reference implementation, but appears necessary
            x *= frame_raw.shape[1] / SIZE_X
            y *= frame_raw.shape[0] / SIZE_Y

            return Point(x=x, y=y)
        else:
            return Point(x=0.0, y=0.0, confidence=-1.0)

    @classmethod
    def metadata(cls) -> MetaData:
        return MetaData(
            name="Accurate CNN-based Pupil Segmentation with an Ellipse Fit Error Regularization Term",
            additional_information="https://doi.org/10.1016/j.eswa.2021.116004",
            authors=["C. Akinlar", "H. K. Kucukkartal", "C. Topal"],
            license="Custom",
            prediction="Point",
        )
