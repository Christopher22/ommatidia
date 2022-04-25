#include <ommatidia/external/server.hpp>

#include "ElSe.hpp"

int main() {
  ommatidia::MetaData meta(
      "ElSe",
      {"Wolfgang Fuhl", "Thiago Santini", "Thomas Kübler", "Enkelejda Kasneci"},
      "https://doi.org/10.1145/2857491.2857505", ommatidia::License::Custom,
      ommatidia::PredictionType::Ellipse, ommatidia::TrainingType::Unsupported,
      ommatidia::SupportStreaming::Yes);
  ommatidia::external::Server<ElSe> server(std::move(meta));
  server.Run(8080);
  return 0;
}