//! Rust bindings for the [AirSim project](https://github.com/microsoft/AirSim)'s `AirLib` C++ library.

use autocxx::include_cpp;

include_cpp! {
    #include "common/EarthCelestial.hpp"
    #include "common/EarthUtils.hpp"
    #include "common/FrequencyLimiter.hpp"
    #include "common/GaussianMarkov.hpp"
    #include "common/GeodeticConverter.hpp"
    #include "physics/DebugPhysicsBody.hpp"
    #include "physics/Environment.hpp"
    #include "physics/ExternalPhysicsEngine.hpp"
    #include "physics/FastPhysicsEngine.hpp"
    #include "physics/Kinematics.hpp"

    // common/EarthCelestial.hpp
    generate!("msr::airlib::EarthCelestial")

    // common/EarthUtils.hpp
    generate!("msr::airlib::EarthUtils")

    // common/FrequencyLimiter.hpp
    generate!("msr::airlib::FrequencyLimiter")

    // common/GaussianMarkov.hpp
    generate!("msr::airlib::GaussianMarkov")

    // common/GeodeticConverter.hpp
    generate!("msr::airlib::GeodeticConverter")

    // physics/DebugPhysicsBody.hpp
    generate!("msr::airlib::DebugPhysicsBody")

    // physics/Environment.hpp
    generate!("msr::airlib::Environment")

    // physics/ExternalPhysicsEngine.hpp
    generate!("msr::airlib::ExternalPhysicsEngine")

    // physics/FastPhysicsEngine.hpp
    generate!("msr::airlib::FastPhysicsEngine")

    // physics/Kinematics.hpp
    generate!("msr::airlib::Kinematics")

    safety!(unsafe_ffi)
}

// Re-export everything
pub use ffi::msr::airlib::*;
