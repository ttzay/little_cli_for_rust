//build.rs 能够在编译时做额外动作，此文件功能是编译 abi.proto 生成 abi.rs 文件
fn main() {
    prost_build::Config::new()
        .out_dir("src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
}