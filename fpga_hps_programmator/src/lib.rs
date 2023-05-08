mod errors;
mod flash;

// use std::path::{Path, PathBuf};
// use std::process::Command;
// use sys_mount::Mount;

//
// /// takes path to .rbf file and then generates all device tree files, mounts fs and flashes fpga
// pub fn flash_fpga(rbf_file_path: &Path) -> Result<(), io::Error> {
//     if !rbf_file_path.is_file() {
//         return Err("Bad path");
//     }
//     let file_name = rbf_file_path.file_name()?;
//     // 1) create dtso file
//     // 2)
// }
//
// /// create device tree source overlay file
// fn create_dtso(file_name: &str) -> PathBuf {
//     let dtso_template = format!("
//         /dts-v1/;
//         /plugin/;
//         /{{
//             fragment@0 {{
//             target-path = \"/soc/base_fpga_region\";
//             __overlay__ {{
//                 #address-cells = <1>;
//                 #size-cells = <1>;
//                 firmware-name = \"{file_name}.rbf\";
//             }};
//             }};
//         }};
//     ");
// }
//
// fn compile_device_tree_overlay() {
//     let command = Command::new("dtc")
//         .args("-O dtb -o {name}.dtbo -b 0 -@ {name}.dtso");
//     // let cmd = "dtc -O dtb -o blink.dtbo -b 0 -@ blink.dtso";
// }
//
// fn mount_abc() -> Result<> {
//     let mount = Mount::new(src, dst);
// }
