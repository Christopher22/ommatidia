#include <ommatidia/external/server.hpp>

#include "PuRe.hpp"

int main() {
  ommatidia::MetaData meta(
      "PuRe", {"Thiago Santini", "Wolfgang Fuhl", "Enkelejda Kasneci"},
      "https://doi.org/10.1016/j.cviu.2018.02.002", ommatidia::License::Custom,
      ommatidia::PredictionType::Ellipse);
  ommatidia::external::Server<PuRe> server(std::move(meta));
  server.Run(8080);
  return 0;
}