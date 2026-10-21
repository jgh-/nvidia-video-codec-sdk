//! Defines `ENCODE_API`, which is a lazy static of [`EncodeAPI`].

use core::ffi::{c_int, c_void};

use super::result::{EncodeError, ErrorKind};

use crate::sys::nvEncodeAPI::{
    NvEncodeAPICreateInstance,
    NvEncodeAPIGetMaxSupportedVersion,
    GUID,
    NVENCAPI_MAJOR_VERSION,
    NVENCAPI_MINOR_VERSION,
    NVENCSTATUS,
    NV_ENCODE_API_FUNCTION_LIST,
    NV_ENC_BUFFER_FORMAT,
    NV_ENC_CAPS_PARAM,
    NV_ENC_CREATE_BITSTREAM_BUFFER,
    NV_ENC_CREATE_INPUT_BUFFER,
    NV_ENC_CREATE_MV_BUFFER,
    NV_ENC_CUSTREAM_PTR,
    NV_ENC_EVENT_PARAMS,
    NV_ENC_INITIALIZE_PARAMS,
    NV_ENC_INPUT_PTR,
    NV_ENC_LOCK_BITSTREAM,
    NV_ENC_LOCK_INPUT_BUFFER,
    NV_ENC_LOOKAHEAD_PIC_PARAMS,
    NV_ENC_MAP_INPUT_RESOURCE,
    NV_ENC_MEONLY_PARAMS,
    NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS,
    NV_ENC_OUTPUT_PTR,
    NV_ENC_PIC_PARAMS,
    NV_ENC_PRESET_CONFIG,
    NV_ENC_RECONFIGURE_PARAMS,
    NV_ENC_REGISTERED_PTR,
    NV_ENC_REGISTER_RESOURCE,
    NV_ENC_RESTORE_ENCODER_STATE_PARAMS,
    NV_ENC_SEQUENCE_PARAM_PAYLOAD,
    NV_ENC_STAT,
    NV_ENC_TUNING_INFO,
};

lazy_static! {
    /// A lazy static for the Encoder API.
    ///
    /// You should not interact with this directly.
    /// [`Encoder`](crate::Encoder) exposes much of the functionality and provides a nicer API.
    pub static ref ENCODE_API: Result<EncodeAPI, EncodeError> =
        EncodeAPI::new();
}

/// The encoder API, or the reason it could not be created.
///
/// Callers get a normal error instead of a panic when the driver is too old.
pub fn api() -> Result<&'static EncodeAPI, EncodeError> {
    match &*ENCODE_API {
        Ok(api) => Ok(api),
        Err(error) => Err(error.clone()),
    }
}

// Function type aliases to shorten later definitions.
type OpenEncodeSession = unsafe extern "C" fn(*mut c_void, u32, *mut *mut c_void) -> NVENCSTATUS;
type GetEncodeGUIDCount = unsafe extern "C" fn(*mut c_void, *mut u32) -> NVENCSTATUS;
type GetEncodeGUIDs = unsafe extern "C" fn(*mut c_void, *mut GUID, u32, *mut u32) -> NVENCSTATUS;
type GetInputFormatCount = unsafe extern "C" fn(*mut c_void, GUID, *mut u32) -> NVENCSTATUS;
type GetInputFormats = unsafe extern "C" fn(
    *mut c_void,
    GUID,
    *mut NV_ENC_BUFFER_FORMAT,
    u32,
    *mut u32,
) -> NVENCSTATUS;
type GetEncodeCaps =
    unsafe extern "C" fn(*mut c_void, GUID, *mut NV_ENC_CAPS_PARAM, *mut c_int) -> NVENCSTATUS;
type GetEncodePresetCount = unsafe extern "C" fn(*mut c_void, GUID, *mut u32) -> NVENCSTATUS;
type GetEncodePresetGUIDs =
    unsafe extern "C" fn(*mut c_void, GUID, *mut GUID, u32, *mut u32) -> NVENCSTATUS;
type GetEncodeProfileGUIDCount = GetEncodePresetCount;
type GetEncodeProfileGUIDs = GetEncodePresetGUIDs;
type GetEncodePresetConfig =
    unsafe extern "C" fn(*mut c_void, GUID, GUID, *mut NV_ENC_PRESET_CONFIG) -> NVENCSTATUS;
