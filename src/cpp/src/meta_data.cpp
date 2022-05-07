#include "../include/meta_data.hpp"

#include <crow/json.h>

namespace ommatidia {

MetaData::MetaData(std::string_view name,
                   std::initializer_list<std::string_view> authors,
                   std::string_view additional_information, License license,
                   PredictionType prediction_type)
    : crow::returnable("text/json"),
      name_(name),
      authors_(),
      additional_information_(additional_information),
      license_(license),
      output_(prediction_type) {
  authors_.reserve(authors.size());
  for (auto author = authors.begin(), end = authors.end(); author != end;
       ++author) {
    authors_.emplace_back(*author);
  }
}

std::vector<std::string_view> MetaData::Authors() const {
  std::vector<std::string_view> authors;
  authors.reserve(authors_.size());
  for (auto author = authors_.begin(), end = authors_.end(); author != end;
       ++author) {
    authors.emplace_back(*author);
  }
  return authors;
};

std::string MetaData::dump() const {
  std::vector<crow::json::wvalue> authors;
  authors.reserve(authors_.size());
  for (auto author = authors_.begin(), end = authors_.end(); author != end;
       ++author) {
    authors.emplace_back(*author);
  }

  return crow::json::wvalue(
             {
                 std::make_pair("name", crow::json::wvalue(name_)),
                 std::make_pair("authors", crow::json::wvalue(authors)),
                 std::make_pair("additional_information",
                                crow::json::wvalue(additional_information_)),
                 std::make_pair(
                     "license",
                     crow::json::wvalue(MetaData::AsString(license_).data())),
                 std::make_pair(
                     "prediction",
                     crow::json::wvalue(MetaData::AsString(output_).data())),
             })
      .dump();
}

}  // namespace ommatidia