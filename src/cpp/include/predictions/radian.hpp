#pragma once

#include <cmath>

#include <crow/json.h>

namespace ommatidia {

class Radian {
 public:
  static constexpr float PI = 3.1415927f;

  static inline Radian FromDegree(float degrees) noexcept {
    return Radian(degrees * (Radian::PI / 180.0f));
  }

  inline Radian(float rotation) noexcept
      : radian_(fmod(rotation, 2.0f * Radian::PI)) {}

  Radian(Radian const &) = default;
  Radian &operator=(Radian const &) = default;

  inline operator crow::json::wvalue() const {
    return crow::json::wvalue(static_cast<double>(radian_));
  }

 protected:
  float radian_;
};

}  // namespace ommatidia