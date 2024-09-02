fn main() {
    // Use vcpkg to find and link OpenCV libraries
    vcpkg::find_package("opencv4").unwrap();

    // Manually specify the path to the OpenCV library if needed
    println!("cargo:rustc-link-search=native=F:\\DevelopmentKit\\vcpkg\\installed\\x64-windows-static\\lib");

    // Link the necessary OpenCV libraries
    println!("cargo:rustc-link-lib=opencv_core");
    println!("cargo:rustc-link-lib=opencv_imgproc");
    println!("cargo:rustc-link-lib=opencv_imgcodecs");
    println!("cargo:rustc-link-lib=opencv_highgui");
    println!("cargo:rustc-link-lib=opencv_dnn");
    println!("cargo:rustc-link-lib=opencv_features2d");
    println!("cargo:rustc-link-lib=opencv_flann");
    println!("cargo:rustc-link-lib=opencv_ml");
    println!("cargo:rustc-link-lib=opencv_objdetect");
    println!("cargo:rustc-link-lib=opencv_photo");
    println!("cargo:rustc-link-lib=opencv_stitching");
    println!("cargo:rustc-link-lib=opencv_video");
    println!("cargo:rustc-link-lib=opencv_videoio");
}