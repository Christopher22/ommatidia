#pragma once

#include <crow/json.h>

namespace ommatidia {
using JsonValue = crow::json::wvalue;
using Position = unsigned int;
using Confidence = float;

class Prediction {
 public:
  constexpr Prediction(Confidence confidence) noexcept : confidence_(confidence) {}
  Prediction(Prediction const &) = delete;
  Prediction &operator=(Prediction const &) = delete;

  virtual ~Prediction() noexcept = default;
  virtual JsonValue Serialize() const noexcept = 0;

  inline Confidence GetConfidence() const noexcept { return this->confidence_; }
  inline bool operator!() const noexcept { return this->confidence_ >= 0.0; }

 protected:
  Confidence confidence_;
};
}  // namespace ommatidia