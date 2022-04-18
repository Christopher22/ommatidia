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
      DetectionParams parameters) noexcept override {
    std::unique_ptr<Detection<T>> detector = std::make_unique<Detection<T>>();
    return detector;
  }
};

}  // namespace external
}  // namespace ommatidia