use fpga_hps_programmator::flash_fpga;
use std::path::Path;

fn main() {
    let rbf_firmware_file_path = Path::new("test.rbf");
    println!("Try to flash {rbf_firmware_file_path:?} file");
    flash_fpga(rbf_firmware_file_path).expect("Error during fpga flash");
    println!("Flash done!!");
}
