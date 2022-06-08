#include <ommatidia/external/server.hpp>

#include "ExCuSe.hpp"

int main() {
  ommatidia::MetaData meta("ExCuSe",
                           {"Wolfgang Fuhl", "Thomas Kübler", "Katrin Sippel",
                            "Wolfgang Rosenstiel", "Enkelejda Kasneci"},
                           "https://doi.org/10.1007/978-3-319-23192-1_4",
                           ommatidia::License::Custom,
                           ommatidia::PredictionType::Ellipse);
  ommatidia::external::Server<ExCuSe> server(std::move(meta));
  server.Run(8080);
  return 0;
}