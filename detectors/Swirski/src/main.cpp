#include <ommatidia/external/server.hpp>

#include "Swirski.hpp"

int main() {
  ommatidia::MetaData meta(
      "Swirski", {"Lech Świrski", "Andreas Bulling", "Neil Dodgson"},
      "https://doi.org/10.1145/2168556.2168585", ommatidia::License::MIT,
      ommatidia::PredictionType::Ellipse, ommatidia::TrainingType::Unsupported,
      ommatidia::SupportStreaming::Yes);
  ommatidia::external::Server<Swirski2D> server(std::move(meta));
  server.Run(8080);
  return 0;
}