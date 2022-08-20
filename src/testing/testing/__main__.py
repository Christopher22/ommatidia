import argparse
import logging
from pathlib import Path

from .docker import Image


def main():
    """
    The main routine for the package.
    """

    logging.basicConfig(format="[%(levelname)s] %(message)s", level=logging.INFO)

    parser = argparse.ArgumentParser(
        description="Integration tests for pupil detectors"
    )
    parser.add_argument(
        "detector",
        type=str,
        help="the directory containing the pupil detector of interest",
    )
    args = parser.parse_args()

    logging.info("Building the image ...")
    with Image(Path(args.detector)) as image:
        logging.info("Spawning the container ...")
        with image.spawn() as container:
            while not container.is_ready(3):
                logging.info("Container not ready")
            logging.info("Detector online!")
        logging.info("Container shut down")


if __name__ == "__main__":
    main()
