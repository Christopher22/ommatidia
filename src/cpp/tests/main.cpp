#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN

#include <string_view>

#include <doctest/doctest.h>
#include <crow/json.h>
#include <opencv2/imgcodecs.hpp>

#include <image_size.hpp>
#include <meta_data.hpp>
#include <predictions/ellipse.hpp>
#include <predictions/pupil_center.hpp>
#include <external/server.hpp>

#include "external/example_detection.hpp"

static ommatidia::MetaData GenerateExampleData() {
  return ommatidia::MetaData("ExampleDetector", {"Jane Doe", "Max Mustermann"},
                             "https://more-information.tld",
                             ommatidia::License::GPL,
                             ommatidia::PredictionType::Ellipse);
}

// Test the output send the the clients
TEST_SUITE("Prediction Output") {
  TEST_CASE("Pupil center") {
    ommatidia::PupilCenter center(4, 5, 120, 130, 0.7);
    const auto serialized_ellipse = center.Serialize().dump();

    auto json = crow::json::load(serialized_ellipse);
    CHECK(json["type"] == "Point");
    CHECK(json["x"] == 4);
    CHECK(json["y"] == 5);
    CHECK(json["sample"]["width"] == 120);
    CHECK(json["sample"]["height"] == 130);
    CHECK(json["confidence"] == 0.7);
  }

  TEST_CASE("Ellipse: From constructor") {
    ommatidia::Size value1, value2;
    SUBCASE("Min-Max order") {
      value1 = 20.0;
      value2 = 21.0;
    }
    SUBCASE("Max-Min order") {
      value1 = 21.0;
      value2 = 20.0;
    }

    ommatidia::Ellipse ellipse(4, 5, value1, value2,
                               ommatidia::Radian::FromDegree(90.0), 120, 130,
                               0.7);
    const auto serialized_ellipse = ellipse.Serialize().dump();

    auto json = crow::json::load(serialized_ellipse);
    CHECK(json["type"] == "Ellipse");
    CHECK(json["x"] == 4);
    CHECK(json["y"] == 5);
    CHECK(json["major"] == 21.0);
    CHECK(json["minor"] == 20.0);
    CHECK(json["sample"]["width"] == 120);
    CHECK(json["sample"]["height"] == 130);
    CHECK(static_cast<float>(json["angle"].d()) ==
          doctest::Approx(ommatidia::Radian::PI / 2.0f).epsilon(0.01));
    CHECK(json["confidence"] == 0.7);
  }

  TEST_CASE("Ellipse: From Pupil") {
    const Pupil pupil(
        cv::RotatedRect(cv::Point2f(5.0, 4.0), cv::Point2f(6.0, 7.0), 42.0),
        0.22);

    ommatidia::Ellipse ellipse(pupil, 120, 130, pupil.confidence);
    const auto serialized_ellipse = ellipse.Serialize().dump();

    auto json = crow::json::load(serialized_ellipse);
    CHECK(json["type"] == "Ellipse");
    CHECK(json["x"] == 5);
    CHECK(json["y"] == 4);
    CHECK(json["major"] == 7.0);
    CHECK(json["minor"] == 6.0);
    CHECK(static_cast<float>(json["angle"].d()) ==
          doctest::Approx(0.733f).epsilon(0.01));
    CHECK(json["sample"]["width"] == 120);
    CHECK(json["sample"]["height"] == 130);
    CHECK(json["confidence"] == 0.22);
  }
}

TEST_SUITE("ImageSize") {
  TEST_CASE("Creation") {
    ommatidia::ImageSize params(42, 22);
    CHECK(params.Width() == 42);
    CHECK(params.Height() == 22);
  }

  TEST_CASE("Parsing") {
    std::string_view input;
    SUBCASE("Easy") { input = "{ \"width\": 42, \"height\": 22 }"; }
    SUBCASE("UnknownKey") {
      input = "{ \"width\": 42, \"height\": 22, \"unknown_key\": 1 }";
    }

    auto params_result = ommatidia::ImageSize::Parse(input);
    auto& params = std::get<ommatidia::ImageSize>(params_result);
    CHECK(params.Width() == 42);
    CHECK(params.Height() == 22);
  }
}

