fn main() {
    // import the std::fs

    // Use the std::env module to to get the appDir for cross Platform
    // --> also need to check if the files exist already. If not 
    //     then go ahead an make them. 
    
    // copy the files to the appDir from ./data
    // --> Only do this if the directory did not exist before!

    // build: Profit 🤯
    tauri_build::build()
}
