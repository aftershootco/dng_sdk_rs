use std::fs::File;

use dng_sdk_rs::coord::Coord;
use dng_sdk_rs::host::Host;
use dng_sdk_rs::temparature::Temperature;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub fn main() -> Result<()> {
    let file = "file.dng";
    let mut f = File::open(file).unwrap();
    let info = Info::default();
    let host = Host::default();
    info.parse(host, stream);
    info.post_parse(stream);
    let negative = host.make_dng_negative();
    negative.parse(host, stream, info);
    let profile_id = CameraProfileID::default();
    let spec = negative.make_color_spec(profile_id);
    if negative.has_camera_neutral() {
        spec.set_white_xy(spec.neutral_to_xy(negative.camera_neutral()));
    } else if negative.has_camera_white_xy() {
        spec.set_white_xy(negative.camera_white_xy());
    } else {
        spec.set_white_xy(Coord::D55_XY_COORD)
    }
    let temp = Temperature::from_xy(spec.white_xy());
    println!("{:?}", temp);
    Ok(())
}