TEST_SUITE("MetaData") {
  TEST_CASE("Creation") {
    auto meta = GenerateExampleData();
    CHECK(meta.Name() == "ExampleDetector");
    CHECK(meta.AdditionalInformation() == "https://more-information.tld");
    CHECK(meta.LicenseInformation() == ommatidia::License::GPL);
    CHECK(meta.PredictionOutput() == ommatidia::PredictionType::Ellipse);

    auto authors = meta.Authors();
    REQUIRE(authors.size() == 2);
    CHECK(authors[0] == "Jane Doe");
    CHECK(authors[1] == "Max Mustermann");
  }

  TEST_CASE("Clone") {
    ommatidia::MetaData meta_original = GenerateExampleData();
    ommatidia::MetaData meta(meta_original);

    CHECK(meta.Name() == "ExampleDetector");
    CHECK(meta.AdditionalInformation() == "https://more-information.tld");
    CHECK(meta.LicenseInformation() == ommatidia::License::GPL);
    CHECK(meta.PredictionOutput() == ommatidia::PredictionType::Ellipse);

    auto authors = meta.Authors();
    REQUIRE(authors.size() == 2);
    CHECK(authors[0] == "Jane Doe");
    CHECK(authors[1] == "Max Mustermann");
  }

  TEST_CASE("Move") {
    ommatidia::MetaData meta_original = GenerateExampleData();
    ommatidia::MetaData meta(std::move(meta_original));

    CHECK(meta.Name() == "ExampleDetector");
    CHECK(meta.AdditionalInformation() == "https://more-information.tld");
    CHECK(meta.LicenseInformation() == ommatidia::License::GPL);
    CHECK(meta.PredictionOutput() == ommatidia::PredictionType::Ellipse);

    auto authors = meta.Authors();
    CHECK(authors.size() == 2);
    CHECK(authors[0] == "Jane Doe");
    CHECK(authors[1] == "Max Mustermann");
  }

  TEST_CASE("JSON") {
    ommatidia::MetaData meta = GenerateExampleData();

    auto json_body = crow::json::load(meta.dump());
    CHECK(json_body["name"] == "ExampleDetector");
    CHECK(json_body["additional_information"] ==
          "https://more-information.tld");
    CHECK(json_body["authors"][0] == "Jane Doe");
    CHECK(json_body["authors"][1] == "Max Mustermann");
    CHECK(json_body["license"] == "GPL");
    CHECK(json_body["prediction"] == "Ellipse");
  }
}

