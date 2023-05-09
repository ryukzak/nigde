use crate::errors::FpgaProgError;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};
use sys_mount::Mount;

/// takes path to .rbf file and then generates all device tree files, mounts fs and flashes fpga
pub fn flash_fpga(rbf_file_path: &Path) -> Result<(), FpgaProgError> {
    if !rbf_file_path.is_file() {
        return Err(FpgaProgError::BadRbfFile);
    }

    let tmp_dir = env::temp_dir();

    let firmware_name = rbf_file_path
        .file_stem()
        .and_then(|file_name| file_name.to_str())
        .ok_or(FpgaProgError::Other(String::from(
            "Cannot get firmware name",
        )))?;

    println!("firmware name: {firmware_name}");

    let _dtso_file_path = create_dtso(&tmp_dir, firmware_name)?;
    println!("dtso_file_path: {_dtso_file_path:?}");
    let dtbo_file_path = create_dtbo(&tmp_dir, firmware_name)?;
    println!("dtbo_file_path: {dtbo_file_path:?}");

    prepare_fs(rbf_file_path, &dtbo_file_path, firmware_name)
}

/// create device tree source (object ? overlay ?) file
fn create_dtso(dir_path: &Path, firmware_name: &str) -> Result<PathBuf, FpgaProgError> {
    let dtso_content = format!(
        "
        /dts-v1/;
        /plugin/;
        /{{
            fragment@0 {{
            target-path = \"/soc/base_fpga_region\";
            __overlay__ {{
                #address-cells = <1>;
                #size-cells = <1>;
                firmware-name = \"{firmware_name}.rbf\";
            }};
            }};
        }};
    "
    );

    // let dtso_file_path = PathBuf::from(format!("{file_name}.dtso"));
    let dtso_file_path = dir_path.join(format!("{firmware_name}.dtso"));
    let mut file = fs::File::create(&dtso_file_path)?;
    file.write_all(dtso_content.as_bytes())?;

    Ok(dtso_file_path)
}

/// generate the device tree binary .dtbo
fn create_dtbo(dir_path: &Path, firmware_name: &str) -> Result<PathBuf, FpgaProgError> {
    let command_status = Command::new("dtc")
        .arg("-O")
        .arg("dtb")
        .arg("-o")
        .arg(format!("{firmware_name}.dtbo"))
        .arg("-b")
        .arg("0")
        .arg("-@")
        .arg(format!("{firmware_name}.dtso"))
        .current_dir(dir_path)
        .status();
    // let cmd = "dtc -O dtb -o blink.dtbo -b 0 -@ blink.dtso";
    if command_status.is_err() {
        return Err(FpgaProgError::DeviceTreeCompileError);
    }

    let dtbo_file_path = dir_path.join(format!("{firmware_name}.dtbo"));
    Ok(dtbo_file_path)
}

fn prepare_fs(
    rbf_file_path: &Path,
    dtbo_file_path: &Path,
    firmware_name: &str,
) -> Result<(), FpgaProgError> {
    let lib_firmware_path = Path::new("/lib/firmware");
    fs::create_dir_all(lib_firmware_path)?;

    fs::copy(
        dtbo_file_path,
        lib_firmware_path.join(format!("{firmware_name}.dtbo")),
    )?;

    fs::copy(
        rbf_file_path,
        lib_firmware_path.join(format!("{firmware_name}.rbf")),
    )?;

    fs::create_dir_all("/config")?;

    let _mount_result = Mount::builder()
        .fstype("configfs")
        .mount("configfs", "/config")?;

    // let _mount_result = Command::new("mount")
    //     .arg("-t")
    //     .arg("configfs")
    //     .arg("configfs")
    //     .arg("/config")
    //     .status()?;

    let overlays_path = PathBuf::from("/config/device-tree/overlays/");
    let firmware_overlays_path = overlays_path.join(firmware_name);

    fs::create_dir(&firmware_overlays_path)?;

    // echo -n "blink.dtbo" > blink/path
    let dtbo_name = dtbo_file_path
        .file_name()
        .ok_or(FpgaProgError::Other(String::from("Cannot get dtbo name")));

    let mut file = fs::File::open(firmware_overlays_path.join("path"))?;
    file.write_fmt(format_args!("{dtbo_name:?}.dtbo"))?;

    Ok(())
}
