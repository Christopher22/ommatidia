#pragma once

#include <opencv2/core.hpp>

#include <external/pupil.hpp>
#include <external/pupil_detection_method.hpp>

class ExampleDetection : public PupilDetectionMethod {
 public:
  ExampleDetection() {
    mDesc = "Example detection";
    mTitle = "Example detection";
  }

  Pupil run(const cv::Mat &frame) override {
    return Pupil(cv::RotatedRect(cv::Point2f(frame.rows, frame.cols),
                                 cv::Point2f(frame.rows, frame.cols), 42.0),
                 0.22);
  }

  bool hasConfidence() override { return false; }
  bool hasCoarseLocation() override { return false; }
  bool hasInliers() override { return false; }
};