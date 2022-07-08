# UnrealRustApi
Rust Bindings for UnrealEngine5

# USAGE
    first install rust environments(1.57 or later)
    git clone https://github.com/DrYaling/UnrealRustApi.git
    cd UnrealRustApi
    git submodule update --init --recursive
    ./build.bat
    (on windows, if your are running on linux or mac then you should use 
        cargo build --release 
    then use cp command or add a .sh file like build.bat)
## notice
close unreal editor if you are prepare to build rust dll/so