type GetEncodePresetConfigEx = unsafe extern "C" fn(
    *mut c_void,
    GUID,
    GUID,
    NV_ENC_TUNING_INFO,
    *mut NV_ENC_PRESET_CONFIG,
) -> NVENCSTATUS;
type InitializeEncoder =
    unsafe extern "C" fn(*mut c_void, *mut NV_ENC_INITIALIZE_PARAMS) -> NVENCSTATUS;
type CreateInputBuffer =
    unsafe extern "C" fn(*mut c_void, *mut NV_ENC_CREATE_INPUT_BUFFER) -> NVENCSTATUS;
type DestroyInputBuffer = unsafe extern "C" fn(*mut c_void, NV_ENC_INPUT_PTR) -> NVENCSTATUS;
type CreateBitstreamBuffer =
    unsafe extern "C" fn(*mut c_void, *mut NV_ENC_CREATE_BITSTREAM_BUFFER) -> NVENCSTATUS;
type DestroyBitstreamBuffer = unsafe extern "C" fn(*mut c_void, NV_ENC_OUTPUT_PTR) -> NVENCSTATUS;
type EncodePicture = unsafe extern "C" fn(*mut c_void, *mut NV_ENC_PIC_PARAMS) -> NVENCSTATUS;
type LockBitstream = unsafe extern "C" fn(*mut c_void, *mut NV_ENC_LOCK_BITSTREAM) -> NVENCSTATUS;
type UnlockBitstream = unsafe extern "C" fn(*mut c_void, NV_ENC_OUTPUT_PTR) -> NVENCSTATUS;
type LockInputBuffer =
    unsafe extern "C" fn(*mut c_void, *mut NV_ENC_LOCK_INPUT_BUFFER) -> NVENCSTATUS;
type UnlockInputBuffer = unsafe extern "C" fn(*mut c_void, NV_ENC_INPUT_PTR) -> NVENCSTATUS;
type GetEncodeStats = unsafe extern "C" fn(*mut c_void, *mut NV_ENC_STAT) -> NVENCSTATUS;
type GetSequenceParams =
    unsafe extern "C" fn(*mut c_void, *mut NV_ENC_SEQUENCE_PARAM_PAYLOAD) -> NVENCSTATUS;
type RegisterAsyncEvent =
    unsafe extern "C" fn(*mut c_void, *mut NV_ENC_EVENT_PARAMS) -> NVENCSTATUS;
type UnregisterAsyncEvent =
    unsafe extern "C" fn(*mut c_void, *mut NV_ENC_EVENT_PARAMS) -> NVENCSTATUS;
type MapInputResource =
    unsafe extern "C" fn(*mut c_void, *mut NV_ENC_MAP_INPUT_RESOURCE) -> NVENCSTATUS;
type UnmapInputResource = unsafe extern "C" fn(*mut c_void, NV_ENC_INPUT_PTR) -> NVENCSTATUS;
type DestroyEncoder = unsafe extern "C" fn(encoder: *mut c_void) -> NVENCSTATUS;
type InvalidateRefFrames = unsafe extern "C" fn(*mut c_void, u64) -> NVENCSTATUS;
type OpenEncodeSessionEx = unsafe extern "C" fn(
    *mut NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS,
    *mut *mut c_void,
) -> NVENCSTATUS;
type RegisterResource =
    unsafe extern "C" fn(*mut c_void, *mut NV_ENC_REGISTER_RESOURCE) -> NVENCSTATUS;
type UnregisterResource = unsafe extern "C" fn(*mut c_void, NV_ENC_REGISTERED_PTR) -> NVENCSTATUS;
type ReconfigureEncoder =
    unsafe extern "C" fn(*mut c_void, *mut NV_ENC_RECONFIGURE_PARAMS) -> NVENCSTATUS;
type CreateMVBuffer =
    unsafe extern "C" fn(*mut c_void, *mut NV_ENC_CREATE_MV_BUFFER) -> NVENCSTATUS;
type DestroyMVBuffer = unsafe extern "C" fn(*mut c_void, NV_ENC_OUTPUT_PTR) -> NVENCSTATUS;
type RunMotionEstimationOnly =
    unsafe extern "C" fn(*mut c_void, *mut NV_ENC_MEONLY_PARAMS) -> NVENCSTATUS;
type GetLastErrorString = unsafe extern "C" fn(encoder: *mut c_void) -> *const ::core::ffi::c_char;
type SetIOCudaStreams =
    unsafe extern "C" fn(*mut c_void, NV_ENC_CUSTREAM_PTR, NV_ENC_CUSTREAM_PTR) -> NVENCSTATUS;
