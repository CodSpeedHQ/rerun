// Log some very simple points.

#include <rerun.hpp>

int main() {
    const auto rec = rerun::RecordingStream("rerun_example_gps_coordinates");
    rec.spawn().exit_on_failure();

    rec.log("points", rerun::GpsCoordinates({{47.6343f, 19.1397f, 0.0f}, {47.6344f, 19.1395f, 1.0f}}));
}
