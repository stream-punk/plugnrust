use std::ffi::c_void;
use std::ffi::CString;
use std::ptr::{null, null_mut};

pub type CBool = i8;
const CTRUE: i8 = 1;
#[allow(dead_code)]
const CFALSE: i8 = 0;

#[export_name = "audioInputsCount"]
pub static mut AUDIO_INPUTS_COUNT: u32 = 0;
#[export_name = "audioOutputsCount"]
pub static mut AUDIO_OUTPUTS_COUNT: u32 = 0;
#[export_name = "auxAudioInputsCount"]
pub static mut AUX_AUDIO_INPUTS_COUNT: u32 = 0;
#[export_name = "auxAudioOutputsCount"]
pub static mut AUX_AUDIO_OUTPUTS_COUNT: u32 = 0;
#[export_name = "maxBlockSize"]
pub static mut MAX_BLOCK_SIZE: i32 = 0;
#[export_name = "sampleRate"]
pub static mut SAMPLE_RATE: f64 = 0.0;
#[export_name = "userDocumentsPath"]
pub static mut USER_DOCUMENTS_PATH: *const i8 = null();
#[export_name = "scriptFilePath"]
pub static mut SCRIPT_FILE_PATH: *const i8 = null();
#[export_name = "scriptDataPath"]
pub static mut SCRIPT_DATA_PATH: *const i8 = null();
#[export_name = "host"]
pub static mut HOST: *mut c_void = null_mut();
#[export_name = "hostPrint"]
pub static mut HOST_PRINT: Option<HostPrintFunc> = None;

macro_rules! declare_array {
    ($len:expr, $array_type:ident, $export_name:expr, $array_name:ident, $pointer_name:ident) => {
        #[export_name = $export_name]
        pub static mut $array_name: $array_type = unsafe {
            {
                $array_type {
                    ptr: $pointer_name.as_ptr() as _,
                    length: $len,
                }
            }
        };
    };
}

pub type HostPrintFunc = unsafe extern "C" fn(_: *mut c_void, _: *const i8) -> ();

