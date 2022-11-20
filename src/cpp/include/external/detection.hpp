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
  Detection() : ommatidia::Detection(), method_(T()) {
    static_assert(std::is_base_of<PupilDetectionMethod, T>::value,
                  "Not a valued pupil detection method");
  }

  virtual Result<std::monostate> SetConfig(const JsonInput &config) noexcept {
    return std::monostate();
  }

  inline Result<JsonValue> Predict(cv::InputArray sample) noexcept override {
    auto pupil = method_.runWithConfidence(sample.getMat());

    // There is a mismatch between the definition of "no confidence". Fix that.
    auto confidence = pupil.confidence;
    if (confidence < 0.0) {
      confidence = 0.0;
    }

    return ommatidia::Ellipse(pupil, sample.cols(), sample.rows(), confidence)
        .Serialize();
  }

 protected:
  T method_;
};
}  // namespace external
}  // namespace ommatidia