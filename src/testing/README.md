# Testing module
Given the standartized interface, all the pupil detection algorithm can be automatically tested for expected behavior. This small Python tool allows the conduction of such tests including the process of building and managing the Docker images and containers.

## Usage
The module does not depend on any external packages. Consequently, Python >= 3.7 is sufficient for running the tests.

### Executing the detectors on images
While the Rust code may be more appropiated, you can use this code tp generate predictions for multiple detectors on multiple packages automatically, too. The call may look like `python -m testing -a ..\..\detectors\ eval folder_with_nested_image_files \results.tsv`

## Utilized resources and corresponding license
The example image used for testing purposes is taken from "Robust real-time pupil tracking in highly off-axis images" by Lech Świrski,Andreas Bulling, and Neil A. Dodgson (https://www.cl.cam.ac.uk/research/rainbow/projects/pupiltracking/datasets/).