type GetSequenceParamEx = unsafe extern "C" fn(
    *mut c_void,
    *mut NV_ENC_INITIALIZE_PARAMS,
    *mut NV_ENC_SEQUENCE_PARAM_PAYLOAD,
) -> NVENCSTATUS;
type RestoreEncoderState =
    unsafe extern "C" fn(*mut c_void, *mut NV_ENC_RESTORE_ENCODER_STATE_PARAMS) -> NVENCSTATUS;
type LookaheadPicture =
    unsafe extern "C" fn(*mut c_void, *mut NV_ENC_LOOKAHEAD_PIC_PARAMS) -> NVENCSTATUS;

/// An instance of the `NvEncodeAPI` interface, containing function pointers
/// which should be used to interface with the rest of the Encoder API.
#[allow(dead_code, missing_docs)]
#[derive(Debug, Clone)]
pub struct EncodeAPI {
    #[doc(alias = "NvEncOpenEncodeSession")]
    pub open_encode_session: OpenEncodeSession,
    #[doc(alias = "NvEncOpenEncodeSessionEx")]
    pub open_encode_session_ex: OpenEncodeSessionEx,
    #[doc(alias = "NvEncInitializeEncoder")]
    pub initialize_encoder: InitializeEncoder,
    #[doc(alias = "NvEncReconfigureEncoder")]
    pub reconfigure_encoder: ReconfigureEncoder,
    #[doc(alias = "NvEncDestroyEncoder")]
    pub destroy_encoder: DestroyEncoder,
    #[doc(alias = "NvEncGetEncodeGuidCount")]
    pub get_encode_guid_count: GetEncodeGUIDCount,
    #[doc(alias = "NvEncGetEncodeGUIDs")]
    pub get_encode_guids: GetEncodeGUIDs,
    #[doc(alias = "NvEncGetEncodeProfileGuidCount")]
    pub get_encode_profile_guid_count: GetEncodeProfileGUIDCount,
    #[doc(alias = "NvEncGetEncodeProfileGUIDs")]
    pub get_encode_profile_guids: GetEncodeProfileGUIDs,
    #[doc(alias = "NvEncGetInputFormatCount")]
    pub get_input_format_count: GetInputFormatCount,
    #[doc(alias = "NvEncGetInputFormats")]
    pub get_input_formats: GetInputFormats,
    #[doc(alias = "NvEncGetEncodePresetCount")]
    pub get_encode_preset_count: GetEncodePresetCount,
    #[doc(alias = "NvEncGetEncodePresetGUIDs")]
    pub get_encode_preset_guids: GetEncodePresetGUIDs,
    #[doc(alias = "NvEncGetEncodePresetConfig")]
    pub get_encode_preset_config: GetEncodePresetConfig,
    #[doc(alias = "NvEncGetEncodePresetConfigEx")]
    pub get_encode_preset_config_ex: GetEncodePresetConfigEx,
    #[doc(alias = "NvEncGetEncodeCaps")]
    pub get_encode_caps: GetEncodeCaps,
    #[doc(alias = "NvEncCreateInputBuffer")]
    pub create_input_buffer: CreateInputBuffer,
    #[doc(alias = "NvEncDestroyInputBuffer")]
    pub destroy_input_buffer: DestroyInputBuffer,
    #[doc(alias = "NvLockInputBuffer")]
    pub lock_input_buffer: LockInputBuffer,
    #[doc(alias = "NvUnlockInputBuffer")]
    pub unlock_input_buffer: UnlockInputBuffer,
    #[doc(alias = "NvEncCreateBitstreamBuffer")]
    pub create_bitstream_buffer: CreateBitstreamBuffer,
    #[doc(alias = "NvEncDestroyBitstreamBuffer")]
    pub destroy_bitstream_buffer: DestroyBitstreamBuffer,
    #[doc(alias = "NvEncLockBitstream")]
    pub lock_bitstream: LockBitstream,
    #[doc(alias = "NvEncUnlockBitstream")]
    pub unlock_bitstream: UnlockBitstream,
    #[doc(alias = "NvEncMapInputResource")]
    pub map_input_resource: MapInputResource,
    #[doc(alias = "NvEncUnmapInputResource")]
    pub unmap_input_resource: UnmapInputResource,
    #[doc(alias = "NvEncRegisterResource")]
    pub register_resource: RegisterResource,
    #[doc(alias = "NvEncUnregisterResource")]
    pub unregister_resource: UnregisterResource,
    #[doc(alias = "NvEncCreateMVBuffer")]
    pub create_mv_buffer: CreateMVBuffer,
    #[doc(alias = "NvEncDestroyMVBuffer")]
    pub destroy_mv_buffer: DestroyMVBuffer,
    #[doc(alias = "NvEncEncodePicture")]
    pub encode_picture: EncodePicture,
    #[doc(alias = "NvEncGetEncodeStats")]
    pub get_encode_stats: GetEncodeStats,
    #[doc(alias = "NvEncGetSequenceParams")]
    pub get_sequence_params: GetSequenceParams,
    #[doc(alias = "NvEncGetSequenceParamEx")]
    pub get_sequence_param_ex: GetSequenceParamEx,
    #[doc(alias = "NvEncRegisterAsyncEvent")]
    pub register_async_event: RegisterAsyncEvent,
    #[doc(alias = "NvEncUnregisterAsyncEvent")]
    pub unregister_async_event: UnregisterAsyncEvent,
    #[doc(alias = "NvEncInvalidateRefFrames")]
    pub invalidate_ref_frames: InvalidateRefFrames,
    #[doc(alias = "NvEncRunMotionEstimationOnly")]
    pub run_motion_estimation_only: RunMotionEstimationOnly,
    #[doc(alias = "NvEncGetLastErrorString")]
    pub get_last_error_string: GetLastErrorString,
    #[doc(alias = "NvEncSetIOCudaStreams")]
    pub set_io_cuda_streams: SetIOCudaStreams,
    #[doc(alias = "NvEncRestoreEncoderState")]
    pub restore_encoder_state: RestoreEncoderState,
    #[doc(alias = "NvEncLookaheadPicture")]
    pub lookahead_picture: LookaheadPicture,
}

