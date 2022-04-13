#include "../include/detection_params.hpp"

namespace ommatidia {

Result<DetectionParams> DetectionParams::Parse(std::string_view request) {
  auto json_body = crow::json::load(request.data(), request.size());
  if (json_body.error()) {
    return Error("Malformed JSON in body", crow::BAD_REQUEST);
  }
  return Parse(json_body);
}

Result<DetectionParams> DetectionParams::Parse(
    const crow::json::rvalue& request) {
  static const char* WIDTH = "width";
  static const char* HEIGTH = "height";

  if (request.t() != crow::json::type::Object) {
    return Error("The given payload is no object", crow::BAD_REQUEST);
  } else if (!request.has(WIDTH) || !request.has(HEIGTH)) {
    return Error("Image dimensions missing", crow::BAD_REQUEST);
  }

  const auto& width_json = request[WIDTH];
  const auto& height_json = request[HEIGTH];
  if (width_json.t() != crow::json::type::Number ||
      height_json.t() != crow::json::type::Number) {
    return Error("Image dimensions invalid", crow::BAD_REQUEST);
  }

  int64_t width = width_json.i();
  int64_t height = height_json.i();
  if (width <= 0 || height <= 0) {
    return Error("Image dimensions invalid", crow::BAD_REQUEST);
  }

  return Result<DetectionParams>(
      DetectionParams((unsigned int)width, (unsigned int)height));
}

}  // namespace ommatidia