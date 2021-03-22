#include <opencv2/opencv.hpp>

int main(int argc, const char** argv) {
  if (argc < 2) {
    fprintf(stderr, "usage: %s <reference-image> <test-image>...", argv[0]);
    return 1;
  }
  return 0;
}
