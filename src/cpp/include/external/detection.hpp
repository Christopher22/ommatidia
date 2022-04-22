#pragma once

#include <memory>
#include <vector>

#include "../detection.hpp"
#include "../predictions/ellipse.hpp"
#include "pupil_detection_method.hpp"
#include "pupil.hpp"

namespace ommatidia {
namespace external {

template <typename T>
class Detection : public ommatidia::Detection {
 public:
  Detection() : ommatidia::Detection(), method_(T()), ellipses_() {
    static_assert(std::is_base_of<PupilDetectionMethod, T>::value,
                  "Not a valued pupil detection method");
  }

  inline Result<PredictionIndex> Predict(cv::InputArray sample, float timestamp) noexcept override {
    auto pupil = method_.runWithConfidence(sample.getMat());
    const auto confidence = pupil.confidence;
    const auto index = ellipses_.size();
    ellipses_.emplace_back(pupil, confidence);
    return index;
  }

  /// Get the number of stored predictions.
  inline PredictionIndex GetNumPredictions() const noexcept override {
    return ellipses_.size();
  }

  /// Get the prediction at a specific sample.
  inline std::optional<JsonValue> GetPrediction(
      PredictionIndex index) const noexcept override {
    if (index < ellipses_.size()) {
      return ellipses_[index].Serialize();
    } else {
      return std::nullopt;
    }
  }

 protected:
  T method_;
  std::vector<Ellipse> ellipses_;
};
}  // namespace external
}  // namespace ommatidia