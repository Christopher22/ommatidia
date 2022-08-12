#include <vector>

#include "algo.h"
#include <ommatidia/server.hpp>
#include <ommatidia/detection.hpp>
#include <ommatidia/predictions/ellipse.hpp>

class Detection : public ommatidia::Detection {
 public:
  inline Detection(int width, int height, int min_rad = 6, int max_rad = 15,
                   int step_rad = 2, float step_ori = 3.0f, int dist_diff = 1)
      : ommatidia::Detection(), detector_(), ellipses_() {
    detector_.m_init_without_file(min_rad, max_rad, step_rad, step_ori,
                                  dist_diff);
    detector_.set_INPUT_SIZE(width, height);
  }

  ommatidia::Result<ommatidia::JsonValue> Predict(
      cv::InputArray sample) noexcept override {
    auto detection = detector_.run_fast(sample.getMat());
    ommatidia::Ellipse detected_ellipse(detection.eli,
                                        detection.valid ? 1.0 : 0.0);
    return detected_ellipse.Serialize();
  }

 private:
  BORE detector_;
  std::vector<ommatidia::Ellipse> ellipses_;
};

class Server : public ommatidia::Server {
 public:
  Server()
      : ommatidia::Server(ommatidia::MetaData(
            "BORE",
            {"Wolfgang Fuhl", "Shahram Eivazi", "Benedikt Hosp", "Anna Eivazi",
             "Wolfgang Rosenstiel", "Enkelejda Kasneci"},
            "https://doi.org/10.1145/3204493.3204558",
            ommatidia::License::Custom, ommatidia::PredictionType::Ellipse)) {}

 protected:
  ommatidia::Result<std::unique_ptr<ommatidia::Detection>> CreateDetection(
      const ommatidia::JsonInput &config) noexcept override {
    if (!config.has("width") || !config.has("height")) {
      return ommatidia::Error("Required fields 'width' or 'height' missing",
                              crow::BAD_REQUEST);
    }
    return std::make_unique<Detection>(config["width"].i(),
                                       config["height"].i());
  }
};

int main() {
  Server server;
  server.Run(8080);
  return 0;
}