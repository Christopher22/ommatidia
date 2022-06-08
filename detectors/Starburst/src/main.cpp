#include <ommatidia/external/server.hpp>

#include "Starburst.hpp"

int main() {
  ommatidia::MetaData meta(
      "Starburst", {"Dongheng Li", "David Winfield", "Derrick Parkhurst"},
      "https://doi.org/10.1109/CVPR.2005.531", ommatidia::License::GPL,
      ommatidia::PredictionType::Ellipse);
  ommatidia::external::Server<Starburst> server(std::move(meta));
  server.Run(8080);
  return 0;
}