# Ommatidia: A modular, platform-independent and scalable platform for the evaluation of pupil detection algorithms.

## Overview
This repository contains the source code associated with our research publication titled "A Software Framework for Evaluating and Comparing Pupil Detection Algorithms". The toolkit developed enables researchers across various disciplines to assess the performance of multiple pupil detection algorithms, facilitating the selection of the most suitable algorithm for specific experimental setups. This software is designed to be user-friendly, flexible, and adaptable to different hardware and software environments.

## Features
- Comparison of Multiple Algorithms: Supports the evaluation of 13 different pupil detection algorithms through a unified interface.
- Hardware and Software Flexibility: Designed to function on a wide range of systems without the need for online connectivity or specific commercial software.
- User-Friendly: Easy setup and operation, suitable for researchers without deep technical knowledge of pupil detection algorithms.
- Scalable and Modular Architecture: Utilizes microservices to ensure scalability and modularity, allowing for efficient use of computational resources.

## Installation
In general, only Docker and a (standard) python installation are required to run the toolkit. Please download the repository and use your terminal to navigate to the folder "src" and follow the instructions in teh corresponding README there.

## Usage
After buidling and starting the services, the toolkit is operational. Researchers can send eye-tracking data to the toolkit's REST API, which then processes the data using the selected algorithms. The definition of the API could be found in the openapi.yaml. The toolkit provides endpoints for submitting eye-tracking samples and retrieving detection results.

### Example
For replicating the values presented in the paper, consider reading the README under "src/testing".

## Contributing
Contributions are welcome! Please create a pull request with your proposed changes. For major changes, open an issue first to discuss what you would like to change.
Ensure to update tests as appropriate and adhere to the existing coding style.

## Citation
If you use this toolkit for your research, please cite the corresponding paper:

**Gundler et al., Improving eye-tracking data quality: A framework for reproducible evaluation of detection algorithm (2024)**
