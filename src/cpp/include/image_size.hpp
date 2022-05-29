#pragma once

#include <string_view>

#include <crow/json.h>

#include "error.hpp"
#include "types.hpp"

namespace ommatidia {

class ImageSize {
 public:
  static Result<ImageSize> Parse(const crow::json::rvalue& request);
  static Result<ImageSize> Parse(std::string_view request);

  constexpr ImageSize(Position width, Position height) noexcept
      : width_(width), height_(height) {}

  constexpr Position Width() const { return width_; }
  constexpr Position Height() const { return height_; }

 protected:
  Position width_;
  Position height_;
};

}  // namespace ommatidia