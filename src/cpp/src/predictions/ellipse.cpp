#include "../../include/predictions/ellipse.hpp"

#include <utility>

namespace ommatidia {

Ellipse::Ellipse(Position x, Position y, Size first_size, Size second_size,
                 Angle angle, Confidence confidence)
    : Prediction(confidence),
      x_(x),
      y_(y),
      major_(std::max(first_size, second_size)),
      minor_(std::min(first_size, second_size)),
      angle_(fmod(angle, 2.0f * Ellipse::PI)) {}

Ellipse::Ellipse(cv::RotatedRect &rotated_rect, Confidence confidence)
    : Ellipse(rotated_rect.center.x, rotated_rect.center.y,
              rotated_rect.boundingRect().width,
              rotated_rect.boundingRect().height,
              Ellipse::DegreesToRadians(rotated_rect.angle), confidence) {}

JsonValue Ellipse::Serialize() const noexcept {
  return JsonValue({
      std::make_pair("x", JsonValue(x_)),
      std::make_pair("y", JsonValue(y_)),
      std::make_pair("major", JsonValue(major_)),
      std::make_pair("minor", JsonValue(minor_)),
      std::make_pair("angle", JsonValue(angle_)),
      std::make_pair("confidence", JsonValue(confidence_)),
  });
}

}  // namespace ommatidia