/*
type characteristic =
    | IMAGE_FILE_RELOCS_STRIPPED
    | IMAGE_FILE_EXECUTABLE_IMAGE
    | IMAGE_FILE_LINE_NUMS_STRIPPED
    | IMAGE_FILE_LOCAL_SYMS_STRIPPED
    | IMAGE_FILE_AGGRESSIVE_WS_TRIM
    | IMAGE_FILE_LARGE_ADDRESS_AWARE
    | RESERVED
    | IMAGE_FILE_BYTES_REVERSED_LO
    | IMAGE_FILE_32BIT_MACHINE
    | IMAGE_FILE_DEBUG_STRIPPED
    | IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP
    | IMAGE_FILE_NET_RUN_FROM_SWAP
    | IMAGE_FILE_SYSTEM
    | IMAGE_FILE_DLL
    | IMAGE_FILE_UP_SYSTEM_ONLY
    | IMAGE_FILE_BYTES_REVERSED_HI
    | UNKNOWN of int

let get_characteristic =
  function
  | 0x0001 -> IMAGE_FILE_RELOCS_STRIPPED
  | 0x0002 -> IMAGE_FILE_EXECUTABLE_IMAGE
  | 0x0004 -> IMAGE_FILE_LINE_NUMS_STRIPPED
  | 0x0008 -> IMAGE_FILE_LOCAL_SYMS_STRIPPED
  | 0x0010 -> IMAGE_FILE_AGGRESSIVE_WS_TRIM
  | 0x0020 -> IMAGE_FILE_LARGE_ADDRESS_AWARE
  | 0x0040 -> RESERVED
  | 0x0080 -> IMAGE_FILE_BYTES_REVERSED_LO
  | 0x0100 -> IMAGE_FILE_32BIT_MACHINE
  | 0x0200 -> IMAGE_FILE_DEBUG_STRIPPED
  | 0x0400 -> IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP
  | 0x0800 -> IMAGE_FILE_NET_RUN_FROM_SWAP
  | 0x1000 -> IMAGE_FILE_SYSTEM
  | 0x2000 -> IMAGE_FILE_DLL
  | 0x4000 -> IMAGE_FILE_UP_SYSTEM_ONLY
  | 0x8000 -> IMAGE_FILE_BYTES_REVERSED_HI
  | x -> UNKNOWN x

let characteristic_to_string =
  function
  | IMAGE_FILE_RELOCS_STRIPPED -> "IMAGE_FILE_RELOCS_STRIPPED"
  | IMAGE_FILE_EXECUTABLE_IMAGE -> "IMAGE_FILE_EXECUTABLE_IMAGE"
  | IMAGE_FILE_LINE_NUMS_STRIPPED -> "IMAGE_FILE_LINE_NUMS_STRIPPED"
  | IMAGE_FILE_LOCAL_SYMS_STRIPPED -> "IMAGE_FILE_LOCAL_SYMS_STRIPPED"
  | IMAGE_FILE_AGGRESSIVE_WS_TRIM -> "IMAGE_FILE_AGGRESSIVE_WS_TRIM"
  | IMAGE_FILE_LARGE_ADDRESS_AWARE -> "IMAGE_FILE_LARGE_ADDRESS_AWARE"
  | RESERVED -> "RESERVED"
  | IMAGE_FILE_BYTES_REVERSED_LO -> "IMAGE_FILE_BYTES_REVERSED_LO"
  | IMAGE_FILE_32BIT_MACHINE -> "IMAGE_FILE_32BIT_MACHINE"
  | IMAGE_FILE_DEBUG_STRIPPED -> "IMAGE_FILE_DEBUG_STRIPPED"
  | IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP -> "IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP"
  | IMAGE_FILE_NET_RUN_FROM_SWAP -> "IMAGE_FILE_NET_RUN_FROM_SWAP"
  | IMAGE_FILE_SYSTEM -> "IMAGE_FILE_SYSTEM"
  | IMAGE_FILE_DLL -> "IMAGE_FILE_DLL"
  | IMAGE_FILE_UP_SYSTEM_ONLY -> "IMAGE_FILE_UP_SYSTEM_ONLY"
  | IMAGE_FILE_BYTES_REVERSED_HI -> "IMAGE_FILE_BYTES_REVERSED_HI"
  | UNKNOWN x -> Printf.sprintf "UNKNOWN_CHARACTERISTIC 0x%x" x

let is_dll characteristics =
  let characteristic = characteristic_to_int IMAGE_FILE_DLL in
  characteristics land characteristic = characteristic

let has characteristic characteristics =
  let characteristic = characteristic_to_int characteristic in
  characteristics land characteristic = characteristic

(* TODO: this is a mad hack *)
let show_type characteristics =
  if (has IMAGE_FILE_DLL characteristics) then "DLL"
  else if (has IMAGE_FILE_EXECUTABLE_IMAGE characteristics) then "EXE"
  else "MANY"                   (* print all *)
 */
