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

  /// Get the current config of the algorithm.
  virtual JsonValue GetConfig() noexcept { return JsonValue(); }

  /// Analye the given sample and store the prediction.
  virtual Result<JsonValue> Predict(cv::InputArray sample) noexcept = 0;

 protected:
  constexpr Detection() noexcept {}
};

}  // namespace ommatidia