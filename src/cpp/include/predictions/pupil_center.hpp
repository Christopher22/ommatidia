#pragma once

#include "prediction.hpp"

namespace ommatidia {

class PupilCenter : public Prediction {
 public:
  constexpr PupilCenter(Position x, Position y, Position width, Position height,
                        Confidence confidence) noexcept
      : Prediction(width, height, confidence), x_(x), y_(y) {}
  JsonValue Serialize() const noexcept override;

 protected:
  Position x_, y_;
};

}  // namespace ommatidia