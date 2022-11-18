use directories::ProjectDirs;

pub fn get_tags_filename() -> String {
    let mut tags_file: String = String::new();
    if let Some(proj_dirs) = ProjectDirs::from("com", "null ptr", "refiv") {
        tags_file = proj_dirs.data_dir().to_owned().into_os_string().into_string().unwrap();
        tags_file.push_str("\\tags.json");
    }
    tags_file
}

pub fn get_conf_filename() -> String {
    let mut conf_file: String = String::new();
    if let Some(proj_dirs) = ProjectDirs::from("com", "null ptr", "refiv") {
        conf_file = proj_dirs.config_dir().to_owned().into_os_string().into_string().unwrap();
        conf_file.push_str("\\conf.json");
    }
    conf_file
}

pub fn get_extension_from_filename(filename: &str) -> Option<&str> {
    std::path::Path::new(filename)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
}