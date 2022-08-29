import importlib.resources as pkg_resources

import torch
import numpy as np
from pydantic import BaseModel

from . import AbstractDetector
from .models.meta_data import MetaData
from .models.ellipse import Ellipse
from .implementation.evaluate_ellseg import preprocess_frame, evaluate_ellseg_on_image, rescale_to_original
from .implementation.modelSummary import model_dict
from .implementation import weights

class Config(BaseModel):
    """An empty base model used for config."""

class _Args:
    """ Configuration params """
    def __init__(self):
        self.ellseg_ellipses = 1

class Detector(AbstractDetector):
    def __init__(self, _: Config = Config()):
        netDict = torch.load(pkg_resources.open_binary(weights, "all.git_ok"), map_location=torch.device('cpu'))
        self.model = model_dict['ritnet_v3']
        self.model.load_state_dict(netDict['state_dict'], strict=True)

        # This is used inside the "evaluate_ellseg_on_image" function
        self.args = _Args()

    def detect(self, frame: np.ndarray) -> Ellipse:
        frame_scaled_shifted, scale_shift = preprocess_frame(frame, (240, 320), True)
        input_tensor = frame_scaled_shifted.unsqueeze(0)

        # Run the prediction network. 
        # Despite not used as a paramter the implementation uses a variable "args" in outer scope.
        seg_map, _, pupil_ellipse, iris_ellipse = evaluate_ellseg_on_image(input_tensor, self.model, self.args)

        # Return ellipse predictions back to original dimensions
        seg_map, pupil_ellipse, iris_ellipse = rescale_to_original(seg_map,
                                                                   pupil_ellipse,
                                                                   iris_ellipse,
                                                                   scale_shift,
                                                                   frame.shape)

        return Ellipse(
            x=pupil_ellipse[1],
            y=pupil_ellipse[0],
            major=pupil_ellipse[3],
            minor=pupil_ellipse[2],
            rotation=pupil_ellipse[4]
        )

    @classmethod
    def metadata(cls) -> MetaData:
        return MetaData(
            name="EllSeg: An Ellipse Segmentation Framework for Robust Gaze Tracking",
            additional_information="https://doi.org/10.1109/TVCG.2021.3067765",
            authors=["R. S. Kothari", "A. K. Chaudhary", "R. J. Bailey", "J. B. Pelz" ,"G. J. Diaz"],
            license="MIT",
            prediction="Ellipse",
        )