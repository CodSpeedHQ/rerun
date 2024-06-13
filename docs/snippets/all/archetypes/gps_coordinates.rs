//! Log some very simple points.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rec = rerun::RecordingStreamBuilder::new("rerun_example_points3d").spawn()?;

    rec.log(
        "points",
        &rerun::GpsCoordinates::new([(47.6344, 19.1397, 0.0), (47.6344, 19.1399, 1.0)]),
    )?;

    Ok(())
}
