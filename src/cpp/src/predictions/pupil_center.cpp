#include "../../include/predictions/pupil_center.hpp"

#include <utility>

namespace ommatidia {

JsonValue PupilCenter::Serialize() const noexcept {
  return JsonValue({
      std::make_pair("type", "Point"),
      std::make_pair("x", x_),
      std::make_pair("y", y_),
      std::make_pair("confidence", static_cast<double>(confidence_)),
      std::make_pair("sample", this->GetSampleData()),
  });
}

}  // namespace ommatidia