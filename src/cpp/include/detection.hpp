#pragma once

#include <optional>

#include <opencv2/core.hpp>

#include "predictions/prediction.hpp"
#include "error.hpp"

namespace ommatidia {

using PredictionIndex = std::size_t;

class Detection {
 public:
  virtual ~Detection() = default;
  Detection(Detection const &) = delete;
  Detection &operator=(Detection const &) = delete;

  /// Analye the given sample and store the prediction.
  virtual Result<PredictionIndex> Predict(cv::InputArray sample, float timestamp) noexcept = 0;

  /// Get the number of stored predictions.
  virtual PredictionIndex GetNumPredictions() const noexcept = 0;

  /// Get the prediction at a specific sample.
  virtual std::optional<JsonValue> GetPrediction(
      PredictionIndex index) const noexcept = 0;

 protected:
  constexpr Detection() noexcept {}
};

}  // namespace ommatidia