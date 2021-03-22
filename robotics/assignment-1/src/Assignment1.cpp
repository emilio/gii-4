#include <opencv2/opencv.hpp>
#include <iostream>
#include <optional>

using Rect = cv::Rect2f;
static Rect estimatePositionViaBinarization(const cv::Mat& image) {
  return Rect();
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
