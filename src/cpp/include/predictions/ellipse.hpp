#pragma once

#include <opencv2/core/types.hpp>

#include "prediction.hpp"
#include "radian.hpp"

namespace ommatidia {
using Size = float;

class Ellipse : public Prediction {
 public:
  Ellipse(Position x, Position y, Size first_size, Size second_size,
          Radian rotation, Position width, Position height,
          Confidence confidence);
  Ellipse(const cv::RotatedRect &rotated_rect, Position width, Position height,
          Confidence confidence);
  constexpr Ellipse(Ellipse &&ellipse) noexcept
      : Prediction(ellipse.width_, ellipse.height_, ellipse.confidence_),
        x_(ellipse.x_),
        y_(ellipse.y_),
        major_(ellipse.major_),
        minor_(ellipse.minor_),
        angle_(ellipse.angle_) {}

  JsonValue Serialize() const noexcept override;

 protected:
  Position x_, y_;
  Size major_, minor_;
  Radian angle_;
};

}  // namespace ommatidia