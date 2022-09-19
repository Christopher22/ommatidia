#pragma once

#include <crow/json.h>

#include "../types.hpp"

namespace ommatidia {
class Prediction {
 public:
  constexpr Prediction(Position width, Position height,
                       Confidence confidence) noexcept
      : confidence_(confidence), width_(width), height_(height) {}
  Prediction(Prediction const &) = delete;
  Prediction &operator=(Prediction const &) = delete;

  virtual ~Prediction() noexcept = default;
  virtual JsonValue Serialize() const noexcept = 0;

  inline Confidence GetConfidence() const noexcept { return this->confidence_; }
  inline bool operator!() const noexcept { return this->confidence_ >= 0.0; }

 protected:
  inline ommatidia::JsonValue GetSampleData() const {
    const std::unordered_map<std::string, ommatidia::JsonValue> sample = {
        {"width", ommatidia::JsonValue(width_)},
        {"height", ommatidia::JsonValue(height_)}};
    return ommatidia::JsonValue(sample);
  }

  Confidence confidence_;
  Position width_, height_;
};
}  // namespace ommatidia