use airlib::EarthUtils;

fn main() {
    println!(
        "The Earth's gravity at sea level is {} m/s^2.",
        EarthUtils::getGravity(0.0)
    );
}
