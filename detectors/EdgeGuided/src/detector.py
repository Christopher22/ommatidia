from enum import Enum
from pathlib import Path

import math
import numpy as np
import torch
from pydantic import BaseModel


from . import AbstractDetector
from .models.ellipse import Ellipse
from .models.meta_data import MetaData
from .implementation.evaluate import (
    BDCN,
    DenseNet2D,
    DeepVOG_pytorch,
    preprocess_frame,
    evaluate_ellseg_on_image,
    rescale_to_original,
    get_config,
)


class DetectorType(str, Enum):
    RIT_NET = "ritnet_v2"
    DEEPVOG = "deepvog"


class Config(BaseModel):
    neural_network: DetectorType = DetectorType.RIT_NET


class _DetectionArguments:
    """
    Detection arguments required by the original information.
    """

    def __init__(self) -> None:
        self.prec = torch.device("cpu")


class Detector(AbstractDetector):
    def __init__(self, config: Config = Config()) -> None:
        if torch.cuda.is_available():
            print("Using CUDA for detection")
            self.device = torch.device("cuda")
        else:
            print("Using CPU for detection")
            self.device = torch.device("cpu")

        state_dict = torch.load(
            Path(__file__).parent / "implementation/gen_00000016.pt",
            map_location=self.device,
        )

        self.bdcn = BDCN()
        self.bdcn.load_state_dict(state_dict["a"])

        if config.neural_network.value == DetectorType.RIT_NET.value:
            setting = get_config(
                Path(__file__).parent / "implementation/configs/baseline_edge.yaml"
            )
            self.model = DenseNet2D(setting)
        else:
            self.model = DeepVOG_pytorch()

        netDict = torch.load(
            Path(__file__).parent / "implementation/baseline_edge_16.pkl",
            map_location=self.device,
        )
        self.model.load_state_dict(netDict["state_dict"])

        # Ensure to place the networks on the suitable device
        self.model = self.model.to(device=self.device)
        self.bdcn = self.bdcn.to(device=self.device)

        # Strange, but well ...
        self.args = _DetectionArguments()
        self.args.prec = self.device

    def detect(self, frame: np.ndarray) -> Ellipse:
        frame_scaled_shifted, scale_shift = preprocess_frame(frame, (240, 320), True)
        input_tensor = frame_scaled_shifted.to(self.device).unsqueeze(0)

        # Run the prediction network
        edge_map, seg_map, pupil_ellipse, iris_ellipse = evaluate_ellseg_on_image(
            input_tensor,
            self.model,
            self.bdcn,
            args=self.args,
            device=self.device,
        )
        edge_map *= 255
        edge_map = 255 - edge_map
        _, _, pupil_ellipse, _ = rescale_to_original(
            edge_map, seg_map, pupil_ellipse, iris_ellipse, scale_shift, frame.shape
        )

        return Ellipse(
            type="Ellipse",
            x=pupil_ellipse[0],
            y=pupil_ellipse[1],
            major=pupil_ellipse[2],
            minor=pupil_ellipse[3],
            rotation=pupil_ellipse[4] + math.pi,
        )

    @classmethod
    def metadata(cls) -> MetaData:
        return MetaData(
            name="Edge-Guided Near-Eye Image Analysis for Head Mounted Displays",
            additional_information="http://doi.org/10.1109/ISMAR52148.2021.00015",
            authors=["Z. Wang", "Y. Zhao", "Y. Liu", "F. Lu"],
            license="Custom",
            prediction="Point",
        )
