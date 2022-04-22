#include "../../include/predictions/ellipse.hpp"

#include <utility>

namespace ommatidia {

Ellipse::Ellipse(Position x, Position y, Size first_size, Size second_size,
                 Radian radian, Confidence confidence)
    : Prediction(confidence),
      x_(x),
      y_(y),
      major_(std::max(first_size, second_size)),
      minor_(std::min(first_size, second_size)),
      angle_(radian) {}

Ellipse::Ellipse(const cv::RotatedRect &rotated_rect, Confidence confidence)
    : Ellipse(rotated_rect.center.x, rotated_rect.center.y,
              rotated_rect.size.width, rotated_rect.size.height,
              Radian::FromDegree(rotated_rect.angle), confidence) {}

JsonValue Ellipse::Serialize() const noexcept {
  return JsonValue({
      std::make_pair("x", x_),
      std::make_pair("y", y_),
      std::make_pair("major", static_cast<double>(major_)),
      std::make_pair("minor", static_cast<double>(minor_)),
      std::make_pair("angle", angle_),
      std::make_pair("confidence", static_cast<double>(confidence_)),
  });
}

}  // namespace ommatidia