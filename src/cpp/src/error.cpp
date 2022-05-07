#include "../include/error.hpp"

namespace ommatidia {

ommatidia::Error::operator crow::response() const {
  return crow::response(this->status_, "txt", std::string(this->message_));
}

}  // namespace ommatidia