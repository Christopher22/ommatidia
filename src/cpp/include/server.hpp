#pragma once

#include <unordered_map>
#include <string>
#include <memory>
#include <functional>

#include <crow/app.h>

#include "detection.hpp"
#include "meta_data.hpp"
#include "detection_params.hpp"
#include "error.hpp"

namespace ommatidia {

class Server {
 public:
  virtual ~Server() = default;
  Server(Server const &) = delete;
  Server &operator=(Server const &) = delete;

  void run(uint16_t port);
  crow::response run(crow::request request);
  crow::response run(std::function<void(crow::request&)> callback);

 protected:
  Server(MetaData &&meta_data) noexcept;
  virtual Result<std::unique_ptr<Detection>> CreateDetection(
      DetectionParams parameters) noexcept = 0;

 private:
  crow::SimpleApp server_;
  MetaData meta_data_;
  std::unordered_map<int, std::unique_ptr<Detection>> detections_;

  crow::response GetRoot();
  crow::response GetDetections();
  crow::response PostDetections(const crow::request &request);
  crow::response GetDetection(int detection_index);
  crow::response DeleteDetection(int detection_index);
  crow::response PostEvaluation(int detection_index,
                                const crow::request &request);
  crow::response GetEvaluation(int detection_index, int sample_index);
};

}  // namespace ommatidia