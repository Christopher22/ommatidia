from .models.meta_data import MetaData


class AbstractDetector:
    """The abstract base for all detectors"""

    @classmethod
    def metadata(cls) -> MetaData:
        """Yield the meta data of the detector."""
        raise NotImplementedError("The pupil detector must override the metadata")
