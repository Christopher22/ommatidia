#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>

#include <detection_params.hpp>

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
      input = "{ \"width\": 42, \"height\": 22, \"unknown_key\": True }";
    }

    auto params_result = ommatidia::DetectionParams::Parse(input);
    auto& params = std::get<ommatidia::DetectionParams>(params_result);
    CHECK(params.Width() == 42);
    CHECK(params.Height() == 22);
  }
}