#pragma once

#include <crow/json.h>

namespace ommatidia {
using JsonValue = crow::json::wvalue;

class Prediction {
 public:
  Prediction(float confidence) noexcept : confidence_(confidence) {}
  Prediction(Prediction const &) = delete;
  Prediction &operator=(Prediction const &) = delete;

  virtual ~Prediction() noexcept = default;
  virtual JsonValue Serialize() const noexcept = 0;

  inline float GetConfidence() const noexcept { return this->confidence_; }
  inline bool operator!() const noexcept { return this->confidence_ >= 0.0; }

 protected:
  float confidence_;
};
}  // namespace ommatidia