/// The API version in use, in `NVENCAPI_VERSION` encoding
/// (`major | minor << 24`).
///
/// The version is *checked*, never negotiated down. Every struct version in this
/// crate (`NV_ENC_*_VER`) is derived from the header version, so speaking an
/// older API while sending newer structures is incoherent: the driver accepts
/// the version word and then rejects the session with `InvalidEncoderDevice`,
/// which reads like a device problem and is not. Supporting an older driver
/// means generating bindings from older headers, which is a build choice, not a
/// runtime one.
static NEGOTIATED_VERSION: std::sync::OnceLock<u32> = std::sync::OnceLock::new();

/// API version actually in use.
pub fn negotiated_api_version() -> u32 {
    *NEGOTIATED_VERSION.get_or_init(|| max_supported_to_version_word(header_version()))
}

/// `NvEncodeAPIGetMaxSupportedVersion` reports `major << 4 | minor`, while
/// `NVENCAPI_VERSION` (used by the function list and by
/// `NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS`) uses `major | minor << 24`.
fn max_supported_to_version_word(max_supported: u32) -> u32 {
    (max_supported >> 4) | ((max_supported & 0b1111) << 24)
}

fn header_version() -> u32 {
    (NVENCAPI_MAJOR_VERSION << 4) | NVENCAPI_MINOR_VERSION
}

/// Reject a driver that is older than the headers, with an error rather than a
/// panic. A panic here aborts inside the encoder thread and reaches callers as
/// "worker exited during initialization", which names nothing actionable.
fn check_driver_version(max_supported_version: u32) -> Result<u32, EncodeError> {
    let header = header_version();
    if max_supported_version >= header {
        return Ok(max_supported_to_version_word(header));
    }
    Err(EncodeError::new(
        ErrorKind::UnsupportedDevice,
        Some(format!(
            "the installed driver supports NVENC API {}.{}, but this build was generated \
             against {}.{}: update the driver, or rebuild the bindings against older headers",
            max_supported_version >> 4,
            max_supported_version & 0b1111,
            header >> 4,
            header & 0b1111,
        )),
    ))
}

