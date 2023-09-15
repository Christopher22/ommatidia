# Re-usable source code for different detectors
This folder contains source code required by different detectors and is used to avoid duplicates. It is ordered according to the programming languages of the detectors.

## Usage
All the pupil detectors require base images to avoid extensive disk usage. You may run `python base_builder.py ../detectors` to create those automatically. Afterwards, you may i.e. use the scripts within the `testing` folder.