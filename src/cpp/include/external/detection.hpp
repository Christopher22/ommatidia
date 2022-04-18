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

  inline std::optional<Error> Predict(cv::InputArray sample,
                                      float timestamp) noexcept override {
    auto pupil = method_.runWithConfidence(sample.getMat());
    auto confidence = pupil.confidence;
    ellipses_.emplace_back(pupil, confidence);
    return std::nullopt;
  }

  /// Get the number of stored predictions.
  inline std::size_t GetNumPredictions() const noexcept override {
    return ellipses_.size();
  }

  /// Get the prediction at a specific sample.
  inline std::optional<JsonValue> GetPrediction(
      std::size_t index) const noexcept override {
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