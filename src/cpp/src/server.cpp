#include <crow/json.h>
#include <opencv2/core.hpp>
#include <opencv2/imgcodecs.hpp>

#include "../include/server.hpp"

namespace ommatidia {
using JsonValue = crow::json::wvalue;

Server::Server(MetaData&& meta_data) noexcept
    : server_(), detections_(), meta_data_(std::move(meta_data)) {
  CROW_ROUTE(server_, "/")
  ([this]() { return this->GetRoot(); });

  CROW_ROUTE(server_, "/detections/")
  ([this]() { return this->GetDetections(); });

  CROW_ROUTE(server_, "/detections/")
      .methods(crow::HTTPMethod::POST)([this](const crow::request& request) {
        return this->PostDetections(request);
      });

  CROW_ROUTE(server_, "/detections/<int>/")
      .methods(crow::HTTPMethod::GET)(
          [this](int detection) { return this->GetDetection(detection); });

  CROW_ROUTE(server_, "/detections/<int>/")
      .methods(crow::HTTPMethod::DELETE)(
          [this](int detection) { return this->DeleteDetection(detection); });

  CROW_ROUTE(server_, "/detections/<int>/evaluate/")
      .methods(crow::HTTPMethod::Post)(
          [this](const crow::request& request, int detection) {
            return this->PostEvaluation(detection, request);
          });

  CROW_ROUTE(server_, "/detections/<int>/evaluate/<int>")
  ([this](int detection, int sample_index) {
    return this->GetEvaluation(detection, sample_index);
  });
}

void Server::run(uint16_t port) { server_.port(port).multithreaded().run(); }

crow::response Server::run(crow::request request) {
  crow::response response;
  server_.handle(request, response);
  return response;
}

crow::response Server::GetRoot() { return crow::response(this->meta_data_); }

crow::response Server::GetDetections() {
  std::vector<JsonValue> detections;
  detections.reserve(this->detections_.size());
  for (const auto& detection : this->detections_) {
    detections.push_back(JsonValue(detection.first));
  }
  return crow::response(JsonValue(detections));
}

crow::response Server::PostDetections(const crow::request& request) {
  auto params = DetectionParams::Parse(request.body);
  return Check(params, [this](DetectionParams& params) {
    auto detection_result = this->CreateDetection(params);
    if (Error* error = std::get_if<Error>(&detection_result)) {
      return (crow::response)*error;
    } else {
      auto detection =
          std::get<std::unique_ptr<Detection>>(std::move(detection_result));
      auto index = this->detections_.size();
      this->detections_.insert({index, std::move(detection)});
      return crow::response(JsonValue(index));
    }
  });
}

crow::response Server::GetDetection(int detection_index) {
  if (this->detections_.find(detection_index) != this->detections_.end()) {
    return crow::response(crow::OK);
  } else {
    return Error("Given detection index is unknown", crow::NOT_FOUND);
  }
}

crow::response Server::DeleteDetection(int detection_index) {
  if (this->detections_.erase(detection_index) == 1) {
    return crow::response(crow::OK);
  } else {
    return Error("Given detection index is unknown", crow::NOT_FOUND);
  }
}

crow::response Server::PostEvaluation(int detection_index,
                                      const crow::request& request) {
  // Query the detection
  auto detection = this->detections_.find(detection_index);
  if (detection == this->detections_.end()) {
    return Error("Given detection index is unknown", crow::NOT_FOUND);
  }

  auto timestamp = request.url_params.get("t");

  // Try to load the image
  cv::_InputArray input_data(request.body.c_str(), (int)request.body.size());
  auto image = cv::imdecode(input_data, cv::IMREAD_GRAYSCALE);
  if (image.empty()) {
    return Error("The given image is invalid", crow::BAD_REQUEST);
  }

  // Analyse the image
  auto detection_result = detection->second->Predict(image, 0.0);
  if (detection_result.has_value()) {
    return *detection_result;
  }

  return crow::response(crow::OK);
}

crow::response Server::GetEvaluation(int detection_index, int sample_index) {
  auto detection = this->detections_.find(detection_index);
  if (detection == this->detections_.end()) {
    return Error("Given detection index is unknown", crow::NOT_FOUND);
  }

  auto sample = detection->second->GetPrediction(sample_index);
  if (sample.has_value()) {
    return crow::response(*sample);
  } else {
    return Error("The given sample is unknown", crow::NOT_FOUND);
  }
}

}  // namespace ommatidia