#include <iomanip>
#include <iostream>
#include <fstream>

#include <opencv2/imgcodecs.hpp>

#include "gfwx.hpp"

int main(int argc, char const * argv[])
{
    if (argc < 6) {
        std::cerr << "Usage: " << argv[0] <<
            " INPUT OUTPUT.GFWX quality filter encoder";
        return 1;
    }

    std::string input = argv[1];
    std::string gfwxOutput = argv[2];
    std::string qualityStr = argv[3];
    std::string filterStr = argv[4];
    std::string encoderStr = argv[5];

    cv::Mat image = cv::imread(input, cv::IMREAD_COLOR);

    int layers = 1;                               // just one image layer
    int channels = 3;                             // 3 interleaved channels
    int bitDepth = GFWX::BitDepthAuto;            // BitDepthAuto selects 8 or 16 based on type
    int chromaScale = 8;                          // chroma quality is divided by this number
    int blockSize = GFWX::BlockDefault;           // probably fine
    int quantization = GFWX::QuantizationScalar;  // only one choice here anyway
    int intent = GFWX::IntentBGR;                 // opencv uses BGR instead of RGB

    int quality = -1;
    try {
        quality = std::stoi(qualityStr);
        if (quality == 0 || quality > 1024) {
            throw std::invalid_argument("wrong value");
        }
    } catch (const std::invalid_argument &) {
        std::cerr << "Wrong quality value" << std::endl;
        return 1;
    }

    std::cout << filterStr << std::endl;
    int filter = -1;
    if (filterStr == "linear") {
        filter = GFWX::FilterLinear;
    } else if (filterStr == "cubic") {
        filter = GFWX::FilterCubic;
    } else {
        std::cerr << "Wrong filter value" << std::endl;
        return 1;
    }

    std::cout << encoderStr << std::endl;
    int encoder = -1;
    if (encoderStr == "fast") {
        encoder = GFWX::EncoderFast;
    } else if (encoderStr == "turbo") {
        encoder = GFWX::EncoderTurbo;
    } else if (encoderStr == "contextual") {
        encoder = GFWX::EncoderContextual;
    } else {
        std::cerr << "Wrong encoder value" << std::endl;
        return 1;
    }

    std::cout << "quality: " << quality << std::endl;
    std::cout << "filter: " << filterStr << std::endl;
    std::cout << "encoder: " << encoderStr << std::endl;
    std::cout << "intent: BGR" << std::endl;

    GFWX::Header header(image.size().width, image.size().height, layers, channels, bitDepth, quality,
        chromaScale, blockSize, filter, quantization, encoder, intent);

    std::vector<uchar> buffer(std::max((size_t)256, image.total() * image.elemSize() * 2));
    ptrdiff_t size = GFWX::compress(image.ptr(), header, &buffer[0], buffer.size(), 0, 0, 0);
    std::ofstream(gfwxOutput, std::ios::binary).write((char*)&buffer[0], size);

    return 0;
}
