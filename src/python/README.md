# Central components for pupil detectors based upon Python
This repository contains all the boilerplate code for creating a pupil detector based upon Python including tests and usable defaults. Custom images could use this image as the base for their own implementation while relying on the server functionality and an API accordingly to the definition.

## Usage
Inherit from the base package. Copy a "detector.py", an with a subclass of AbstractDetector and a class Config, into the PUPIL_DETECTOR_DIR.