pub type MidiQueuePushEventFunc =
    unsafe extern "C" fn(_: *mut MidiQueue, _: *const MidiEvent) -> ();

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CStringArray {
    pub ptr: *mut *const i8,
    pub length: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CDoubleArray {
    pub ptr: *mut f64,
    pub length: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CIntArray {
    pub ptr: *mut i32,
    pub length: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransportInfo {
    pub bpm: f64,
    pub time_sig_top: u32,
    pub time_sig_bottom: u32,
    pub is_playing: CBool,
    pub is_looping: CBool,
    pub is_recording: CBool,
    pub position_in_samples: i64,
    pub position_in_quarter_notes: f64,
    pub position_in_seconds: f64,
    pub current_measure_down_beat: f64,
    pub loop_start: f64,
    pub loop_end: f64,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MidiEvent {
    pub byte0: u8,
    pub byte1: u8,
    pub byte2: u8,
    pub byte3: u8,
    pub time_stamp: f64,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MidiQueue {
    pub events: *mut MidiEvent,
    pub length: u32,
    pub push_event: Option<MidiQueuePushEventFunc>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockData {
    pub samples: *mut *mut f64,
    pub samples_to_process: u32,
    pub input_midi_events: *const MidiQueue,
    pub output_midi_events: *mut MidiQueue,
    pub begin_param_values: *const f64,
    pub end_param_values: *const f64,
    pub transport: *const TransportInfo,
}

#[no_mangle]
pub static mut name: *const u8 = b"Script Name\0".as_ptr();
#[no_mangle]
pub static mut description: *const u8 = b"Script Description\0".as_ptr();

static mut _INPUT_PARAMETERS: [f64; 2] = [0.9; 2];
declare_array!(
    2,
    CStringArray,
    "inputParameters",
    INPUT_PARAMETERS,
    _INPUT_PARAMETERS
);

static mut _INPUT_PARAMETERS_NAMES: [*const u8; 2] = [b"P1\0".as_ptr(), b"P2\0".as_ptr()];
declare_array!(
    2,
    CStringArray,
    "inputParametersNames",
    INPUT_PARAMETERS_NAMES,
    _INPUT_PARAMETERS_NAMES
);

static mut _INPUT_PARAMETERS_ENUMS: [*const u8; 2] = [b"value1;value2\0".as_ptr(), b"\0".as_ptr()];
declare_array!(
    2,
    CStringArray,
    "inputParametersEnums",
    INPUT_PARAMETERS_ENUMS,
    _INPUT_PARAMETERS_ENUMS
);

static mut _INPUT_PARAMETERS_UNITS: [*const u8; 2] = [b"%\0".as_ptr(), b"dB\0".as_ptr()];
declare_array!(
    2,
    CStringArray,
    "inputParametersUnits",
    INPUT_PARAMETERS_UNITS,
    _INPUT_PARAMETERS_UNITS
);

static mut _INPUT_PARAMETERS_FORMATS: [*const u8; 2] = [b".0\0".as_ptr(), b"+.2\0".as_ptr()];
declare_array!(
    2,
    CStringArray,
    "inputParametersFormats",
    INPUT_PARAMETERS_FORMATS,
    _INPUT_PARAMETERS_FORMATS
);

static mut _INPUT_PARAMETERS_MIN: [f64; 2] = [0.0, 0.0];
declare_array!(
    2,
    CDoubleArray,
    "inputParametersMin",
    INPUT_PARAMETERS_MIN,
    _INPUT_PARAMETERS_MIN
);

static mut _INPUT_PARAMETERS_MAX: [f64; 2] = [1.0, 20.0];
declare_array!(
    2,
    CDoubleArray,
    "inputParametersMax",
    INPUT_PARAMETERS_MAX,
    _INPUT_PARAMETERS_MAX
);

static mut _INPUT_PARAMETERS_DEFAULT: [f64; 2] = [0.0, 0.0];
declare_array!(
    2,
    CDoubleArray,
    "inputParametersDefault",
    INPUT_PARAMETERS_DEFAULT,
    _INPUT_PARAMETERS_DEFAULT
);

static mut _INPUT_PARAMETERS_STEPS: [i32; 2] = [2, 0];
declare_array!(
    2,
    CIntArray,
    "inputParametersSteps",
    INPUT_PARAMETERS_STEPS,
    _INPUT_PARAMETERS_STEPS
);

static mut _INPUT_STRINGS: [*const u8; 2] = [b".0\0".as_ptr(), b"+.2\0".as_ptr()];
declare_array!(
    2,
    CStringArray,
    "inputStrings",
    INPUT_STRINGS,
    _INPUT_STRINGS
);

static mut _INPUT_STRINGS_NAMES: [*const u8; 2] = [b"S1\0".as_ptr(), b"S2\0".as_ptr()];
declare_array!(
    2,
    CStringArray,
    "inputStringNames",
    INPUT_STRINGS_NAMES,
    _INPUT_STRINGS_NAMES
);

static mut _OUTPUT_PARAMETERS: [f64; 2] = [0.0, 0.0];
declare_array!(
    2,
    CDoubleArray,
    "outputParameters",
    OUTPUT_PARAMETERS,
    _OUTPUT_PARAMETERS
);

static mut _OUTPUT_PARAMETERS_NAMES: [*const u8; 2] = [b"OUT 1\0".as_ptr(), b"OUT 2\0".as_ptr()];
declare_array!(
    2,
    CStringArray,
    "outputParametersNames",
    OUTPUT_PARAMETERS_NAMES,
    _OUTPUT_PARAMETERS_NAMES
);

static mut _OUTPUT_PARAMETERS_UNITS: [*const u8; 2] = [b"dB\0".as_ptr(), b"dB\0".as_ptr()];
declare_array!(
    2,
    CStringArray,
    "outputParametersUnits",
    OUTPUT_PARAMETERS_UNITS,
    _OUTPUT_PARAMETERS_UNITS
);

static mut _OUTPUT_PARAMETERS_ENUMS: [*const u8; 2] = [b"value1;value2\0".as_ptr(), b"\0".as_ptr()];
declare_array!(
    2,
    CStringArray,
    "outputParametersEnums",
    OUTPUT_PARAMETERS_ENUMS,
    _OUTPUT_PARAMETERS_ENUMS
);

static mut _OUTPUT_PARAMETERS_FORMATS: [*const u8; 2] = [b".0\0".as_ptr(), b"+.2\0".as_ptr()];
declare_array!(
    2,
    CStringArray,
    "outputParametersFormats",
    OUTPUT_PARAMETERS_FORMATS,
    _OUTPUT_PARAMETERS_FORMATS
);

static mut _OUTPUT_PARAMETERS_MIN: [f64; 2] = [0.0, 0.0];
declare_array!(
    2,
    CDoubleArray,
    "outputParametersMin",
    OUTPUT_PARAMETERS_MIN,
    _OUTPUT_PARAMETERS_MIN
);

static mut _OUTPUT_PARAMETERS_MAX: [f64; 2] = [1.0, 20.0];
declare_array!(
    2,
    CDoubleArray,
    "outputParametersMax",
    OUTPUT_PARAMETERS_MAX,
    _OUTPUT_PARAMETERS_MAX
);

static mut _OUTPUT_PARAMETERS_DEFAULT: [f64; 2] = [0.0, 0.0];
declare_array!(
    2,
    CDoubleArray,
    "outputParametersDefault",
    OUTPUT_PARAMETERS_DEFAULT,
    _OUTPUT_PARAMETERS_DEFAULT
);

static mut _OUTPUT_STRINGS: [*const u8; 2] = [b".0\0".as_ptr(), b"+.2\0".as_ptr()];
declare_array!(
    2,
    CStringArray,
    "outputStrings",
    OUTPUT_STRINGS,
    _OUTPUT_STRINGS
);

static mut _OUTPUT_STRINGS_NAMES: [*const u8; 2] = [b"S1\0".as_ptr(), b"S2\0".as_ptr()];
declare_array!(
    2,
    CStringArray,
    "outputStringNames",
    OUTPUT_STRINGS_NAMES,
    _OUTPUT_STRINGS_NAMES
);

static mut _OUTPUT_STRINGS_MAX_LENGTHS: [i32; 2] = [2, 0];
declare_array!(
    2,
    CStringArray,
    "outputStringsMaxLengths",
    OUTPUT_STRINGS_MAX_LENGTHS,
    _OUTPUT_STRINGS_MAX_LENGTHS
);

pub fn print(message: &str) {
    let cstr = CString::new(message).unwrap();
    unsafe {
        if let Some(host_print) = HOST_PRINT {
            host_print(HOST, cstr.as_ptr());
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn initialize() -> CBool {
    CTRUE
}

#[no_mangle]
pub unsafe extern "C" fn reset() {}

#[export_name = "getTailSize"]
pub unsafe extern "C" fn get_tail_size() -> i32 {
    0
}

#[export_name = "getLatency"]
pub unsafe extern "C" fn get_latency() -> i32 {
    0
}

#[export_name = "updateInputParameters"]
pub unsafe extern "C" fn update_input_parameters() {}

#[export_name = "processBlock"]
pub unsafe extern "C" fn process_block(data: *mut BlockData) {
    static mut V: f64 = 0.0;
    for i in 0..(*data).samples_to_process {
        V += 440.0 / 44100.0;
        V %= 1.0;
        for ch in 0..AUDIO_OUTPUTS_COUNT {
            *(*(*data).samples.offset(ch as _)).offset(i as _) = V - 0.5;
            *(*(*data).samples.offset(ch as _)).offset(i as _) = V - 0.5;
        }
    }
}

#[allow(unused_variables)]
#[export_name = "updateInputParametersForBlock"]
pub unsafe extern "C" fn update_input_parameters_for_block(info: *const TransportInfo) {}

#[export_name = "computeOutputData"]
pub unsafe extern "C" fn compute_output_data() {}

#[no_mangle]
pub unsafe extern "C" fn shutdown() {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn check_sizes() {
        assert_eq!(size_of::<BlockData>(), 56);
        assert_eq!(size_of::<MidiEvent>(), 16);
        assert_eq!(size_of::<MidiQueue>(), 24);
        assert_eq!(size_of::<TransportInfo>(), 72);
        assert_eq!(size_of::<CDoubleArray>(), 16);
        assert_eq!(size_of::<CIntArray>(), 16);
        assert_eq!(size_of::<CStringArray>(), 16);
    }
}
