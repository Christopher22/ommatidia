#pragma once

#include "prediction.hpp"

namespace ommatidia {

class Ellipse : public Prediction {
 public:
  JsonValue Serialize() const noexcept override;
};

}  // namespace ommatidia