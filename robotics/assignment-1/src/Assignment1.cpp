#include <opencv2/opencv.hpp>
#include <iostream>
#include <optional>
#include <cassert>

using Rect = cv::Rect2i;
using Point = cv::Point2i;
using Size = cv::Size2i;

template <typename Callback>
static void iterateAllPoints(const cv::Mat& image, Callback&& cb) {
  assert(image.channels() == 1);
  for (int x = 0; x < image.rows; x++) {
    for (int y = 0; y < image.cols; ++y) {
      cb(Point(x, y), x * y);
    }
  }
}

static Rect estimatePositionViaBinarization(const cv::Mat& image) {
  assert(image.channels() == 3);
  // const double kThreshold = 253.0;
  const int kMax = std::numeric_limits<int>::max();
  std::vector<cv::Mat> channels;
  cv::split(image, channels);

  int t = kMax, r = 0, b = 0, l = kMax;
  const unsigned kBlueThreshold = 50;
  const unsigned kGreenThreshold = 50;
  const unsigned kRedThreshold = 250;

  const auto& blueChannel = channels[0];
  const auto& greenChannel = channels[1];
  const auto& redChannel = channels[2];
  assert(channels.size() == 3);

  cv::imshow("BLUE", blueChannel);
  cv::imshow("GREEN", greenChannel);
  cv::imshow("RED", redChannel);
  cv::waitKey();

  // This is some sort of "manual" binary thresholding, to be able to account
  // for all color channels and not.
  iterateAllPoints(redChannel, [&](const Point& point, int index) {
    const unsigned char red = redChannel.data[index];
    const unsigned char green = greenChannel.data[index];
    const unsigned char blue = blueChannel.data[index];

    if (red < kRedThreshold || green > kGreenThreshold || blue > kBlueThreshold) {
      return;
    }

    t = std::min(point.y, t);
    r = std::max(point.x, r);
    b = std::max(point.y, b);
    l = std::min(point.x, l);
  });
  return Rect(Point(t, l), Size(r - l, b - t));
}

static void compareRects(const Rect& reference, const Rect& estimate) {
  // TODO.
}

template <typename Approximation, typename Comparison>
int scanImages(const char** images, size_t imageCount, Approximation&& approximate, Comparison&& compare) {
  using Result = decltype(approximate(cv::Mat()));
  std::optional<Result> reference;
  for (size_t i = 0; i < imageCount; ++i) {
    const char* image = images[i];
    std::cout << image << ": ";
    cv::Mat mat = cv::imread(image);
    if (!mat.data) {
      std::cout << "ERROR" << std::endl;
      if (!reference) {
        return 1;
      }
    }
    Result r = approximate(mat);
    std::cout << r << " ";
    if (!reference) {
      reference.emplace(r);
    } else {
      compare(reference.value(), r);
    }
    std::cout << std::endl;
  }
  return 0;
}

int main(int argc, const char** argv) {
  if (argc < 3) {
    fprintf(stderr, "usage: %s <algorithm> <reference-image> <test-image>...\n", argv[0]);
    fprintf(stderr, "  <algorithm>: binarization|countours\n");
    return 1;
  }
  Rect reference;
  const char* algo = argv[1];
  const char** images = argv + 2;
  const size_t imageCount = size_t(argc) - 2;
  if (!std::strcmp(algo, "binarization")) {
    return scanImages(images, imageCount, estimatePositionViaBinarization, compareRects);
  }
  if (!std::strcmp(algo, "countours")) {
    // TODO
  }
  fprintf(stderr, "Unknown algorithm: %s\n", algo);
  return 1;
}
