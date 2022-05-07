#pragma once

#include "../server.hpp"
#include "detection.hpp"

namespace ommatidia {
namespace external {

template <typename T>
class Server : public ommatidia::Server {
 public:
  Server(MetaData &&meta_data) : ommatidia::Server(std::move(meta_data)) {}

 protected:
  inline Result<std::unique_ptr<ommatidia::Detection>> CreateDetection(
      const JsonInput &configuration) noexcept override {
    std::unique_ptr<Detection<T>> detector = std::make_unique<Detection<T>>();

    // Try to set the configuration
    auto configuration_result = detector->SetConfig(configuration);
    if (std::holds_alternative<Error>(configuration_result)) {
      return std::get<Error>(std::move(configuration_result));
    }

    return detector;
  }
};

}  // namespace external
}  // namespace ommatidia