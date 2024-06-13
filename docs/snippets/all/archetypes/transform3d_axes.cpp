// Log different transforms with visualized coordinates axes.

#include <rerun.hpp>

int main() {
    const auto rec = rerun::RecordingStream("rerun_example_transform3d_axes");
    rec.spawn().exit_on_failure();

    auto base_axes = rerun::Axes3D().with_length(1.0);
    auto other_axes = rerun::Axes3D().with_length(0.5);

    rec.log_static("base", base_axes);
    rec.log_static("base/rotated", other_axes);
    rec.log_static("base/rotated/translated", other_axes);

    for (int deg = 0; deg < 360; deg++) {
        rec.set_time_sequence("step", deg);

        rec.log(
            "base/rotated",
            rerun::Transform3D(rerun::RotationAxisAngle(
                {1.0f, 1.0f, 1.0f},
                rerun::Angle::degrees(static_cast<float>(deg))
            ))
        );

        rec.log("base/rotated/translated", rerun::Transform3D({2.0f, 0.0f, 0.0f}));
    }
}
