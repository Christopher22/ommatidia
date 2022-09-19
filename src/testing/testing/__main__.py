import argparse
import logging
from pathlib import Path
import sys

from .docker import Image, InvalidContainerException
from .test_runner import TestRunner


def test_detector(detector_dir: Path, show_output: bool, ignore_cache: bool) -> bool:
    """
    Test a specific Docker container for its suitability as a detector.
    """

    logging.info(
        "Building the image at '%s' (show output: %s)... ",
        str(detector_dir),
        show_output,
    )
    try:
        with Image(detector_dir, ignore_cache=ignore_cache) as image:
            logging.info("Spawning the container '%s' ...", image.name_and_tag)
            with image.spawn(show_output=show_output) as container:
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
                    return True

                # Print output on case of error, if requested  
                output = container.output
                if output is not None:
                    logging.info(output)
                logging.info("Some tests failed")
                return False
    except InvalidContainerException as ex:
        logging.warning(str(ex))
        return False


def test_detectors(detectors_dirs: Path, show_output: bool, ignore_cache: bool) -> bool:
    """
    Test all detectors within a directory.
    """

    all_valid = True
    for entry in detectors_dirs.glob("*/Dockerfile"):
        # Test the detector. However, we do not stop early!
        if not test_detector(
            entry.parent, show_output=show_output, ignore_cache=ignore_cache
        ):
            all_valid = False
        logging.info("Testing detector done\n")

    return all_valid


def parse_arguments() -> int:
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
    parser.add_argument(
        "-a",
        "--test_all",
        action="store_true",
        help="check all detectors within a folder",
    )
    parser.add_argument(
        "-c",
        "--ignore_cache",
        action="store_true",
        help="ignore existing caches",
    )

    args = parser.parse_args()

    all_tests_valid = (
        test_detectors(
            Path(args.detector), show_output=args.output, ignore_cache=args.ignore_cache
        )
        if args.test_all
        else test_detector(
            Path(args.detector), show_output=args.output, ignore_cache=args.ignore_cache
        )
    )
    return 0 if all_tests_valid else 1


if __name__ == "__main__":
    sys.exit(parse_arguments())
