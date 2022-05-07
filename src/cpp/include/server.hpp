#pragma once

#include <unordered_map>
#include <string>
#include <memory>
#include <functional>

#include <crow/app.h>

#include "detection.hpp"
#include "meta_data.hpp"
#include "error.hpp"
#include "types.hpp"

namespace ommatidia {

class Server {
 public:
  virtual ~Server() = default;
  Server(Server const &) = delete;
  Server &operator=(Server const &) = delete;

  void Run(uint16_t port);
  crow::response Run(crow::request request);
  crow::response Run(std::function<void(crow::request &)> callback);

 protected:
  Server(MetaData &&meta_data) noexcept;
  virtual Result<std::unique_ptr<Detection>> CreateDetection(const ommatidia::JsonInput &config) noexcept = 0;
  
 private:
  crow::SimpleApp server_;
  MetaData meta_data_;
  std::unordered_map<int, std::unique_ptr<Detection>> detections_;

  crow::response GetRoot();
  crow::response GetDetections();
  crow::response PostDetections(const crow::request &request);
  crow::response GetDetection(int detection_index);
  crow::response DeleteDetection(int detection_index);
  crow::response Evaluate(int detection_index, const crow::request &request);
};

}  // namespace ommatidia