impl EncodeAPI {
    fn new() -> Result<Self, EncodeError> {
        const MSG: &str = "The API instance should populate the whole function list.";

        // Check that the driver max supported version matches the version
        // from the header files. If they do not match, the bindings should be updated.
        let mut version = 0;
        unsafe { NvEncodeAPIGetMaxSupportedVersion(&mut version) }.result_without_string()?;
        let api_version = check_driver_version(version)?;
        let _ = NEGOTIATED_VERSION.set(api_version);

        // Create empty function buffer, telling the driver which version we
        // settled on rather than the one the headers were generated from.
        let mut function_list = NV_ENCODE_API_FUNCTION_LIST {
            version: api_version | (std::mem::size_of::<NV_ENCODE_API_FUNCTION_LIST>() as u32) << 16
                | (0x7 << 28),
            ..Default::default()
        };
        // Create Encode API Instance (populate function buffer).
        unsafe { NvEncodeAPICreateInstance(&mut function_list) }.result_without_string()?;

        Ok(Self {
            open_encode_session: function_list.nvEncOpenEncodeSession.expect(MSG),
            open_encode_session_ex: function_list.nvEncOpenEncodeSessionEx.expect(MSG),
            initialize_encoder: function_list.nvEncInitializeEncoder.expect(MSG),
            reconfigure_encoder: function_list.nvEncReconfigureEncoder.expect(MSG),
            destroy_encoder: function_list.nvEncDestroyEncoder.expect(MSG),
            get_encode_guid_count: function_list.nvEncGetEncodeGUIDCount.expect(MSG),
            get_encode_guids: function_list.nvEncGetEncodeGUIDs.expect(MSG),
            get_encode_profile_guid_count: function_list.nvEncGetEncodeProfileGUIDCount.expect(MSG),
            get_encode_profile_guids: function_list.nvEncGetEncodeProfileGUIDs.expect(MSG),
            get_input_format_count: function_list.nvEncGetInputFormatCount.expect(MSG),
            get_input_formats: function_list.nvEncGetInputFormats.expect(MSG),
            get_encode_preset_count: function_list.nvEncGetEncodePresetCount.expect(MSG),
            get_encode_preset_guids: function_list.nvEncGetEncodePresetGUIDs.expect(MSG),
            get_encode_preset_config: function_list.nvEncGetEncodePresetConfig.expect(MSG),
            get_encode_preset_config_ex: function_list.nvEncGetEncodePresetConfigEx.expect(MSG),
            get_encode_caps: function_list.nvEncGetEncodeCaps.expect(MSG),
            create_input_buffer: function_list.nvEncCreateInputBuffer.expect(MSG),
            destroy_input_buffer: function_list.nvEncDestroyInputBuffer.expect(MSG),
            lock_input_buffer: function_list.nvEncLockInputBuffer.expect(MSG),
            unlock_input_buffer: function_list.nvEncUnlockInputBuffer.expect(MSG),
            create_bitstream_buffer: function_list.nvEncCreateBitstreamBuffer.expect(MSG),
            destroy_bitstream_buffer: function_list.nvEncDestroyBitstreamBuffer.expect(MSG),
            lock_bitstream: function_list.nvEncLockBitstream.expect(MSG),
            unlock_bitstream: function_list.nvEncUnlockBitstream.expect(MSG),
            map_input_resource: function_list.nvEncMapInputResource.expect(MSG),
            unmap_input_resource: function_list.nvEncUnmapInputResource.expect(MSG),
            register_resource: function_list.nvEncRegisterResource.expect(MSG),
            unregister_resource: function_list.nvEncUnregisterResource.expect(MSG),
            create_mv_buffer: function_list.nvEncCreateMVBuffer.expect(MSG),
            destroy_mv_buffer: function_list.nvEncDestroyMVBuffer.expect(MSG),
            encode_picture: function_list.nvEncEncodePicture.expect(MSG),
            get_encode_stats: function_list.nvEncGetEncodeStats.expect(MSG),
            get_sequence_params: function_list.nvEncGetSequenceParams.expect(MSG),
            get_sequence_param_ex: function_list.nvEncGetSequenceParamEx.expect(MSG),
            register_async_event: function_list.nvEncRegisterAsyncEvent.expect(MSG),
            unregister_async_event: function_list.nvEncUnregisterAsyncEvent.expect(MSG),
            invalidate_ref_frames: function_list.nvEncInvalidateRefFrames.expect(MSG),
            run_motion_estimation_only: function_list.nvEncRunMotionEstimationOnly.expect(MSG),
            get_last_error_string: function_list.nvEncGetLastErrorString.expect(MSG),
            set_io_cuda_streams: function_list.nvEncSetIOCudaStreams.expect(MSG),
            restore_encoder_state: function_list.nvEncRestoreEncoderState.expect(MSG),
            lookahead_picture: function_list.nvEncLookaheadPicture.expect(MSG),
        })
    }
}
