#pragma once

#include <optional>

#include <opencv2/core.hpp>

#include "predictions/prediction.hpp"
#include "error.hpp"

namespace ommatidia {

class Detection {
 public:
  virtual ~Detection() = default;
  Detection(Detection const &) = delete;
  Detection &operator=(Detection const &) = delete;

  /// Analye the given sample and store the prediction.
  virtual std::optional<Error> Predict(cv::InputArray sample,
                                       float timestamp) noexcept = 0;

  /// Get the number of stored predictions.
  virtual std::size_t GetNumPredictions() const noexcept = 0;

  /// Get the prediction at a specific sample.
  virtual std::optional<JsonValue> GetPrediction(
      std::size_t index) const noexcept = 0;
};

}  // namespace ommatidia