#include <vector>

#include <ommatidia/server.hpp>
#include <ommatidia/detection.hpp>
#include <ommatidia/predictions/ellipse.hpp>

#include "PupilDetectorHaar.h"

class Detection : public ommatidia::Detection {
 public:
  inline Detection() : ommatidia::Detection(), detector_() {}

  ommatidia::Result<ommatidia::JsonValue> Predict(
      cv::InputArray frame) noexcept override {
    auto frame_mat = frame.getMat();
    checkImg(frame_mat);
    Mat img_gray;
    img2Gray(frame_mat, img_gray);
    detector_.detect(img_gray);

    cv::RotatedRect ellipse_rect;
    Point2f center_fitting;
    bool flag = detector_.extractEllipse(img_gray, detector_.pupil_rect_fine_,
                                         ellipse_rect, center_fitting);

    return ommatidia::Ellipse(ellipse_rect, flag ? 1.0 : 0.0).Serialize();
  }

 private:
  PupilDetectorHaar detector_;
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