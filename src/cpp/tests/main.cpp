#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>
#include <crow/json.h>

#include <detection_params.hpp>
#include <meta_data.hpp>

TEST_SUITE("DetectionParams") {
  TEST_CASE("Creation") {
    ommatidia::DetectionParams params(42, 22);
    CHECK(params.Width() == 42);
    CHECK(params.Height() == 22);
  }

  TEST_CASE("Parsing") {
    std::string_view input;
    SUBCASE("Easy") { input = "{ \"width\": 42, \"height\": 22 }"; }
    SUBCASE("UnknownKey") {
      input = "{ \"width\": 42, \"height\": 22, \"unknown_key\": 1 }";
    }

    auto params_result = ommatidia::DetectionParams::Parse(input);
    auto& params = std::get<ommatidia::DetectionParams>(params_result);
    CHECK(params.Width() == 42);
    CHECK(params.Height() == 22);
  }
}

TEST_SUITE("MetaData") {
  TEST_CASE("Creation") {
    ommatidia::MetaData meta(
        "ExampleDetector", {"Jane Doe", "Max Mustermann"},
        "https://more-information.tld", ommatidia::License::GPL,
        ommatidia::PredictionType::Ellipse,
        ommatidia::TrainingType::Unsupported, ommatidia::SupportStreaming::Yes);

    CHECK(meta.Name() == "ExampleDetector");
    CHECK(meta.AdditionalInformation() == "https://more-information.tld");
    CHECK(meta.LicenseInformation() == ommatidia::License::GPL);
    CHECK(meta.PredictionOutput() == ommatidia::PredictionType::Ellipse);
    CHECK(meta.TrainingSupport() == ommatidia::TrainingType::Unsupported);

    auto authors = meta.Authors();
    REQUIRE(authors.size() == 2);
    CHECK(authors[0] == "Jane Doe");
    CHECK(authors[1] == "Max Mustermann");
  }

  TEST_CASE("Clone") {
    ommatidia::MetaData meta_original(
        "ExampleDetector", {"Jane Doe", "Max Mustermann"},
        "https://more-information.tld", ommatidia::License::GPL,
        ommatidia::PredictionType::Ellipse,
        ommatidia::TrainingType::Unsupported, ommatidia::SupportStreaming::Yes);
    ommatidia::MetaData meta(meta_original);

    CHECK(meta.Name() == "ExampleDetector");
    CHECK(meta.AdditionalInformation() == "https://more-information.tld");
    CHECK(meta.LicenseInformation() == ommatidia::License::GPL);
    CHECK(meta.PredictionOutput() == ommatidia::PredictionType::Ellipse);
    CHECK(meta.TrainingSupport() == ommatidia::TrainingType::Unsupported);

    auto authors = meta.Authors();
    REQUIRE(authors.size() == 2);
    CHECK(authors[0] == "Jane Doe");
    CHECK(authors[1] == "Max Mustermann");
  }

  TEST_CASE("Move") {
    ommatidia::MetaData meta_original(
        "ExampleDetector", {"Jane Doe", "Max Mustermann"},
        "https://more-information.tld", ommatidia::License::GPL,
        ommatidia::PredictionType::Ellipse,
        ommatidia::TrainingType::Unsupported, ommatidia::SupportStreaming::Yes);
    ommatidia::MetaData meta(std::move(meta_original));

    CHECK(meta.Name() == "ExampleDetector");
    CHECK(meta.AdditionalInformation() == "https://more-information.tld");
    CHECK(meta.LicenseInformation() == ommatidia::License::GPL);
    CHECK(meta.PredictionOutput() == ommatidia::PredictionType::Ellipse);
    CHECK(meta.TrainingSupport() == ommatidia::TrainingType::Unsupported);

    auto authors = meta.Authors();
    CHECK(authors.size() == 2);
    CHECK(authors[0] == "Jane Doe");
    CHECK(authors[1] == "Max Mustermann");
  }

  TEST_CASE("JSON") {
    ommatidia::MetaData meta(
        "ExampleDetector", {"Jane Doe", "Max Mustermann"},
        "https://more-information.tld", ommatidia::License::GPL,
        ommatidia::PredictionType::Ellipse,
        ommatidia::TrainingType::Unsupported, ommatidia::SupportStreaming::Yes);

    auto json_body = crow::json::load(meta.dump());
    CHECK(json_body["name"] == "ExampleDetector");
    CHECK(json_body["additional_information"] ==
          "https://more-information.tld");
    CHECK(json_body["authors"][0] == "Jane Doe");
    CHECK(json_body["authors"][1] == "Max Mustermann");
    CHECK(json_body["license"] == "GPL");
    CHECK(json_body["prediction"] == "Ellipse");
    CHECK(json_body["training"] == "Unsupported");
    CHECK(json_body["supports_streaming"]);
  }
}