TEST_SUITE("Server") {
  TEST_CASE("Creation") {
    ommatidia::external::Server<ExampleDetection> server(
        std::move(GenerateExampleData()));
  }

  TEST_CASE("Quey meta data") {
    ommatidia::external::Server<ExampleDetection> server(
        std::move(GenerateExampleData()));

    auto response =
        server.Run([](crow::request& request) { request.url = "/"; });
    REQUIRE(response.code == 200);

    auto json_body = crow::json::load(response.body);
    CHECK(json_body["name"] == "ExampleDetector");
    CHECK(json_body["additional_information"] ==
          "https://more-information.tld");
    CHECK(json_body["authors"][0] == "Jane Doe");
    CHECK(json_body["authors"][1] == "Max Mustermann");
    CHECK(json_body["license"] == "GPL");
    CHECK(json_body["prediction"] == "Ellipse");
  }

  TEST_SUITE("Detections") {
    TEST_CASE("Query all") {
      ommatidia::external::Server<ExampleDetection> server(
          std::move(GenerateExampleData()));

      auto response = server.Run(
          [](crow::request& request) { request.url = "/detections/"; });
      REQUIRE(response.code == 200);

      auto json_body = crow::json::load(response.body);
      CHECK(json_body.t() == crow::json::type::List);
      CHECK(json_body.size() == 0);
    }

    TEST_CASE("Query invalid") {
      ommatidia::external::Server<ExampleDetection> server(
          std::move(GenerateExampleData()));

      auto response = server.Run(
          [](crow::request& request) { request.url = "/detections/42/"; });
      REQUIRE(response.code == 404);
    }

    TEST_CASE("Create") {
      ommatidia::external::Server<ExampleDetection> server(
          std::move(GenerateExampleData()));

      auto response = server.Run([](crow::request& request) {
        request.method = crow::HTTPMethod::POST;
        request.url = "/detections/";
        request.body = crow::json::wvalue({std::make_pair("width", 4),
                                           std::make_pair("height", 5)})
                           .dump();
      });
      REQUIRE(response.code == 200);

      auto json_body = crow::json::load(response.body);
      CHECK(json_body.t() == crow::json::type::Number);
      CHECK(json_body.i() == 0);

      SUBCASE("Queryable") {
        auto response = server.Run(
            [](crow::request& request) { request.url = "/detections/0/"; });
        CHECK(response.code == 200);
      }
      SUBCASE("Findable in list") {
        auto response = server.Run(
            [](crow::request& request) { request.url = "/detections/"; });
        REQUIRE(response.code == 200);

        auto json_body = crow::json::load(response.body);
        CHECK(json_body.t() == crow::json::type::List);
        CHECK(json_body.size() == 1);
        CHECK(json_body[0] == 0);
      }
    }

    TEST_CASE("Delete") {
      ommatidia::external::Server<ExampleDetection> server(
          std::move(GenerateExampleData()));

      // The correct creation is checked by the previous test
      auto response = server.Run([](crow::request& request) {
        request.method = crow::HTTPMethod::POST;
        request.url = "/detections/";
        request.body = crow::json::wvalue({std::make_pair("width", 4),
                                           std::make_pair("height", 5)})
                           .dump();
      });
      REQUIRE(response.code == 200);

      server.Run([](crow::request& request) {
        request.method = crow::HTTPMethod::DELETE;
        request.url = "/detections/0/";
      });
      REQUIRE(response.code == 200);

      SUBCASE("Not queryable") {
        auto response = server.Run(
            [](crow::request& request) { request.url = "/detections/0/"; });
        CHECK(response.code == 404);
      }
      SUBCASE("Not findable in list") {
        auto response = server.Run(
            [](crow::request& request) { request.url = "/detections/"; });
        REQUIRE(response.code == 200);

        auto json_body = crow::json::load(response.body);
        CHECK(json_body.t() == crow::json::type::List);
        CHECK(json_body.size() == 0);
      }
    }

    TEST_CASE("Delete non-existing") {
      ommatidia::external::Server<ExampleDetection> server(
          std::move(GenerateExampleData()));

      auto response = server.Run([](crow::request& request) {
        request.method = crow::HTTPMethod::DELETE;
        request.url = "/detections/0/";
      });
      REQUIRE(response.code == 404);
    }

    TEST_CASE("Evaluation") {
      ommatidia::external::Server<ExampleDetection> server(
          std::move(GenerateExampleData()));

      // Create detection
      {
        auto response = server.Run([](crow::request& request) {
          request.method = crow::HTTPMethod::POST;
          request.url = "/detections/";
          request.body = crow::json::wvalue({}).dump();
        });
        REQUIRE(response.code == 200);
      }

      // Send the image to the server
      {
        crow::request request;
        request.method = crow::HTTPMethod::POST;
        request.url = "/detections/0/";

        {
          cv::Mat image = cv::Mat::zeros(5, 4, CV_8UC1);
          std::vector<unsigned char> buffer;
          SUBCASE("PNG") { cv::imencode(".png", image, buffer); }
          SUBCASE("JPG") { cv::imencode(".jpg", image, buffer); }
          SUBCASE("BMP") { cv::imencode(".bmp", image, buffer); }
          request.body.assign(reinterpret_cast<char*>(buffer.data()),
                              buffer.size());
        }

        auto response = server.Run(request);
        REQUIRE(response.code == 200);

        auto json = crow::json::load(response.body);
        CHECK(json.t() == crow::json::type::Object);
        CHECK(json["x"] == 5);
        CHECK(json["y"] == 4);
        CHECK(json["major"] == 5.0);
        CHECK(json["minor"] == 4.0);
        CHECK(static_cast<float>(json["angle"].d()) ==
              doctest::Approx(0.733f).epsilon(0.01));
        CHECK(json["confidence"] == 0.22);
      }
    }
  }
}