#pragma once

#include <crow/json.h>

#include "error.hpp"

namespace ommatidia {

class DetectionParams {
 public:
  static Result<DetectionParams> Parse(const crow::json::rvalue& request);

 protected:
  inline DetectionParams(unsigned int width, unsigned int height) noexcept
      : width_(width), height_(height) {}

  unsigned int width_;
  unsigned int height_;
};

}  // namespace ommatidia