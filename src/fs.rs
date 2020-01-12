use std::path::Path;
use android_glue::AssetError;

pub fn load<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, AssetError> {
    let filename = path.as_ref().to_str().expect("Can't convert path to &str");
    android_glue::load_asset(filename)
}
