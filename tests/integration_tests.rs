use laydown::data_file;
use laydown::parse_arguments;

#[test]
fn test_did() {}

fn get_path_to_test_file() -> PathBuf {
    let test_config_directory = dirs::config_dir()
        .expect("Failed to find laydown test directory")
        .join("test_laydown");

    fs::create_dir(&laydown_test_directory).ok();

    let test_file_path: PathBuf = test_config_directory.join("test_laydown.ron");

    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&ron_file_path)
        .expect("Failed to find laydown.ron file");

    ron_file_path
}
