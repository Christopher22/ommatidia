import argparse
import logging
from pathlib import Path
from dataclasses import dataclass, field, InitVar
from typing import Sequence, Tuple, Optional, List
import csv
import sys

from .docker import Image, Container, InvalidContainerException
from .test_runner import TestRunner


@dataclass
class DetectorHandler:
    """
    A base class for spawning detectors.
    """

    detector_dir: Path
    show_output: bool
    ignore_cache: bool

    def run(self) -> bool:
        """
        Run a single detector.
        """
        logging.info(
            "Building the image at '%s' (show output: %s)... ",
            str(self.detector_dir),
            self.show_output,
        )
        try:
            with Image(self.detector_dir, ignore_cache=self.ignore_cache) as image:
                logging.info("Spawning the container '%s' ...", image.name_and_tag)
                with image.spawn(show_output=self.show_output) as container:
                    # Start the detector
                    while not container.is_ready(3):
                        logging.info("Waiting for detector not get ready ...")
                    logging.info("Detector sucessfully started")

                    return self._run_detector(image, container)
        except InvalidContainerException as ex:
            logging.warning(str(ex))
            return False

    def run_all(self) -> bool:
        """
        Run all detectors within a directory.
        """

        all_valid = True
        for entry in self.detector_dir.glob("*/Dockerfile"):
            # Run the detector. However, we do not stop early on error!
            self.detector_dir = entry.parent
            if not self.run():
                all_valid = False
            logging.info("Testing detector done\n")

        return all_valid

    def _run_detector(self, _: Image, _c: Container) -> bool:
        raise NotImplementedError("Not implemented by subclass")


class TestDetector(DetectorHandler):
    """
    Evaluate unit tests on the detector.
    """

    def _run_detector(self, _: Image, container: Container) -> bool:
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


@dataclass
class EvaluateDetector(DetectorHandler):
    """
    Evaluate the detector on images.
    """

    image_folder: InitVar[Path]
    _files: Sequence[Path] = field(init=False)
    _results: List[
        Tuple[str, str, Optional[float], Optional[float], Optional[float]]
    ] = field(init=False)

    def __post_init__(self, image_folder: Path):
        self._files = list(image_folder.glob("*.png"))
        self._results = []

    def _run_detector(self, image: Image, container: Container) -> bool:
        # Create the detector
        creation_response = container.request("/detections/", method="POST", body={})
        if creation_response.status != 200:
            logging.warning("Unable to create detector")
            return False
        detector_path = f"/detections/{creation_response.body}/"

        for image_path in self._files:
            with image_path.open("rb") as image_file:
                raw_result = container.request(
                    detector_path,
                    method="POST",
                    body=image_file.read(),
                    content_type="image/png",
                )
                if raw_result.status != 200:
                    logging.warning(
                        "Detector '%s' failed on '%s'",
                        image.name_and_tag,
                        image_path.name,
                    )
                    continue

                result = raw_result.json
                self._results.append(
                    (
                        image.name_and_tag,
                        image_path.name,
                        result["x"],
                        result["y"],
                        result["confidence"] if "confidence" in result else None,
                    )
                )

        return True

    def export(self, file_path: Path) -> None:
        """
        Export generated annotations as files.
        """
        with file_path.open("w", newline="") as file:
            writer = csv.writer(
                file, delimiter="\t", quotechar='"', quoting=csv.QUOTE_MINIMAL
            )
            writer.writerow(("Detector", "File", "X", "Y", "Confidence"))
            writer.writerows(self._results)


def parse_arguments() -> int:
    """
    The main routine for the package.
    """

    MODE_UNIT_TEST = "unit"
    MODE_EVALUATION = "eval"

    logging.basicConfig(format="[%(levelname)s] %(message)s", level=logging.INFO)

    parser = argparse.ArgumentParser(
        description="Integration tests for pupil detectors"
    )
    parser.add_argument(
        "directory",
        type=str,
        help="the directory containing the pupil detector(s) of interest",
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

    commands = parser.add_subparsers(dest="subcommand")
    commands.required = True

    commands.add_parser(
        MODE_UNIT_TEST, help="Run the unit tests on selected detector(s)"
    )
    evaluation_parser = commands.add_parser(
        MODE_EVALUATION, help="Evaluate the parsers on images for testing"
    )
    evaluation_parser.add_argument(
        "image_directory",
        type=str,
        help="the directory containing the PNG files of interest",
    )
    evaluation_parser.add_argument(
        "output_file",
        type=str,
        help="the location the output TSV is written to",
    )

    args = parser.parse_args()
    if args.subcommand == MODE_UNIT_TEST:
        tests = TestDetector(
            Path(args.directory),
            show_output=args.output,
            ignore_cache=args.ignore_cache,
        )
        all_tests_valid = tests.run_all() if args.test_all else tests.run()
        return 0 if all_tests_valid else 1

    evaluation = EvaluateDetector(
        Path(args.directory),
        show_output=args.output,
        ignore_cache=args.ignore_cache,
        image_folder=Path(args.image_directory),
    )

    if args.test_all:
        evaluation.run_all()
    else:
        evaluation.run()
    evaluation.export(Path(args.output_file))
    return 0


if __name__ == "__main__":
    sys.exit(parse_arguments())
