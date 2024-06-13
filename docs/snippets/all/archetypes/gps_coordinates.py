"""Log some gps coordinates."""

import rerun as rr

rr.init("rerun_example_gps_coordinates", spawn=True)

rr.log("points", rr.GpsCoordinates([[47.6344, 19.1397, 0], [47.6334, 19.1399, 1]]))
