#pragma once

#include "prediction.hpp"

namespace ommatidia {

class PupilCenter : public Prediction {
 public:
  constexpr PupilCenter(Position x, Position y, Confidence confidence) noexcept
      : Prediction(confidence), x_(x), y_(y) {}
  JsonValue Serialize() const noexcept override;

 protected:
  Position x_, y_;
};

}  // namespace ommatidia