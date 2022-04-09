#pragma once

#include <string>
#include <vector>

#include <crow/returnable.h>

namespace ommatidia {

// The type of prediction supported by the choosen detector.
enum class PredictionType { Point, Ellipse, Mask };

// An indicator for type of training supported by the choosen detector.
enum class TrainingType { Unsupported, Optional, Required };

struct MetaData : public crow::returnable {
  std::string name;
  std::string publication;
  std::vector<std::string> authors;
  std::string license;
  PredictionType output;
  TrainingType training;

  // Whether or not the evaluation can get queried before all samples are given
  // to the detector.
  bool supports_streaming;

  MetaData(MetaData const &) = default;
  MetaData &operator=(MetaData const &) = delete;

  std::string dump() const override;
};
}  // namespace ommatidia