#include "../include/error.hpp"

namespace ommatidia {

ommatidia::Error::operator crow::response() const {
  return crow::response(this->status_, "text/json",
                        Error::format("{ \"error_msg\": \"%s\", \"code\": %u }",
                                      this->message_, this->status_));
}

}  // namespace ommatidia