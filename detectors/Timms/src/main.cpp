#include <vector>

#include <ommatidia/server.hpp>
#include <ommatidia/detection.hpp>
#include <ommatidia/predictions/pupil_center.hpp>

#include "timm_two_stage.h"

class Detection : public ommatidia::Detection {
 public:
  inline Detection(int blur = 0, int window_width = 150)
      : ommatidia::Detection(), detector_() {
    detector_.setup(USE_NO_VEC);
  }

  ommatidia::Result<ommatidia::JsonValue> Predict(
      cv::InputArray sample) noexcept override {
    auto sample_mat = sample.getMat();
    auto detection = std::get<0>(detector_.pupil_center(sample_mat));
    ommatidia::PupilCenter detected_center(
        ommatidia::Position(detection.x), ommatidia::Position(detection.y),
        sample_mat.cols, sample_mat.rows, -1.0f);
    return detected_center.Serialize();
  }

 private:
  Timm_two_stage detector_;
};

class Server : public ommatidia::Server {
 public:
  Server()
      : ommatidia::Server(ommatidia::MetaData(
            "TIMM", {"André Frank Krause", "Kai Essig"},
            "https://doi.org/10.1145/3314111.3319849", ommatidia::License::GPL,
            ommatidia::PredictionType::Ellipse)) {}

 protected:
  ommatidia::Result<std::unique_ptr<ommatidia::Detection>> CreateDetection(
      const ommatidia::JsonInput &config) noexcept override {
    return std::make_unique<Detection>();
  }
};

int main() {
  Server server;
  server.Run(8080);
  return 0;
}