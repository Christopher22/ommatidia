#pragma once

#include <variant>
#include <string>
#include <memory>

#include <crow/common.h>
#include <crow/returnable.h>
#include <crow/http_response.h>

namespace ommatidia {
class Error {
 public:
  constexpr Error(const char *message) noexcept
      : Error(message, crow::INTERNAL_SERVER_ERROR) {}
  constexpr Error(const char *message, crow::status code) noexcept
      : message_(message), status_(code) {}
  constexpr Error(Error &&error) noexcept
      : message_(error.message_), status_(error.status_) {}

  operator crow::response() const;

 protected:
  const char *message_;
  crow::status status_;

 private:
  /// Formats a given string.
  /// Taken from
  /// https://stackoverflow.com/questions/2342162/stdstring-formatting-like-sprintf
  /// under CC0 1.0.
  template <typename... Args>
  static std::string format(const std::string &format, Args... args) {
    int size_s = std::snprintf(nullptr, 0, format.c_str(), args...) +
                 1;  // Extra space for '\0'
    if (size_s <= 0) {
      throw std::runtime_error("Error during formatting.");
    }
    auto size = static_cast<size_t>(size_s);
    std::unique_ptr<char[]> buf(new char[size]);
    std::snprintf(buf.get(), size, format.c_str(), args...);
    return std::string(buf.get(),
                       buf.get() + size - 1);  // We don't want the '\0' inside
  }
};

template <class T>
using Result = std::variant<T, Error>;

template <class T, class V>
struct ResultHandler {
 public:
  inline crow::response operator()(V &value) const noexcept {
    return this->callback_(value);
  }

  inline crow::response operator()(Error &error) const noexcept {
    return error;
  }

  T callback_;
};

template <typename T, typename V>
crow::response Check(Result<V> &result, T &&t) {
  ResultHandler<typename std::decay<T>::type, V> visitor = {std::forward<T>(t)};
  return std::visit(visitor, result);
}

}  // namespace ommatidia