fn main() {
    let include_paths = [
        std::path::PathBuf::from("AirSim/AirLib/include"),
        std::path::PathBuf::from("AirSim/AirLib/deps/eigen3"),
        std::path::PathBuf::from("AirSim/AirLib/deps/rpclib/include"),
        std::path::PathBuf::from("AirSim/AirLib/deps/MavLinkCom/include"),
    ];
    let mut build = autocxx_build::build("src/lib.rs", &include_paths, &[]).unwrap();
    build.flag_if_supported("-std=c++14").compile("airsim");

    println!("cargo:rerun-if-changed=src/lib.rs");
}
