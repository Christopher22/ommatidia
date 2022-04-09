#pragma once

#include "prediction.hpp"

namespace ommatidia {

struct PupilCenter : public Prediction {
  unsigned int x;
  unsigned int y;

  JsonValue Serialize() const noexcept override;
};

}  // namespace ommatidia