use crate::error;

pub const IMAGE_FILE_RELOCS_STRIPPED: u16 = 0x0001;
pub const IMAGE_FILE_EXECUTABLE_IMAGE: u16 = 0x0002;
pub const IMAGE_FILE_LINE_NUMS_STRIPPED: u16 = 0x0004;
pub const IMAGE_FILE_LOCAL_SYMS_STRIPPED: u16 = 0x0008;
pub const IMAGE_FILE_AGGRESSIVE_WS_TRIM: u16 = 0x0010;
pub const IMAGE_FILE_LARGE_ADDRESS_AWARE: u16 = 0x0020;
pub const RESERVED: u16 = 0x0040;
pub const IMAGE_FILE_BYTES_REVERSED_LO: u16 = 0x0080;
pub const IMAGE_FILE_32BIT_MACHINE: u16 = 0x0100;
pub const IMAGE_FILE_DEBUG_STRIPPED: u16 = 0x0200;
pub const IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP: u16 = 0x0400;
pub const IMAGE_FILE_NET_RUN_FROM_SWAP: u16 = 0x0800;
pub const IMAGE_FILE_SYSTEM: u16 = 0x1000;
pub const IMAGE_FILE_DLL: u16 = 0x2000;
pub const IMAGE_FILE_UP_SYSTEM_ONLY: u16 = 0x4000;
pub const IMAGE_FILE_BYTES_REVERSED_HI: u16 = 0x8000;

fn has_flag(characteristics: u16, flag: u16) -> bool {
    characteristics & flag == flag
}

pub fn is_dll(characteristics: u16) -> bool {
    has_flag(characteristics, IMAGE_FILE_DLL)
}

pub fn is_exe(characteristics: u16) -> bool {
    has_flag(characteristics, IMAGE_FILE_EXECUTABLE_IMAGE)
}

/// Validate the characteristics.
/// 
/// `is_image` being `true` indicates the characteristics is in a `PE`,
/// being `false` indicates the characteristics is in a `Coff`.
pub fn validate(characteristics: u16, is_image: bool) -> error::Result<()> {
    let mut error_messages = vec![];
    if is_image {
        if !has_flag(characteristics, IMAGE_FILE_EXECUTABLE_IMAGE) {
            error_messages.push("If IMAGE_FILE_EXECUTABLE_IMAGE is not set, it indicates a linker error.");
        }
    } else {
        if has_flag(characteristics, IMAGE_FILE_RELOCS_STRIPPED) {
            error_messages.push("IMAGE_FILE_RELOCS_STRIPPED is image only.");
        }
        if has_flag(characteristics, IMAGE_FILE_EXECUTABLE_IMAGE) {
            error_messages.push("IMAGE_FILE_EXECUTABLE_IMAGE is image only.");
        }

    }
    if has_flag(characteristics, IMAGE_FILE_LINE_NUMS_STRIPPED) {
        error_messages.push("IMAGE_FILE_LINE_NUMS_STRIPPED is deprecated and should be zero.")
    }
    if has_flag(characteristics, IMAGE_FILE_LOCAL_SYMS_STRIPPED) {
        error_messages.push("IMAGE_FILE_LOCAL_SYMS_STRIPPED is deprecated and should be zero.")
    }
    if has_flag(characteristics, IMAGE_FILE_AGGRESSIVE_WS_TRIM) {
        error_messages.push("IMAGE_FILE_AGGRESSIVE_WS_TRIM is deprecated and must be zero.");
    }
    if has_flag(characteristics, IMAGE_FILE_BYTES_REVERSED_LO) {
        error_messages.push("IMAGE_FILE_BYTES_REVERSED_LO is deprecated and should be zero.");
    }
    if has_flag(characteristics, IMAGE_FILE_BYTES_REVERSED_HI) {
        error_messages.push("IMAGE_FILE_BYTES_REVERSED_HI is deprecated and should be zero.");
    }
    if error_messages.is_empty() {
        Ok(())
    } else {
        Err(error::Error::Malformed(error_messages.join("\n")))
    }
}
