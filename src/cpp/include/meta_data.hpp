#pragma once

#include <string>
#include <string_view>
#include <initializer_list>
#include <vector>

#include <crow/returnable.h>

namespace ommatidia {

// The type of prediction supported by the choosen detector.
enum class PredictionType { Point, Ellipse, Mask };

// An indicator for type of training supported by the choosen detector.
enum class TrainingType { Unsupported, Optional, Required };

// The license which applies for any usage of the detector.
enum class License { PublicDomain, MIT, Apache, GPL, AGPL, Custom };

// Whether or not the evaluation can get queried before all samples are given
// to the detector.
enum class SupportStreaming { Yes = true, No = false };

class MetaData : public crow::returnable {
 public:
  MetaData(std::string_view name,
           std::initializer_list<std::string_view> authors,
           std::string_view additional_information, License license,
           PredictionType prediction_type, TrainingType training_type,
           SupportStreaming supports_streaming);
  MetaData(MetaData const &) = default;
  inline MetaData(MetaData &&moved_meta_data) noexcept
      : crow::returnable("text/json"),
        name_(std::move(moved_meta_data.name_)),
        authors_(std::move(moved_meta_data.authors_)),
        additional_information_(
            std::move(moved_meta_data.additional_information_)),
        license_(moved_meta_data.license_),
        output_(moved_meta_data.output_),
        training_(moved_meta_data.training_),
        supports_streaming_(moved_meta_data.supports_streaming_) {}

  virtual ~MetaData() {}
  MetaData &operator=(MetaData const &) = delete;

  std::string dump() const override;
  inline std::string_view Name() const noexcept { return name_; }
  std::vector<std::string_view> Authors() const;
  inline std::string_view AdditionalInformation() const noexcept {
    return additional_information_;
  }
  inline License LicenseInformation() const noexcept { return license_; }
  inline TrainingType TrainingSupport() const noexcept { return training_; }
  inline PredictionType PredictionOutput() const noexcept { return output_; }
  inline SupportStreaming StreamingSupport() const noexcept {
    return supports_streaming_;
  }

 protected:
  std::string name_;
  std::vector<std::string> authors_;
  std::string additional_information_;
  License license_;
  PredictionType output_;
  TrainingType training_;
  SupportStreaming supports_streaming_;

  static constexpr std::string_view AsString(PredictionType prediction) {
    switch (prediction) {
      case PredictionType::Point:
        return "Point";
      case PredictionType::Ellipse:
        return "Ellipse";
      default:
        return "Mask";
    }
  }

  static constexpr std::string_view AsString(TrainingType training) {
    switch (training) {
      case TrainingType::Unsupported:
        return "Unsupported";
      case TrainingType::Optional:
        return "Optional";
      default:
        return "Required";
    }
  }

  static constexpr std::string_view AsString(License license) {
    switch (license) {
      case License::PublicDomain:
        return "PublicDomain";
      case License::MIT:
        return "MIT";
      case License::Apache:
        return "Apache";
      case License::GPL:
        return "GPL";
      case License::AGPL:
        return "AGPL";
      default:
        return "Custom";
    }
  }
};
}  // namespace ommatidia