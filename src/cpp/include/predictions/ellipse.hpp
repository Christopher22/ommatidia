#pragma once

#include <opencv2/core/types.hpp>

#include "prediction.hpp"

namespace ommatidia {
using Size = float;
using Angle = float;

class Ellipse : public Prediction {
 public:
  Ellipse(Position x, Position y, Size first_size, Size second_size,
          Angle angle, Confidence confidence);
  Ellipse(cv::RotatedRect &rotated_rect, Confidence confidence);
  constexpr Ellipse(Ellipse &&ellipse) noexcept
      : Prediction(ellipse.confidence_),
        x_(ellipse.x_),
        y_(ellipse.y_),
        major_(ellipse.major_),
        minor_(ellipse.minor_),
        angle_(ellipse.angle_) {}

  JsonValue Serialize() const noexcept override;

 protected:
  Position x_, y_;
  Size major_, minor_;
  Angle angle_;

 private:
  static constexpr Angle PI = 3.1415927f;

  constexpr Angle DegreesToRadians(Angle degrees) noexcept {
    return degrees * (Ellipse::PI / 180.0f);
  }
};

}  // namespace ommatidia