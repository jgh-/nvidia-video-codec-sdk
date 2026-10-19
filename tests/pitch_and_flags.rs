//! Regression tests for the public `encode_pic_flags` field and the input-buffer
//! pitch accessors.
//!
//! Both require a real NVENC-capable GPU. Run with:
//!
//! ```sh
//! NVIDIA_VIDEO_CODEC_SDK_PATH=/usr/lib/wsl/lib cargo test --test pitch_and_flags
//! ```

use std::sync::Arc;

use cudarc::driver::CudaContext;
use nvidia_video_codec_sdk::{
    sys::nvEncodeAPI::{
        NV_ENC_BUFFER_FORMAT, NV_ENC_CODEC_AV1_GUID, NV_ENC_PIC_FLAGS, NV_ENC_PIC_TYPE,
        NV_ENC_PRESET_P4_GUID, NV_ENC_TUNING_INFO,
    },
    EncodePictureParams, Encoder, EncoderInitParams, Session,
};

const FRAME_INTERVAL_P: i32 = 1; // no B-frames: one output per input, no reordering
const GOP: u32 = 1000; // caller drives the only forced IDR

fn open_av1_session(
    width: u32,
    height: u32,
) -> (
    &'static Session,
    nvidia_video_codec_sdk::Buffer<'static>,
    nvidia_video_codec_sdk::Bitstream<'static>,
) {
    let ctx: Arc<CudaContext> = CudaContext::new(0).expect("CUDA should be installed.");
    let encoder = Encoder::initialize_with_cuda(ctx).expect("NVENC should initialize.");
    let mut preset = encoder
        .get_preset_config(
            NV_ENC_CODEC_AV1_GUID,
            NV_ENC_PRESET_P4_GUID,
            NV_ENC_TUNING_INFO::NV_ENC_TUNING_INFO_HIGH_QUALITY,
        )
        .expect("AV1 preset should be available.");
    preset.presetCfg.gopLength = GOP;
    preset.presetCfg.frameIntervalP = FRAME_INTERVAL_P;
    let mut init = EncoderInitParams::new(NV_ENC_CODEC_AV1_GUID, width, height);
    init.preset_guid(NV_ENC_PRESET_P4_GUID)
        .tuning_info(NV_ENC_TUNING_INFO::NV_ENC_TUNING_INFO_HIGH_QUALITY)
        .framerate(24, 1)
        .encode_config(&mut preset.presetCfg)
        .enable_picture_type_decision();
    let session = encoder
        .start_session(NV_ENC_BUFFER_FORMAT::NV_ENC_BUFFER_FORMAT_NV12, init)
        .expect("AV1 NV12 session should start.");
    // The session is intentionally leaked so the returned buffer/bitstream can
    // carry 'static lifetimes out of this helper; the test process exits shortly
    // after and the driver reclaims the session.
    let session: &'static Session = Box::leak(Box::new(session));
    let input = session.create_input_buffer().expect("input buffer");
    let output = session.create_output_bitstream().expect("bitstream");
    // `session` is leaked, so it lives for the rest of the process and the
    // 'static borrow on both handles is satisfied.
    (session, input, output)
}

/// A flat mid-grey NV12 frame. Content does not matter for either assertion.
fn nv12_frame(width: u32, height: u32) -> Vec<u8> {
    vec![128u8; (width * height * 3 / 2) as usize]
}

#[test]
fn pitch_is_observable_and_consistent() {
    // 834 is deliberately not a multiple of any likely pitch alignment.
    let (session, mut input, mut output) = open_av1_session(834, 480);
    // Buffer::pitch is the width this wrapper passes as inputPitch; the
    // driver's allocation stride is only visible on the lock and can be larger.
    let width_pitch = input.pitch();
    assert_eq!(width_pitch, 834, "Buffer::pitch is the frame width");
    let stride = {
        let lock = input.lock().expect("input lock");
        lock.pitch()
    };
    assert!(
        stride >= width_pitch,
        "allocation stride {stride} must cover the width {width_pitch}"
    );
    assert_eq!(stride % 2, 0, "NV12 stride must be even for chroma pairs");
    // A session with no submitted frames is dropped without draining; encoding
    // one frame keeps the API contract honest (EOS on drop needs a free encoder).
    let frame = nv12_frame(834, 480);
    unsafe { input.lock().expect("input lock").write(&frame) };
    session
        .encode_picture(
            &mut input,
            &mut output,
            EncodePictureParams {
                input_timestamp: 0,
                ..Default::default()
            },
        )
        .expect("first frame encodes");
}

#[test]
fn force_idr_flag_produces_an_idr() {
    let (session, mut input, mut output) = open_av1_session(320, 240);
    let frame = nv12_frame(320, 240);
    let mut types = Vec::new();

    for i in 0..4u64 {
        unsafe { input.lock().expect("input lock").write(&frame) };
        let flags = if i == 3 {
            NV_ENC_PIC_FLAGS::NV_ENC_PIC_FLAG_FORCEIDR as u32
        } else {
            0
        };
        session
            .encode_picture(
                &mut input,
                &mut output,
                EncodePictureParams {
                    input_timestamp: i * 3750,
                    encode_pic_flags: flags,
                    ..Default::default()
                },
            )
            .expect("frame encodes");
        let lock = output.lock().expect("bitstream lock");
        types.push(lock.picture_type());
    }

    assert_eq!(
        types[0],
        NV_ENC_PIC_TYPE::NV_ENC_PIC_TYPE_IDR,
        "first frame must be an IDR"
    );
    assert_eq!(
        types[3],
        NV_ENC_PIC_TYPE::NV_ENC_PIC_TYPE_IDR,
        "forced frame must be an IDR, got {:?}",
        types[3]
    );
    assert!(
        types[1..3]
            .iter()
            .all(|t| *t != NV_ENC_PIC_TYPE::NV_ENC_PIC_TYPE_IDR),
        "unforced frames must not be IDRs, got {types:?}"
    );
}
