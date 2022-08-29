import argparse
import logging
from pathlib import Path

from .docker import Image, InvalidContainerException
from .test_runner import TestRunner


def main() -> int:
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
    parser.add_argument(
        "-o",
        "--output",
        action="store_true",
        help="show the output of the building process",
    )
    args = parser.parse_args()

    logging.info(
        "Building the image at '%s' (show output: %s)... ", args.detector, args.output
    )
    try:
        with Image(Path(args.detector), show_output=args.output) as image:
            logging.info("Spawning the container '%s' ...", image.name_and_tag)
            with image.spawn(show_output=args.output) as container:
                # Start the detector
                while not container.is_ready(3):
                    logging.info("Waiting for detector not get ready ...")
                logging.info("Detector sucessfully started")

                # Run all the tests
                test_runner = TestRunner(container)
                logging.info("Found %d tests for the detector", len(test_runner))
                for result in test_runner:
                    if not result:
                        logging.warning(str(result))

                if test_runner:
                    logging.info("All tests done without any error")
                    return 0
                logging.info("Some tests failed")
                return 1
    except InvalidContainerException as ex:
        logging.warning(str(ex))
        return 1


if __name__ == "__main__":
    status = main()
    # exit(status)
