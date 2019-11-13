#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
extern crate c2rust_bitfields;
extern crate libc;

use serde_derive::{Serialize, Deserialize};

use std::env;

use crate::rdo_tables::*;

use c2rust_bitfields::BitfieldStruct;
extern {
  pub type _IO_wide_data;
  pub type _IO_codecvt;
  pub type _IO_marker;
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn fwrite(
    _: *const libc::c_void, _: libc::c_ulong, _: libc::c_ulong, _: *mut FILE
  ) -> libc::c_ulong;
  #[no_mangle]
  fn fread(
    _: *mut libc::c_void, _: libc::c_ulong, _: libc::c_ulong, _: *mut FILE
  ) -> libc::c_ulong;
  #[no_mangle]
  fn ldexp(_: libc::c_double, _: libc::c_int) -> libc::c_double;
  #[no_mangle]
  fn sqrt(_: libc::c_double) -> libc::c_double;
  #[no_mangle]
  fn memcpy(
    _: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(
    _: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn abs(_: libc::c_int) -> libc::c_int;
  #[no_mangle]
  static OC_DCT_TOKEN_EXTRA_BITS: [libc::c_uchar; 32];
  #[no_mangle]
  static mut OC_MODE_RD_SATD: [[[[oc_mode_rd; 24]; 2]; 3]; 8];
  #[no_mangle]
  static mut OC_MODE_RD_SAD: [[[[oc_mode_rd; 24]; 2]; 3]; 8];
}
pub type size_t = libc::c_ulong;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
  pub _flags: libc::c_int,
  pub _IO_read_ptr: *mut libc::c_char,
  pub _IO_read_end: *mut libc::c_char,
  pub _IO_read_base: *mut libc::c_char,
  pub _IO_write_base: *mut libc::c_char,
  pub _IO_write_ptr: *mut libc::c_char,
  pub _IO_write_end: *mut libc::c_char,
  pub _IO_buf_base: *mut libc::c_char,
  pub _IO_buf_end: *mut libc::c_char,
  pub _IO_save_base: *mut libc::c_char,
  pub _IO_backup_base: *mut libc::c_char,
  pub _IO_save_end: *mut libc::c_char,
  pub _markers: *mut _IO_marker,
  pub _chain: *mut _IO_FILE,
  pub _fileno: libc::c_int,
  pub _flags2: libc::c_int,
  pub _old_offset: __off_t,
  pub _cur_column: libc::c_ushort,
  pub _vtable_offset: libc::c_schar,
  pub _shortbuf: [libc::c_char; 1],
  pub _lock: *mut libc::c_void,
  pub _offset: __off64_t,
  pub _codecvt: *mut _IO_codecvt,
  pub _wide_data: *mut _IO_wide_data,
  pub _freeres_list: *mut _IO_FILE,
  pub _freeres_buf: *mut libc::c_void,
  pub __pad5: size_t,
  pub _mode: libc::c_int,
  pub _unused2: [libc::c_char; 20]
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ptrdiff_t = libc::c_long;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type ogg_int16_t = int16_t;
pub type ogg_uint16_t = uint16_t;
pub type ogg_int32_t = int32_t;
pub type ogg_uint32_t = uint32_t;
pub type ogg_int64_t = int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oggpack_buffer {
  pub endbyte: libc::c_long,
  pub endbit: libc::c_int,
  pub buffer: *mut libc::c_uchar,
  pub ptr: *mut libc::c_uchar,
  pub storage: libc::c_long
}
/* *******************************************************************
*                                                                  *
* THIS FILE IS PART OF THE OggTheora SOFTWARE CODEC SOURCE CODE.   *
* USE, DISTRIBUTION AND REPRODUCTION OF THIS LIBRARY SOURCE IS     *
* GOVERNED BY A BSD-STYLE SOURCE LICENSE INCLUDED WITH THIS SOURCE *
* IN 'COPYING'. PLEASE READ THESE TERMS BEFORE DISTRIBUTING.       *
*                                                                  *
* THE Theora SOURCE CODE IS COPYRIGHT (C) 2002-2009                *
* by the Xiph.Org Foundation http://www.xiph.org/                  *
*                                                                  *
********************************************************************

 function:
 last mod: $Id: theora.h,v 1.8 2004/03/15 22:17:32 derf Exp $

********************************************************************/
/* *\mainpage
 *
 * \section intro Introduction
 *
 * This is the documentation for the <tt>libtheora</tt> C API.
 *
 * The \c libtheora package is the current reference
 * implementation for <a href="http://www.theora.org/">Theora</a>, a free,
 * patent-unencumbered video codec.
 * Theora is derived from On2's VP3 codec with additional features and
 *  integration with Ogg multimedia formats by
 *  <a href="http://www.xiph.org/">the Xiph.Org Foundation</a>.
 * Complete documentation of the format itself is available in
 * <a href="http://www.theora.org/doc/Theora.pdf">the Theora
 *  specification</a>.
 *
 * \section Organization
 *
 * The functions documented here are divided between two
 * separate libraries:
 * - \c libtheoraenc contains the encoder interface,
 *   described in \ref encfuncs.
 * - \c libtheoradec contains the decoder interface,
 *   described in \ref decfuncs, \n
 *   and additional \ref basefuncs.
 *
 * New code should link to \c libtheoradec. If using encoder
 * features, it must also link to \c libtheoraenc.
 *
 * During initial development, prior to the 1.0 release,
 * \c libtheora exported a different \ref oldfuncs which
 * combined both encode and decode functions.
 * In general, legacy API symbols can be indentified
 * by their \c theora_ or \c OC_ namespace prefixes.
 * The current API uses \c th_ or \c TH_ instead.
 *
 * While deprecated, \c libtheoraenc and \c libtheoradec
 * together export the legacy api as well at the one documented above.
 * Likewise, the legacy \c libtheora included with this package
 * exports the new 1.x API. Older code and build scripts can therefore
 * but updated independently to the current scheme.
 */
/* *\file
 * The shared <tt>libtheoradec</tt> and <tt>libtheoraenc</tt> C API.
 * You don't need to include this directly.*/
/* *\name Return codes*/
/*@{*/
/* *An invalid pointer was provided.*/
/* *An invalid argument was provided.*/
/* *The contents of the header were incomplete, invalid, or unexpected.*/
/* *The header does not belong to a Theora stream.*/
/* *The bitstream version is too high.*/
/* *The specified function is not implemented.*/
/* *There were errors in the video data packet.*/
/* *The decoded packet represented a dropped frame.
The player can continue to display the current frame, as the contents of the
 decoded frame buffer have not changed.*/
/*@}*/
/* *The currently defined color space tags.
 * See <a href="http://www.theora.org/doc/Theora.pdf">the Theora
 *  specification</a>, Chapter 4, for exact details on the meaning
 *  of each of these color spaces.*/
pub type th_colorspace = libc::c_uint;
/* *The total number of currently defined color spaces.*/
pub const TH_CS_NSPACES: th_colorspace = 3;
/* *A color space designed for PAL/SECAM content.*/
pub const TH_CS_ITU_REC_470BG: th_colorspace = 2;
/* *A color space designed for NTSC content.*/
pub const TH_CS_ITU_REC_470M: th_colorspace = 1;
/* *The color space was not specified at the encoder.
It may be conveyed by an external means.*/
pub const TH_CS_UNSPECIFIED: th_colorspace = 0;
/* *The currently defined pixel format tags.
 * See <a href="http://www.theora.org/doc/Theora.pdf">the Theora
 *  specification</a>, Section 4.4, for details on the precise sample
 *  locations.*/
pub type th_pixel_fmt = libc::c_uint;
/* *The total number of currently defined pixel formats.*/
pub const TH_PF_NFORMATS: th_pixel_fmt = 4;
/* *No chroma decimation (4:4:4).
The Cb and Cr chroma planes are full width and full height.*/
pub const TH_PF_444: th_pixel_fmt = 3;
/* *Chroma decimation by 2 in the X direction (4:2:2).
The Cb and Cr chroma planes are half the width of the luma plane, but full
 height.*/
pub const TH_PF_422: th_pixel_fmt = 2;
/* *Currently reserved.*/
pub const TH_PF_RSVD: th_pixel_fmt = 1;
/* *Chroma decimation by 2 in both the X and Y directions (4:2:0).
The Cb and Cr chroma planes are half the width and half the
 height of the luma plane.*/
pub const TH_PF_420: th_pixel_fmt = 0;
/* *A buffer for a single color plane in an uncompressed image.
 * This contains the image data in a left-to-right, top-down format.
 * Each row of pixels is stored contiguously in memory, but successive
 *  rows need not be.
 * Use \a stride to compute the offset of the next row.
 * The encoder accepts both positive \a stride values (top-down in memory)
 *  and negative (bottom-up in memory).
 * The decoder currently always generates images with positive strides.*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct th_img_plane {
  pub width: libc::c_int,
  pub height: libc::c_int,
  pub stride: libc::c_int,
  pub data: *mut libc::c_uchar
}
/* *A complete image buffer for an uncompressed frame.
 * The chroma planes may be decimated by a factor of two in either
 *  direction, as indicated by th_info#pixel_fmt.
 * The width and height of the Y' plane must be multiples of 16.
 * They may need to be cropped for display, using the rectangle
 *  specified by th_info#pic_x, th_info#pic_y, th_info#pic_width,
 *  and th_info#pic_height.
 * All samples are 8 bits.
 * \note The term YUV often used to describe a colorspace is ambiguous.
 * The exact parameters of the RGB to YUV conversion process aside, in
 *  many contexts the U and V channels actually have opposite meanings.
 * To avoid this confusion, we are explicit: the name of the color
 *  channels are Y'CbCr, and they appear in that order, always.
 * The prime symbol denotes that the Y channel is non-linear.
 * Cb and Cr stand for "Chroma blue" and "Chroma red", respectively.*/
pub type th_ycbcr_buffer = [th_img_plane; 3];
/* *Theora bitstream information.
 * This contains the basic playback parameters for a stream, and corresponds to
 *  the initial 'info' header packet.
 * To initialize an encoder, the application fills in this structure and
 *  passes it to th_encode_alloc().
 * A default encoding mode is chosen based on the values of the #quality and
 *  #target_bitrate fields.
 * On decode, it is filled in by th_decode_headerin(), and then passed to
 *  th_decode_alloc().
 *
 * Encoded Theora frames must be a multiple of 16 in size;
 *  this is what the #frame_width and #frame_height members represent.
 * To handle arbitrary picture sizes, a crop rectangle is specified in the
 *  #pic_x, #pic_y, #pic_width and #pic_height members.
 *
 * All frame buffers contain pointers to the full, padded frame.
 * However, the current encoder <em>will not</em> reference pixels outside of
 *  the cropped picture region, and the application does not need to fill them
 *  in.
 * The decoder <em>will</em> allocate storage for a full frame, but the
 *  application <em>should not</em> rely on the padding containing sensible
 *  data.
 *
 * It is also generally recommended that the offsets and sizes should still be
 *  multiples of 2 to avoid chroma sampling shifts when chroma is sub-sampled.
 * See <a href="http://www.theora.org/doc/Theora.pdf">the Theora
 *  specification</a>, Section 4.4, for more details.
 *
 * Frame rate, in frames per second, is stored as a rational fraction, as is
 *  the pixel aspect ratio.
 * Note that this refers to the aspect ratio of the individual pixels, not of
 *  the overall frame itself.
 * The frame aspect ratio can be computed from pixel aspect ratio using the
 *  image dimensions.*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct th_info {
  pub version_major: libc::c_uchar,
  pub version_minor: libc::c_uchar,
  pub version_subminor: libc::c_uchar,
  pub frame_width: ogg_uint32_t,
  pub frame_height: ogg_uint32_t,
  pub pic_width: ogg_uint32_t,
  pub pic_height: ogg_uint32_t,
  pub pic_x: ogg_uint32_t,
  pub pic_y: ogg_uint32_t,
  pub fps_numerator: ogg_uint32_t,
  pub fps_denominator: ogg_uint32_t,
  pub aspect_numerator: ogg_uint32_t,
  pub aspect_denominator: ogg_uint32_t,
  pub colorspace: th_colorspace,
  pub pixel_fmt: th_pixel_fmt,
  pub target_bitrate: libc::c_int,
  pub quality: libc::c_int,
  pub keyframe_granule_shift: libc::c_int
}
/* *A single base matrix.*/
pub type th_quant_base = [libc::c_uchar; 64];
/* *A set of \a qi ranges.*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct th_quant_ranges {
  pub nranges: libc::c_int,
  pub sizes: *const libc::c_int,
  pub base_matrices: *const th_quant_base
}
/* *A complete set of quantization parameters.
The quantizer for each coefficient is calculated as:
\code
 Q=MAX(MIN(qmin[qti][ci!=0],scale[ci!=0][qi]*base[qti][pli][qi][ci]/100),
  1024).
\endcode

\a qti is the quantization type index: 0 for intra, 1 for inter.
<tt>ci!=0</tt> is 0 for the DC coefficient and 1 for AC coefficients.
\a qi is the quality index, ranging between 0 (low quality) and 63 (high
 quality).
\a pli is the color plane index: 0 for Y', 1 for Cb, 2 for Cr.
\a ci is the DCT coefficient index.
Coefficient indices correspond to the normal 2D DCT block
 ordering--row-major with low frequencies first--\em not zig-zag order.

Minimum quantizers are constant, and are given by:
\code
qmin[2][2]={{4,2},{8,4}}.
\endcode

Parameters that can be stored in the bitstream are as follows:
 - The two scale matrices ac_scale and dc_scale.
   \code
   scale[2][64]={dc_scale,ac_scale}.
   \endcode
 - The base matrices for each \a qi, \a qti and \a pli (up to 384 in all).
   In order to avoid storing a full 384 base matrices, only a sparse set of
    matrices are stored, and the rest are linearly interpolated.
   This is done as follows.
   For each \a qti and \a pli, a series of \a n \a qi ranges is defined.
   The size of each \a qi range can vary arbitrarily, but they must sum to
    63.
   Then, <tt>n+1</tt> matrices are specified, one for each endpoint of the
    ranges.
   For interpolation purposes, each range's endpoints are the first \a qi
    value it contains and one past the last \a qi value it contains.
   Fractional values are rounded to the nearest integer, with ties rounded
    away from zero.

   Base matrices are stored by reference, so if the same matrices are used
    multiple times, they will only appear once in the bitstream.
   The bitstream is also capable of omitting an entire set of ranges and
    its associated matrices if they are the same as either the previous
    set (indexed in row-major order) or if the inter set is the same as the
    intra set.

 - Loop filter limit values.
   The same limits are used for the loop filter in all color planes, despite
    potentially differing levels of quantization in each.

For the current encoder, <tt>scale[ci!=0][qi]</tt> must be no greater
 than <tt>scale[ci!=0][qi-1]</tt> and <tt>base[qti][pli][qi][ci]</tt> must
 be no greater than <tt>base[qti][pli][qi-1][ci]</tt>.
These two conditions ensure that the actual quantizer for a given \a qti,
 \a pli, and \a ci does not increase as \a qi increases.
This is not required by the decoder.*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct th_quant_info {
  pub dc_scale: [ogg_uint16_t; 64],
  pub ac_scale: [ogg_uint16_t; 64],
  pub loop_filter_limits: [libc::c_uchar; 64],
  pub qi_ranges: [[th_quant_ranges; 3]; 2]
}
/* *The number of Huffman tables used by Theora.*/
/* *The number of DCT token values in each table.*/
/* *A Huffman code for a Theora DCT token.
 * Each set of Huffman codes in a given table must form a complete, prefix-free
 *  code.
 * There is no requirement that all the tokens in a table have a valid code,
 *  but the current encoder is not optimized to take advantage of this.
 * If each of the five grouops of 16 tables does not contain at least one table
 *  with a code for every token, then the encoder may fail to encode certain
 *  frames.
 * The complete table in the first group of 16 does not have to be in the same
 *  place as the complete table in the other groups, but the complete tables in
 *  the remaining four groups must all be in the same place.*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct th_huff_code {
  pub pattern: ogg_uint32_t,
  pub nbits: libc::c_int
}
/*The internal encoder state.*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct th_enc_ctx {
  pub state: oc_theora_state,
  pub opb: oggpack_buffer,
  pub mb_info: *mut oc_mb_enc_info,
  pub frag_dc: *mut ogg_int16_t,
  pub coded_mbis: *mut libc::c_uint,
  pub ncoded_mbis: size_t,
  pub packet_state: libc::c_int,
  pub keyframe_frequency_force: ogg_uint32_t,
  pub dup_count: ogg_uint32_t,
  pub nqueued_dups: ogg_uint32_t,
  pub prev_dup_count: ogg_uint32_t,
  pub sp_level: libc::c_int,
  pub vp3_compatible: libc::c_uchar,
  pub coded_inter_frame: libc::c_uchar,
  pub prevframe_dropped: libc::c_uchar,
  pub huff_idxs: [[[libc::c_uchar; 2]; 2]; 2],
  pub mv_bits: [size_t; 2],
  pub chooser: oc_mode_scheme_chooser,
  pub pipe: oc_enc_pipeline_state,
  pub mcu_nvsbs: libc::c_int,
  pub mcu_skip_ssd: *mut libc::c_uint,
  pub mcu_rd_scale: *mut ogg_uint16_t,
  pub mcu_rd_iscale: *mut ogg_uint16_t,
  pub dct_tokens: [*mut *mut libc::c_uchar; 3],
  pub extra_bits: [*mut *mut ogg_uint16_t; 3],
  pub ndct_tokens: [[ptrdiff_t; 64]; 3],
  pub eob_run: [[ogg_uint16_t; 64]; 3],
  pub dct_token_offs: [[libc::c_uchar; 64]; 3],
  pub dc_pred_last: [[libc::c_int; 4]; 3],
  pub frag_sad: *mut libc::c_uint,
  pub frag_satd: *mut libc::c_uint,
  pub frag_ssd: *mut libc::c_uint,
  pub lambda: libc::c_int,
  pub activity_avg: libc::c_uint,
  pub luma_avg: libc::c_uint,
  pub huff_codes: [[th_huff_code; 32]; 80],
  pub qinfo: th_quant_info,
  pub dequant_dc: [[[ogg_uint16_t; 2]; 3]; 64],
  pub dequant: [[[*const ogg_uint16_t; 2]; 3]; 3],
  pub enquant: [[[*mut libc::c_void; 2]; 3]; 3],
  pub enquant_tables: [[[*mut libc::c_void; 2]; 3]; 64],
  pub enquant_table_data: *mut libc::c_uchar,
  pub log_qavg: [[ogg_int64_t; 64]; 2],
  pub log_plq: [[[ogg_int16_t; 2]; 3]; 64],
  pub chroma_rd_scale: [[[ogg_uint16_t; 2]; 64]; 2],
  pub mode_rd: [[[[oc_mode_rd; 24]; 2]; 3]; 3],
  pub rc: oc_rc_state,
  pub opt_data: oc_enc_opt_data
}
/*Encoder specific data that varies according to which variants of the above
functions are used.*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oc_enc_opt_data {
  pub enquant_table_size: size_t,
  pub enquant_table_alignment: libc::c_int
}
/*Rate control state information.*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oc_rc_state {
  pub bits_per_frame: ogg_int64_t,
  pub fullness: ogg_int64_t,
  pub target: ogg_int64_t,
  pub max: ogg_int64_t,
  pub log_npixels: ogg_int64_t,
  pub exp: [libc::c_uint; 2],
  pub buf_delay: libc::c_int,
  pub prev_drop_count: ogg_uint32_t,
  pub log_drop_scale: ogg_int64_t,
  pub log_scale: [ogg_int64_t; 2],
  pub log_qtarget: ogg_int64_t,
  pub drop_frames: libc::c_uchar,
  pub cap_overflow: libc::c_uchar,
  pub cap_underflow: libc::c_uchar,
  pub scalefilter: [oc_iir_filter; 2],
  pub inter_count: libc::c_int,
  pub inter_delay: libc::c_int,
  pub inter_delay_target: libc::c_int,
  pub vfrfilter: oc_iir_filter,
  pub twopass: libc::c_int,
  pub twopass_buffer: [libc::c_uchar; 48],
  pub twopass_buffer_bytes: libc::c_int,
  pub twopass_buffer_fill: libc::c_int,
  pub twopass_force_kf: libc::c_uchar,
  pub prev_metrics: oc_frame_metrics,
  pub cur_metrics: oc_frame_metrics,
  pub frame_metrics: *mut oc_frame_metrics,
  pub nframe_metrics: libc::c_int,
  pub cframe_metrics: libc::c_int,
  pub frame_metrics_head: libc::c_int,
  pub frames_total: [ogg_uint32_t; 3],
  pub frames_left: [ogg_uint32_t; 3],
  pub scale_sum: [ogg_int64_t; 2],
  pub scale_window0: libc::c_int,
  pub scale_window_end: libc::c_int,
  pub nframes: [libc::c_int; 3],
  pub rate_bias: ogg_int64_t
}
/*The 2-pass metrics associated with a single frame.*/
#[derive(BitfieldStruct, Clone, Copy)]
#[repr(C)]
pub struct oc_frame_metrics {
  pub log_scale: ogg_int32_t,
  #[bitfield(name = "dup_count", ty = "libc::c_uint", bits = "0..=30")]
  #[bitfield(name = "frame_type", ty = "libc::c_uint", bits = "31..=31")]
  pub dup_count_frame_type: [u8; 4],
  pub activity_avg: libc::c_uint
}
/*A 2nd order low-pass Bessel follower.
We use this for rate control because it has fast reaction time, but is
 critically damped.*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oc_iir_filter {
  pub c: [ogg_int32_t; 2],
  pub g: ogg_int64_t,
  pub x: [ogg_int32_t; 2],
  pub y: [ogg_int32_t; 2]
}
/*Statistics used to estimate R-D cost of a block in a given coding mode.
See modedec.h for more details.*/
#[derive(Copy, Clone, Serialize, Deserialize, Debug, Default)]
#[repr(C)]
pub struct oc_mode_rd {
  pub rate: ogg_int16_t,
  pub rmse: ogg_int16_t
}
/*Temporary encoder state for the analysis pipeline.*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oc_enc_pipeline_state {
  pub dct_data: [ogg_int16_t; 192],
  pub bounding_values: [libc::c_schar; 256],
  pub fr: [oc_fr_state; 3],
  pub qs: [oc_qii_state; 3],
  pub skip_ssd: [*mut libc::c_uint; 3],
  pub coded_fragis: [*mut ptrdiff_t; 3],
  pub uncoded_fragis: [*mut ptrdiff_t; 3],
  pub ncoded_fragis: [ptrdiff_t; 3],
  pub nuncoded_fragis: [ptrdiff_t; 3],
  pub froffset: [ptrdiff_t; 3],
  pub fragy0: [libc::c_int; 3],
  pub fragy_end: [libc::c_int; 3],
  pub sbi0: [libc::c_uint; 3],
  pub sbi_end: [libc::c_uint; 3],
  pub ndct_tokens1: [libc::c_int; 3],
  pub eob_run1: [libc::c_int; 3],
  pub loop_filter: libc::c_int
}
#[derive(BitfieldStruct, Clone, Copy)]
#[repr(C)]
pub struct oc_qii_state {
  pub bits: ptrdiff_t,
  #[bitfield(name = "qi01_count", ty = "libc::c_uint", bits = "0..=13")]
  #[bitfield(name = "qi01", ty = "libc::c_int", bits = "14..=15")]
  #[bitfield(name = "qi12_count", ty = "libc::c_uint", bits = "16..=29")]
  #[bitfield(name = "qi12", ty = "libc::c_int", bits = "30..=31")]
  pub qi01_count_qi01_qi12_count_qi12: [u8; 4],
  //#[bitfield(padding)]
  pub _pad: [u8; 4]
}
/*State to track coded block flags and their bit cost.
We use opportunity cost to measure the bits required to code or skip the next
 block, using the cheaper of the cost to code it fully or partially, so long
 as both are possible.*/
#[derive(BitfieldStruct, Clone, Copy)]
#[repr(C)]
pub struct oc_fr_state {
  pub bits: ptrdiff_t,
  #[bitfield(name = "sb_partial_count", ty = "libc::c_uint", bits = "0..=15")]
  #[bitfield(name = "sb_full_count", ty = "libc::c_uint", bits = "16..=31")]
  #[bitfield(
    name = "b_coded_count_prev",
    ty = "libc::c_uint",
    bits = "32..=37"
  )]
  #[bitfield(name = "b_coded_prev", ty = "libc::c_int", bits = "38..=39")]
  #[bitfield(name = "b_coded_count", ty = "libc::c_uint", bits = "40..=45")]
  #[bitfield(name = "b_coded", ty = "libc::c_int", bits = "46..=47")]
  #[bitfield(name = "b_count", ty = "libc::c_uint", bits = "48..=52")]
  #[bitfield(
    name = "sb_prefer_partial",
    ty = "libc::c_uint",
    bits = "53..=53"
  )]
  #[bitfield(name = "sb_partial", ty = "libc::c_int", bits = "54..=55")]
  #[bitfield(name = "sb_bits", ty = "libc::c_uint", bits = "56..=61")]
  #[bitfield(name = "sb_full", ty = "libc::c_int", bits = "62..=63")]
  pub sb_partial_count_sb_full_count_b_coded_count_prev_b_coded_prev_b_coded_count_b_coded_b_count_sb_prefer_partial_sb_partial_sb_bits_sb_full:
    [u8; 8]
}
/*State machine to estimate the opportunity cost of coding a MB mode.*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oc_mode_scheme_chooser {
  pub mode_ranks: [*const libc::c_uchar; 8],
  pub scheme0_ranks: [libc::c_uchar; 8],
  pub scheme0_list: [libc::c_uchar; 8],
  pub mode_counts: [libc::c_uint; 8],
  pub scheme_list: [libc::c_uchar; 8],
  pub scheme_bits: [ptrdiff_t; 8]
}
/*Encoder-specific macroblock information.*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oc_mb_enc_info {
  pub cneighbors: [libc::c_uint; 4],
  pub pneighbors: [libc::c_uint; 4],
  pub ncneighbors: libc::c_uchar,
  pub npneighbors: libc::c_uchar,
  pub refined: libc::c_uchar,
  pub analysis_mv: [oc_mv2; 3],
  pub unref_mv: [oc_mv; 2],
  pub block_mv: [oc_mv; 4],
  pub ref_mv: [oc_mv; 4],
  pub error: [ogg_uint16_t; 2],
  pub satd: [libc::c_uint; 2],
  pub block_satd: [libc::c_uint; 4]
}
/*A motion vector.*/
pub type oc_mv = ogg_int16_t;
/* *******************************************************************
*                                                                  *
* THIS FILE IS PART OF THE OggTheora SOFTWARE CODEC SOURCE CODE.   *
* USE, DISTRIBUTION AND REPRODUCTION OF THIS LIBRARY SOURCE IS     *
* GOVERNED BY A BSD-STYLE SOURCE LICENSE INCLUDED WITH THIS SOURCE *
* IN 'COPYING'. PLEASE READ THESE TERMS BEFORE DISTRIBUTING.       *
*                                                                  *
* THE Theora SOURCE CODE IS COPYRIGHT (C) 2002-2009                *
* by the Xiph.Org Foundation http://www.xiph.org/                  *
*                                                                  *
********************************************************************

 function:
 last mod: $Id$

********************************************************************/
/*# define OC_COLLECT_METRICS*/
pub type oc_mv2 = [oc_mv; 2];
/*State information common to both the encoder and decoder.*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oc_theora_state {
  pub info: th_info,
  pub opt_data: oc_base_opt_data,
  pub cpu_flags: ogg_uint32_t,
  pub fplanes: [oc_fragment_plane; 3],
  pub frags: *mut oc_fragment,
  pub frag_buf_offs: *mut ptrdiff_t,
  pub frag_mvs: *mut oc_mv,
  pub nfrags: ptrdiff_t,
  pub sb_maps: *mut oc_sb_map,
  pub sb_flags: *mut oc_sb_flags,
  pub nsbs: libc::c_uint,
  pub mb_maps: *mut oc_mb_map,
  pub mb_modes: *mut libc::c_schar,
  pub nhmbs: libc::c_uint,
  pub nvmbs: libc::c_uint,
  pub nmbs: size_t,
  pub coded_fragis: *mut ptrdiff_t,
  pub ncoded_fragis: [ptrdiff_t; 3],
  pub ntotal_coded_fragis: ptrdiff_t,
  pub ref_frame_bufs: [th_ycbcr_buffer; 6],
  pub ref_frame_idx: [libc::c_int; 6],
  pub ref_frame_data: [*mut libc::c_uchar; 6],
  pub ref_frame_handle: *mut libc::c_uchar,
  pub ref_ystride: [libc::c_int; 3],
  pub nborders: libc::c_int,
  pub borders: [oc_border_info; 16],
  pub keyframe_num: ogg_int64_t,
  pub curframe_num: ogg_int64_t,
  pub granpos: ogg_int64_t,
  pub frame_type: libc::c_schar,
  pub granpos_bias: libc::c_uchar,
  pub nqis: libc::c_uchar,
  pub qis: [libc::c_uchar; 3],
  pub dequant_tables: [[[*mut ogg_uint16_t; 2]; 3]; 64],
  pub dequant_table_data: [[[oc_quant_table; 2]; 3]; 64],
  pub loop_filter_limits: [libc::c_uchar; 64]
}
/* *******************************************************************
*                                                                  *
* THIS FILE IS PART OF THE OggTheora SOFTWARE CODEC SOURCE CODE.   *
* USE, DISTRIBUTION AND REPRODUCTION OF THIS LIBRARY SOURCE IS     *
* GOVERNED BY A BSD-STYLE SOURCE LICENSE INCLUDED WITH THIS SOURCE *
* IN 'COPYING'. PLEASE READ THESE TERMS BEFORE DISTRIBUTING.       *
*                                                                  *
* THE Theora SOURCE CODE IS COPYRIGHT (C) 2002-2009                *
* by the Xiph.Org Foundation and contributors http://www.xiph.org/ *
*                                                                  *
********************************************************************

 function:
   last mod: $Id$

********************************************************************/
pub type oc_quant_table = [ogg_uint16_t; 64];
/*Information about a fragment which intersects the border of the displayable
 region.
This marks which pixels belong to the displayable region.*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oc_border_info {
  pub mask: ogg_int64_t,
  pub npixels: libc::c_int
}
/*A map from a macro block to fragment numbers.*/
pub type oc_mb_map = [oc_mb_map_plane; 3];
/*A single plane of the map from a macro block to fragment numbers.*/
pub type oc_mb_map_plane = [ptrdiff_t; 4];
/*Encoded difference from the previous frame offset by the given motion
vector.*/
/*Encoded difference from the previous frame offset by the last coded motion
vector.*/
/*Encoded difference from the previous frame offset by the second to last
coded motion vector.*/
/*Encoded difference from the same macro block in the previous golden
frame.*/
/*Encoded difference from the previous golden frame offset by the given motion
vector.*/
/*Encoded difference from the previous frame offset by the individual motion
vectors given for each block.*/
/*The number of (coded) modes.*/
/*Determines the reference frame used for a given MB mode.*/
/*Constants for the packet state machine common between encoder and decoder.*/
/*Next packet to emit/read: Codec info header.*/
/*Next packet to emit/read: Comment header.*/
/*Next packet to emit/read: Codec setup header.*/
/*No more packets to emit/read.*/
/*Super blocks are 32x32 segments of pixels in a single color plane indexed
 in image order.
Internally, super blocks are broken up into four quadrants, each of which
 contains a 2x2 pattern of blocks, each of which is an 8x8 block of pixels.
Quadrants, and the blocks within them, are indexed in a special order called
 a "Hilbert curve" within the super block.

In order to differentiate between the Hilbert-curve indexing strategy and
 the regular image order indexing strategy, blocks indexed in image order
 are called "fragments".
Fragments are indexed in image order, left to right, then bottom to top,
 from Y' plane to Cb plane to Cr plane.

The co-located fragments in all image planes corresponding to the location
 of a single quadrant of a luma plane super block form a macro block.
Thus there is only a single set of macro blocks for all planes, each of which
 contains between 6 and 12 fragments, depending on the pixel format.
Therefore macro block information is kept in a separate set of arrays from
 super blocks to avoid unused space in the other planes.
The lists are indexed in super block order.
That is, the macro block corresponding to the macro block mbi in (luma plane)
 super block sbi is at index (sbi<<2|mbi).
Thus the number of macro blocks in each dimension is always twice the number
 of super blocks, even when only an odd number fall inside the coded frame.
These "extra" macro blocks are just an artifact of our internal data layout,
 and not part of the coded stream; they are flagged with a negative MB mode.*/
/*Super block information.*/
#[derive(BitfieldStruct, Clone, Copy)]
#[repr(C)]
pub struct oc_sb_flags {
  #[bitfield(name = "coded_fully", ty = "libc::c_uchar", bits = "0..=0")]
  #[bitfield(name = "coded_partially", ty = "libc::c_uchar", bits = "1..=1")]
  #[bitfield(name = "quad_valid", ty = "libc::c_uchar", bits = "2..=5")]
  pub coded_fully_coded_partially_quad_valid: [u8; 1]
}
/*A map from a super block to fragment numbers.*/
pub type oc_sb_map = [oc_sb_map_quad; 4];
/* *******************************************************************
*                                                                  *
* THIS FILE IS PART OF THE OggTheora SOFTWARE CODEC SOURCE CODE.   *
* USE, DISTRIBUTION AND REPRODUCTION OF THIS LIBRARY SOURCE IS     *
* GOVERNED BY A BSD-STYLE SOURCE LICENSE INCLUDED WITH THIS SOURCE *
* IN 'COPYING'. PLEASE READ THESE TERMS BEFORE DISTRIBUTING.       *
*                                                                  *
* THE Theora SOURCE CODE IS COPYRIGHT (C) 2002-2009                *
* by the Xiph.Org Foundation and contributors http://www.xiph.org/ *
*                                                                  *
********************************************************************

 function:
   last mod: $Id: internal.h 17337 2010-07-19 16:08:54Z tterribe $

********************************************************************/
/*A single quadrant of the map from a super block to fragment numbers.*/
pub type oc_sb_map_quad = [ptrdiff_t; 4];
/*Fragment information.*/
#[derive(BitfieldStruct, Clone, Copy)]
#[repr(C)]
pub struct oc_fragment {
  #[bitfield(name = "coded", ty = "libc::c_uint", bits = "0..=0")]
  #[bitfield(name = "invalid", ty = "libc::c_uint", bits = "1..=1")]
  #[bitfield(name = "qii", ty = "libc::c_uint", bits = "2..=5")]
  #[bitfield(name = "refi", ty = "libc::c_uint", bits = "6..=7")]
  #[bitfield(name = "mb_mode", ty = "libc::c_uint", bits = "8..=10")]
  #[bitfield(name = "borderi", ty = "libc::c_int", bits = "11..=15")]
  #[bitfield(name = "dc", ty = "libc::c_int", bits = "16..=31")]
  pub coded_invalid_qii_refi_mb_mode_borderi_dc: [u8; 4]
}
/*A description of each fragment plane.*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oc_fragment_plane {
  pub nhfrags: libc::c_int,
  pub nvfrags: libc::c_int,
  pub froffset: ptrdiff_t,
  pub nfrags: ptrdiff_t,
  pub nhsbs: libc::c_uint,
  pub nvsbs: libc::c_uint,
  pub sboffset: libc::c_uint,
  pub nsbs: libc::c_uint
}
/*The shared (encoder and decoder) tables that vary according to which variants
of the above functions are used.*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oc_base_opt_data {
  pub dct_fzig_zag: *const libc::c_uchar
}
pub type oc_enc_ctx = th_enc_ctx;
/* *Sets the file name to load/store mode metrics from/to.
 * The file name string is stored by reference, and so must be valid for the
 *  lifetime of the encoder.
 * Mode metric collection uses global tables; do not attempt to perform
 *  multiple collections at once.
 * \param[in] _buf <tt>char[]</tt> The file name.
 * \retval TH_EIMPL   Not supported by this implementation.*/
/*Accumulates various weighted sums of the measurements.
w -> weight
s -> SATD
q -> log quantizer
r -> rate (in bits)
d -> RMSE
All of the single letters correspond to direct, weighted sums, e.g.,
 w=sum(w_i), s=sum(s_i*w_i), etc.
The others correspond to central moments (or co-moments) of the given order,
 e.g., sq=sum((s_i-s/w)*(q_i-q/w)*w_i).
Because we need some moments up to fourth order, we use central moments to
 minimize the dynamic range and prevent rounding error from dominating the
 calculations.*/
#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
#[repr(C)]
pub struct oc_mode_metrics {
  pub w: libc::c_double,
  pub s: libc::c_double,
  pub q: libc::c_double,
  pub r: libc::c_double,
  pub d: libc::c_double,
  pub s2: libc::c_double,
  pub sq: libc::c_double,
  pub q2: libc::c_double,
  pub sr: libc::c_double,
  pub qr: libc::c_double,
  pub r2: libc::c_double,
  pub sd: libc::c_double,
  pub qd: libc::c_double,
  pub d2: libc::c_double,
  pub s2q: libc::c_double,
  pub sq2: libc::c_double,
  pub sqr: libc::c_double,
  pub sqd: libc::c_double,
  pub s2q2: libc::c_double
}
/*The following token skipping code used to also be used in the decoder (and
 even at one point other places in the encoder).
However, it was obsoleted by other optimizations, and is now only used here.
It has been moved here to avoid generating the code when it's not needed.*/
/*Determines the number of blocks or coefficients to be skipped for a given
 token value.
_token:      The token value to skip.
_extra_bits: The extra bits attached to this token.
Return: A positive value indicates that number of coefficients are to be
         skipped in the current block.
        Otherwise, the negative of the return value indicates that number of
         blocks are to be ended.*/
pub type oc_token_skip_func =
  Option<unsafe extern fn(_: libc::c_int, _: libc::c_int) -> ptrdiff_t>;
/* *******************************************************************
*                                                                  *
* THIS FILE IS PART OF THE OggTheora SOFTWARE CODEC SOURCE CODE.   *
* USE, DISTRIBUTION AND REPRODUCTION OF THIS LIBRARY SOURCE IS     *
* GOVERNED BY A BSD-STYLE SOURCE LICENSE INCLUDED WITH THIS SOURCE *
* IN 'COPYING'. PLEASE READ THESE TERMS BEFORE DISTRIBUTING.       *
*                                                                  *
* THE Theora SOURCE CODE IS COPYRIGHT (C) 2002-2011                *
* by the Xiph.Org Foundation http://www.xiph.org/                  *
*                                                                  *
********************************************************************

 function: mode selection code
 last mod: $Id$

********************************************************************/

#[no_mangle]
pub static mut OC_HAS_MODE_METRICS: libc::c_int = 0;
#[no_mangle]
pub static mut OC_MODE_RD_WEIGHT_SATD: [[[[libc::c_double; 24]; 2]; 3]; 8] =
  [[[[0.; 24]; 2]; 3]; 8];
#[no_mangle]
pub static mut OC_MODE_RD_WEIGHT_SAD: [[[[libc::c_double; 24]; 2]; 3]; 8] =
  [[[[0.; 24]; 2]; 3]; 8];
#[no_mangle]
pub static mut OC_MODE_METRICS_SATD: [[[[oc_mode_metrics; 24]; 2]; 3]; 7] =
  [[[[oc_mode_metrics {
    w: 0.,
    s: 0.,
    q: 0.,
    r: 0.,
    d: 0.,
    s2: 0.,
    sq: 0.,
    q2: 0.,
    sr: 0.,
    qr: 0.,
    r2: 0.,
    sd: 0.,
    qd: 0.,
    d2: 0.,
    s2q: 0.,
    sq2: 0.,
    sqr: 0.,
    sqd: 0.,
    s2q2: 0.
  }; 24]; 2]; 3]; 7];
#[no_mangle]
pub static mut OC_MODE_METRICS_SAD: [[[[oc_mode_metrics; 24]; 2]; 3]; 7] =
  [[[[oc_mode_metrics {
    w: 0.,
    s: 0.,
    q: 0.,
    r: 0.,
    d: 0.,
    s2: 0.,
    sq: 0.,
    q2: 0.,
    sr: 0.,
    qr: 0.,
    r2: 0.,
    sd: 0.,
    qd: 0.,
    d2: 0.,
    s2q: 0.,
    sq2: 0.,
    sqr: 0.,
    sqd: 0.,
    s2q2: 0.
  }; 24]; 2]; 3]; 7];
#[no_mangle]
pub unsafe extern fn oc_mode_metrics_add(
  mut _metrics: *mut oc_mode_metrics, mut _w: libc::c_double,
  mut _s: libc::c_int, mut _q: libc::c_int, mut _r: libc::c_int,
  mut _d: libc::c_double
) {
  if (*_metrics).w > 0i32 as libc::c_double {
    let mut ds: libc::c_double = 0.;
    let mut dq: libc::c_double = 0.;
    let mut dr: libc::c_double = 0.;
    let mut dd: libc::c_double = 0.;
    let mut ds2: libc::c_double = 0.;
    let mut dq2: libc::c_double = 0.;
    let mut s2: libc::c_double = 0.;
    let mut sq: libc::c_double = 0.;
    let mut q2: libc::c_double = 0.;
    let mut sr: libc::c_double = 0.;
    let mut qr: libc::c_double = 0.;
    let mut sd: libc::c_double = 0.;
    let mut qd: libc::c_double = 0.;
    let mut s2q: libc::c_double = 0.;
    let mut sq2: libc::c_double = 0.;
    let mut w: libc::c_double = 0.;
    let mut wa: libc::c_double = 0.;
    let mut rwa: libc::c_double = 0.;
    let mut rwa2: libc::c_double = 0.;
    let mut rwb: libc::c_double = 0.;
    let mut rwb2: libc::c_double = 0.;
    let mut rw2: libc::c_double = 0.;
    let mut rw3: libc::c_double = 0.;
    let mut rw4: libc::c_double = 0.;
    wa = (*_metrics).w;
    ds = _s as libc::c_double - (*_metrics).s / wa;
    dq = _q as libc::c_double - (*_metrics).q / wa;
    dr = _r as libc::c_double - (*_metrics).r / wa;
    dd = _d - (*_metrics).d / wa;
    ds2 = ds * ds;
    dq2 = dq * dq;
    s2 = (*_metrics).s2;
    sq = (*_metrics).sq;
    q2 = (*_metrics).q2;
    sr = (*_metrics).sr;
    qr = (*_metrics).qr;
    sd = (*_metrics).sd;
    qd = (*_metrics).qd;
    s2q = (*_metrics).s2q;
    sq2 = (*_metrics).sq2;
    w = wa + _w;
    rwa = wa / w;
    rwb = _w / w;
    rwa2 = rwa * rwa;
    rwb2 = rwb * rwb;
    rw2 = wa * rwb;
    rw3 = rw2 * (rwa2 - rwb2);
    rw4 = _w * rwa2 * rwa2 + wa * rwb2 * rwb2;
    (*_metrics).s2q2 += -2i32 as libc::c_double * (ds * sq2 + dq * s2q) * rwb
      + (ds2 * q2 + 4i32 as libc::c_double * ds * dq * sq + dq2 * s2) * rwb2
      + ds2 * dq2 * rw4;
    (*_metrics).s2q +=
      (-2i32 as libc::c_double * ds * sq - dq * s2) * rwb + ds2 * dq * rw3;
    (*_metrics).sq2 +=
      (-ds * q2 - 2i32 as libc::c_double * dq * sq) * rwb + ds * dq2 * rw3;
    (*_metrics).sqr +=
      (-ds * qr - dq * sr - dr * sq) * rwb + ds * dq * dr * rw3;
    (*_metrics).sqd +=
      (-ds * qd - dq * sd - dd * sq) * rwb + ds * dq * dd * rw3;
    (*_metrics).s2 += ds2 * rw2;
    (*_metrics).sq += ds * dq * rw2;
    (*_metrics).q2 += dq2 * rw2;
    (*_metrics).sr += ds * dr * rw2;
    (*_metrics).qr += dq * dr * rw2;
    (*_metrics).r2 += dr * dr * rw2;
    (*_metrics).sd += ds * dd * rw2;
    (*_metrics).qd += dq * dd * rw2;
    (*_metrics).d2 += dd * dd * rw2
  }
  (*_metrics).w += _w;
  (*_metrics).s += _s as libc::c_double * _w;
  (*_metrics).q += _q as libc::c_double * _w;
  (*_metrics).r += _r as libc::c_double * _w;
  (*_metrics).d += _d * _w;
}
#[no_mangle]
pub unsafe extern fn oc_mode_metrics_merge(
  mut _dst: *mut oc_mode_metrics, mut _src: *const oc_mode_metrics,
  mut _n: libc::c_int
) {
  let mut i: libc::c_int = 0;
  i = 0i32;
  while i < _n && (*_src.offset(i as isize)).w == 0i32 as libc::c_double {
    i += 1
  }
  if i >= _n {
    memset(
      _dst as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<oc_mode_metrics>() as libc::c_ulong
    );
    return;
  }
  memcpy(
    _dst as *mut libc::c_void,
    _src.offset(i as isize) as *const libc::c_void,
    ::std::mem::size_of::<oc_mode_metrics>() as libc::c_ulong
  );
  i += 1;
  while i < _n {
    if (*_src.offset(i as isize)).w != 0i32 as libc::c_double {
      let mut ds: libc::c_double = 0.;
      let mut dq: libc::c_double = 0.;
      let mut dr: libc::c_double = 0.;
      let mut dd: libc::c_double = 0.;
      let mut ds2: libc::c_double = 0.;
      let mut dq2: libc::c_double = 0.;
      let mut s2a: libc::c_double = 0.;
      let mut s2b: libc::c_double = 0.;
      let mut sqa: libc::c_double = 0.;
      let mut sqb: libc::c_double = 0.;
      let mut q2a: libc::c_double = 0.;
      let mut q2b: libc::c_double = 0.;
      let mut sra: libc::c_double = 0.;
      let mut srb: libc::c_double = 0.;
      let mut qra: libc::c_double = 0.;
      let mut qrb: libc::c_double = 0.;
      let mut sda: libc::c_double = 0.;
      let mut sdb: libc::c_double = 0.;
      let mut qda: libc::c_double = 0.;
      let mut qdb: libc::c_double = 0.;
      let mut s2qa: libc::c_double = 0.;
      let mut s2qb: libc::c_double = 0.;
      let mut sq2a: libc::c_double = 0.;
      let mut sq2b: libc::c_double = 0.;
      let mut w: libc::c_double = 0.;
      let mut wa: libc::c_double = 0.;
      let mut wb: libc::c_double = 0.;
      let mut rwa: libc::c_double = 0.;
      let mut rwb: libc::c_double = 0.;
      let mut rwa2: libc::c_double = 0.;
      let mut rwb2: libc::c_double = 0.;
      let mut rw2: libc::c_double = 0.;
      let mut rw3: libc::c_double = 0.;
      let mut rw4: libc::c_double = 0.;
      wa = (*_dst).w;
      wb = (*_src.offset(i as isize)).w;
      ds = (*_src.offset(i as isize)).s / wb - (*_dst).s / wa;
      dq = (*_src.offset(i as isize)).q / wb - (*_dst).q / wa;
      dr = (*_src.offset(i as isize)).r / wb - (*_dst).r / wa;
      dd = (*_src.offset(i as isize)).d / wb - (*_dst).d / wa;
      ds2 = ds * ds;
      dq2 = dq * dq;
      s2a = (*_dst).s2;
      sqa = (*_dst).sq;
      q2a = (*_dst).q2;
      sra = (*_dst).sr;
      qra = (*_dst).qr;
      sda = (*_dst).sd;
      qda = (*_dst).qd;
      s2qa = (*_dst).s2q;
      sq2a = (*_dst).sq2;
      s2b = (*_src.offset(i as isize)).s2;
      sqb = (*_src.offset(i as isize)).sq;
      q2b = (*_src.offset(i as isize)).q2;
      srb = (*_src.offset(i as isize)).sr;
      qrb = (*_src.offset(i as isize)).qr;
      sdb = (*_src.offset(i as isize)).sd;
      qdb = (*_src.offset(i as isize)).qd;
      s2qb = (*_src.offset(i as isize)).s2q;
      sq2b = (*_src.offset(i as isize)).sq2;
      w = wa + wb;
      if w == 0i32 as libc::c_double {
        rwb = 0i32 as libc::c_double;
        rwa = rwb
      } else {
        rwa = wa / w;
        rwb = wb / w
      }
      rwa2 = rwa * rwa;
      rwb2 = rwb * rwb;
      rw2 = wa * rwb;
      rw3 = rw2 * (rwa2 - rwb2);
      rw4 = wb * rwa2 * rwa2 + wa * rwb2 * rwb2;
      (*_dst).s2q2 += (*_src.offset(i as isize)).s2q2
        + 2i32 as libc::c_double
          * (ds * (rwa * sq2b - rwb * sq2a) + dq * (rwa * s2qb - rwb * s2qa))
        + ds2 * (rwa2 * q2b + rwb2 * q2a)
        + 4i32 as libc::c_double * ds * dq * (rwa2 * sqb + rwb2 * sqa)
        + dq2 * (rwa2 * s2b + rwb2 * s2a)
        + ds2 * dq2 * rw4;
      (*_dst).s2q += (*_src.offset(i as isize)).s2q
        + 2i32 as libc::c_double * ds * (rwa * sqb - rwb * sqa)
        + dq * (rwa * s2b - rwb * s2a)
        + ds2 * dq * rw3;
      (*_dst).sq2 += (*_src.offset(i as isize)).sq2
        + ds * (rwa * q2b - rwb * q2a)
        + 2i32 as libc::c_double * dq * (rwa * sqb - rwb * sqa)
        + ds * dq2 * rw3;
      (*_dst).sqr += (*_src.offset(i as isize)).sqr
        + ds * (rwa * qrb - rwb * qra)
        + dq * (rwa * srb - rwb * sra)
        + dr * (rwa * sqb - rwb * sqa)
        + ds * dq * dr * rw3;
      (*_dst).sqd += (*_src.offset(i as isize)).sqd
        + ds * (rwa * qdb - rwb * qda)
        + dq * (rwa * sdb - rwb * sda)
        + dd * (rwa * sqb - rwb * sqa)
        + ds * dq * dd * rw3;
      (*_dst).s2 += (*_src.offset(i as isize)).s2 + ds2 * rw2;
      (*_dst).sq += (*_src.offset(i as isize)).sq + ds * dq * rw2;
      (*_dst).q2 += (*_src.offset(i as isize)).q2 + dq2 * rw2;
      (*_dst).sr += (*_src.offset(i as isize)).sr + ds * dr * rw2;
      (*_dst).qr += (*_src.offset(i as isize)).qr + dq * dr * rw2;
      (*_dst).r2 += (*_src.offset(i as isize)).r2 + dr * dr * rw2;
      (*_dst).sd += (*_src.offset(i as isize)).sd + ds * dd * rw2;
      (*_dst).qd += (*_src.offset(i as isize)).qd + dq * dd * rw2;
      (*_dst).d2 += (*_src.offset(i as isize)).d2 + dd * dd * rw2;
      (*_dst).w += (*_src.offset(i as isize)).w;
      (*_dst).s += (*_src.offset(i as isize)).s;
      (*_dst).q += (*_src.offset(i as isize)).q;
      (*_dst).r += (*_src.offset(i as isize)).r;
      (*_dst).d += (*_src.offset(i as isize)).d
    }
    i += 1
  }
}
/*Adjust a single corner of a set of metric bins to minimize the squared
 prediction error of R and D.
Each bin is assumed to cover a quad like so:
  (s0,q0)    (s1,q0)
     A----------B
     |          |
     |          |
     |          |
     |          |
     C----------Z
  (s0,q1)    (s1,q1)
The values A, B, and C are fixed, and Z is the free parameter.
Then, for example, R_i is predicted via bilinear interpolation as
  x_i=(s_i-s0)/(s1-s0)
  y_i=(q_i-q0)/(q1-q0)
  dRds1_i=A+(B-A)*x_i
  dRds2_i=C+(Z-C)*x_i
  R_i=dRds1_i+(dRds2_i-dRds1_i)*y_i
To find the Z that minimizes the squared prediction error over i, this can
 be rewritten as
  R_i-(A+(B-A)*x_i+(C-A)*y_i+(A-B-C)*x_i*y_i)=x_i*y_i*Z
Letting X={...,x_i*y_i,...}^T and
 Y={...,R_i-(A+(B-A)*x_i+(C-A)*y_i+(A-B-C)*x_i*y_i),...}^T,
 the optimal Z is given by Z=(X^T.Y)/(X^T.X).
Now, we need to compute these dot products without actually storing data for
 each sample.
Starting with X^T.X, we have
 X^T.X = sum(x_i^2*y_i^2) = sum((s_i-s0)^2*(q_i-q0)^2)/((s1-s0)^2*(q1-q0)^2).
Expanding the interior of the sum in a monomial basis of s_i and q_i gives
  s0^2*q0^2  *(1)
   -2*s0*q0^2*(s_i)
   -2*s0^2*q0*(q_i)
   +q0^2     *(s_i^2)
   +4*s0*q0  *(s_i*q_i)
   +s0^2     *(q_i^2)
   -2*q0     *(s_i^2*q_i)
   -2*s0     *(s_i*q_i^2)
   +1        *(s_i^2*q_i^2).
However, computing things directly in this basis leads to gross numerical
 errors, as most of the terms will have similar size and destructive
 cancellation results.
A much better basis is the central (co-)moment basis:
  {1,s_i-sbar,q_i-qbar,(s_i-sbar)^2,(s_i-sbar)*(q_i-qbar),(q_i-qbar)^2,
   (s_i-sbar)^2*(q_i-qbar),(s_i-sbar)*(q_i-qbar)^2,(s_i-sbar)^2*(q_i-qbar)^2},
 where sbar and qbar are the average s and q values over the bin,
 respectively.
In that basis, letting ds=sbar-s0 and dq=qbar-q0, (s_i-s0)^2*(q_i-q0)^2 is
  ds^2*dq^2*(1)
   +dq^2   *((s_i-sbar)^2)
   +4*ds*dq*((s_i-sbar)*(q_i-qbar))
   +ds^2   *((q_i-qbar)^2)
   +2*dq   *((s_i-sbar)^2*(q_i-qbar))
   +2*ds   *((s_i-sbar)*(q_i-qbar)^2)
   +1      *((s_i-sbar)^2*(q_i-qbar)^2).
With these expressions in the central (co-)moment bases, all we need to do
 is compute sums over the (co-)moment terms, which can be done
 incrementally (see oc_mode_metrics_add() and oc_mode_metrics_merge()),
 with no need to store the individual samples.
Now, for X^T.Y, we have
  X^T.Y = sum((R_i-A-((B-A)/(s1-s0))*(s_i-s0)-((C-A)/(q1-q0))*(q_i-q0)
   -((A-B-C)/((s1-s0)*(q1-q0)))*(s_i-s0)*(q_i-q0))*(s_i-s0)*(q_i-q0))/
   ((s1-s0)*(q1-q0)),
 or, rewriting the constants to simplify notation,
  X^T.Y = sum((C0+C1*(s_i-s0)+C2*(q_i-q0)
   +C3*(s_i-s0)*(q_i-q0)+R_i)*(s_i-s0)*(q_i-q0))/((s1-s0)*(q1-q0)).
Again, converting to the central (co-)moment basis, the interior of the
 above sum is
  ds*dq*(rbar+C0+C1*ds+C2*dq+C3*ds*dq)  *(1)
   +(C1*dq+C3*dq^2)                     *((s_i-sbar)^2)
   +(rbar+C0+2*C1*ds+2*C2*dq+4*C3*ds*dq)*((s_i-sbar)*(q_i-qbar))
   +(C2*ds+C3*ds^2)                     *((q_i-qbar)^2)
   +dq                                  *((s_i-sbar)*(r_i-rbar))
   +ds                                  *((q_i-qbar)*(r_i-rbar))
   +(C1+2*C3*dq)                        *((s_i-sbar)^2*(q_i-qbar))
   +(C2+2*C3*ds)                        *((s_i-sbar)*(q_i-qbar)^2)
   +1                                   *((s_i-sbar)*(q_i-qbar)*(r_i-rbar))
   +C3                                  *((s_i-sbar)^2*(q_i-qbar)^2).
You might think it would be easier (if perhaps slightly less robust) to
 accumulate terms directly around s0 and q0.
However, we update each corner of the bins in turn, so we would have to
 change basis to move the sums from corner to corner anyway.*/
#[no_mangle]
pub unsafe extern fn oc_mode_metrics_solve(
  mut _r: *mut libc::c_double, mut _d: *mut libc::c_double,
  mut _metrics: *const oc_mode_metrics, mut _s0: *const libc::c_int,
  mut _s1: *const libc::c_int, mut _q0: *const libc::c_int,
  mut _q1: *const libc::c_int, mut _ra: *const libc::c_double,
  mut _rb: *const libc::c_double, mut _rc: *const libc::c_double,
  mut _da: *const libc::c_double, mut _db: *const libc::c_double,
  mut _dc: *const libc::c_double, mut _n: libc::c_int
) -> libc::c_double {
  let mut xx: libc::c_double = 0.;
  let mut rxy: libc::c_double = 0.;
  let mut dxy: libc::c_double = 0.;
  let mut wt: libc::c_double = 0.;
  let mut i: libc::c_int = 0;
  wt = 0i32 as libc::c_double;
  dxy = wt;
  rxy = dxy;
  xx = rxy;
  i = 0i32;
  while i < _n {
    if (*_metrics.offset(i as isize)).w > 0i32 as libc::c_double {
      let mut s10: libc::c_double = 0.;
      let mut q10: libc::c_double = 0.;
      let mut sq10: libc::c_double = 0.;
      let mut ds: libc::c_double = 0.;
      let mut dq: libc::c_double = 0.;
      let mut ds2: libc::c_double = 0.;
      let mut dq2: libc::c_double = 0.;
      let mut r: libc::c_double = 0.;
      let mut d: libc::c_double = 0.;
      let mut s2: libc::c_double = 0.;
      let mut sq: libc::c_double = 0.;
      let mut q2: libc::c_double = 0.;
      let mut sr: libc::c_double = 0.;
      let mut qr: libc::c_double = 0.;
      let mut sd: libc::c_double = 0.;
      let mut qd: libc::c_double = 0.;
      let mut s2q: libc::c_double = 0.;
      let mut sq2: libc::c_double = 0.;
      let mut sqr: libc::c_double = 0.;
      let mut sqd: libc::c_double = 0.;
      let mut s2q2: libc::c_double = 0.;
      let mut c0: libc::c_double = 0.;
      let mut c1: libc::c_double = 0.;
      let mut c2: libc::c_double = 0.;
      let mut c3: libc::c_double = 0.;
      let mut w: libc::c_double = 0.;
      w = (*_metrics.offset(i as isize)).w;
      wt += w;
      s10 =
        (*_s1.offset(i as isize) - *_s0.offset(i as isize)) as libc::c_double;
      q10 =
        (*_q1.offset(i as isize) - *_q0.offset(i as isize)) as libc::c_double;
      sq10 = s10 * q10;
      ds = (*_metrics.offset(i as isize)).s / w
        - *_s0.offset(i as isize) as libc::c_double;
      dq = (*_metrics.offset(i as isize)).q / w
        - *_q0.offset(i as isize) as libc::c_double;
      ds2 = ds * ds;
      dq2 = dq * dq;
      s2 = (*_metrics.offset(i as isize)).s2;
      sq = (*_metrics.offset(i as isize)).sq;
      q2 = (*_metrics.offset(i as isize)).q2;
      s2q = (*_metrics.offset(i as isize)).s2q;
      sq2 = (*_metrics.offset(i as isize)).sq2;
      s2q2 = (*_metrics.offset(i as isize)).s2q2;
      xx += (dq2 * (ds2 * w + s2)
        + 4i32 as libc::c_double * ds * dq * sq
        + ds2 * q2
        + 2i32 as libc::c_double * (dq * s2q + ds * sq2)
        + s2q2)
        / (sq10 * sq10);
      r = (*_metrics.offset(i as isize)).r / w;
      sr = (*_metrics.offset(i as isize)).sr;
      qr = (*_metrics.offset(i as isize)).qr;
      sqr = (*_metrics.offset(i as isize)).sqr;
      c0 = -*_ra.offset(i as isize);
      c1 = -(*_rb.offset(i as isize) - *_ra.offset(i as isize)) / s10;
      c2 = -(*_rc.offset(i as isize) - *_ra.offset(i as isize)) / q10;
      c3 = -(*_ra.offset(i as isize)
        - *_rb.offset(i as isize)
        - *_rc.offset(i as isize))
        / sq10;
      rxy += (ds * dq * (r + c0 + c1 * ds + c2 * dq + c3 * ds * dq) * w
        + (c1 * dq + c3 * dq2) * s2
        + (r
          + c0
          + 2i32 as libc::c_double
            * (c1 * ds + (c2 + 2i32 as libc::c_double * c3 * ds) * dq))
          * sq
        + (c2 * ds + c3 * ds2) * q2
        + dq * sr
        + ds * qr
        + (c1 + 2i32 as libc::c_double * c3 * dq) * s2q
        + (c2 + 2i32 as libc::c_double * c3 * ds) * sq2
        + sqr
        + c3 * s2q2)
        / sq10;
      d = (*_metrics.offset(i as isize)).d / w;
      sd = (*_metrics.offset(i as isize)).sd;
      qd = (*_metrics.offset(i as isize)).qd;
      sqd = (*_metrics.offset(i as isize)).sqd;
      c0 = -*_da.offset(i as isize);
      c1 = -(*_db.offset(i as isize) - *_da.offset(i as isize)) / s10;
      c2 = -(*_dc.offset(i as isize) - *_da.offset(i as isize)) / q10;
      c3 = -(*_da.offset(i as isize)
        - *_db.offset(i as isize)
        - *_dc.offset(i as isize))
        / sq10;
      dxy += (ds * dq * (d + c0 + c1 * ds + c2 * dq + c3 * ds * dq) * w
        + (c1 * dq + c3 * dq2) * s2
        + (d
          + c0
          + 2i32 as libc::c_double
            * (c1 * ds + (c2 + 2i32 as libc::c_double * c3 * ds) * dq))
          * sq
        + (c2 * ds + c3 * ds2) * q2
        + dq * sd
        + ds * qd
        + (c1 + 2i32 as libc::c_double * c3 * dq) * s2q
        + (c2 + 2i32 as libc::c_double * c3 * ds) * sq2
        + sqd
        + c3 * s2q2)
        / sq10;

/*
      if rxy < 0.0 || xx < 0.0 {
        dbg!(
          i,
          (*_metrics.offset(i as isize)).s / w,
          *_s0.offset(i as isize)
        );
        dbg!(
          s10,
          q10,
          sq10,
          ds,
          dq,
          ds2,
          dq2,
          r,
          d,
          s2,
          sq,
          q2,
          sr,
          qr,
          sd,
          qd,
          s2q,
          sq2,
          sqr,
          sqd,
          s2q2,
          c0,
          c1,
          c2,
          c3,
          w,
        );
        dbg!(xx, rxy, dxy, wt, i);
        dbg!(ds * dq * (r + c0 + c1 * ds + c2 * dq + c3 * ds * dq) * w
        , (c1 * dq + c3 * dq2) * s2
        , (r
          + c0
          + 2i32 as libc::c_double
            * (c1 * ds + (c2 + 2i32 as libc::c_double * c3 * ds) * dq))
          * sq
        , (c2 * ds + c3 * ds2) * q2
        , dq * sr
        , ds * qr
        , (c1 + 2i32 as libc::c_double * c3 * dq) * s2q
        , (c2 + 2i32 as libc::c_double * c3 * ds) * sq2
        , sqr
        , c3 * s2q2);
      }*/
    }
    i += 1
  }
  if xx > 1E-3f64 {
    *_r = rxy / xx;
    *_d = dxy / xx
  } else {
    *_r = 0i32 as libc::c_double;
    *_d = 0i32 as libc::c_double
  }
  return wt;
}
/*Compile collected SATD/logq/rate/RMSE metrics into a form that's immediately
useful for mode decision.*/
#[no_mangle]
pub unsafe extern fn oc_mode_metrics_update(
  mut _metrics: *mut [[[oc_mode_metrics; 24]; 2]; 3],
  mut _niters_min: libc::c_int, mut _reweight: libc::c_int,
  mut _table: *mut [[[oc_mode_rd; 24]; 2]; 3], mut _shift: libc::c_int,
  mut _weight: *mut [[[libc::c_double; 24]; 2]; 3]
) {
  let mut niters: libc::c_int = 0;
  let mut prevdr: libc::c_int = 0;
  let mut prevdd: libc::c_int = 0;
  let mut dr: libc::c_int = 0;
  let mut dd: libc::c_int = 0;
  let mut pli: libc::c_int = 0;
  let mut qti: libc::c_int = 0;
  let mut qi: libc::c_int = 0;
  let mut si: libc::c_int = 0;
  dr = 2147483647i32;
  dd = dr;
  niters = 0i32;
  loop {
    prevdr = dr;
    prevdd = dd;
    dr = 0i32;
    dd = dr;
    pli = 0i32;
    while pli < 3i32 {
      qti = 0i32;
      while qti < 2i32 {
        qi = 0i32;
        while qi < 8i32 {
          si = 0i32;
          while si < 24i32 {
            let mut m: [oc_mode_metrics; 4] = [oc_mode_metrics {
              w: 0.,
              s: 0.,
              q: 0.,
              r: 0.,
              d: 0.,
              s2: 0.,
              sq: 0.,
              q2: 0.,
              sr: 0.,
              qr: 0.,
              r2: 0.,
              sd: 0.,
              qd: 0.,
              d2: 0.,
              s2q: 0.,
              sq2: 0.,
              sqr: 0.,
              sqd: 0.,
              s2q2: 0.
            }; 4];
            let mut s0: [libc::c_int; 4] = [0; 4];
            let mut s1: [libc::c_int; 4] = [0; 4];
            let mut q0: [libc::c_int; 4] = [0; 4];
            let mut q1: [libc::c_int; 4] = [0; 4];
            let mut ra: [libc::c_double; 4] = [0.; 4];
            let mut rb: [libc::c_double; 4] = [0.; 4];
            let mut rc: [libc::c_double; 4] = [0.; 4];
            let mut da: [libc::c_double; 4] = [0.; 4];
            let mut db: [libc::c_double; 4] = [0.; 4];
            let mut dc: [libc::c_double; 4] = [0.; 4];
            let mut r: libc::c_double = 0.;
            let mut d: libc::c_double = 0.;
            let mut rate: libc::c_int = 0;
            let mut rmse: libc::c_int = 0;
            let mut ds: libc::c_int = 0;
            let mut n: libc::c_int = 0;
            n = 0i32;
            if qi > 0i32 && si > 0i32 {
              q0[n as usize] = OC_MODE_LOGQ[(qi - 1i32) as usize][pli as usize][qti as usize] as libc::c_int;
              q1[n as usize] = OC_MODE_LOGQ[qi as usize][pli as usize][qti as usize] as libc::c_int;
              s0[n as usize] = si - 1i32 << _shift;
              s1[n as usize] = si << _shift;
              ra[n as usize] = ldexp(
                (*_table.offset((qi - 1i32) as isize))[pli as usize]
                  [qti as usize][(si - 1i32) as usize]
                  .rate as libc::c_double,
                -6i32
              );
              da[n as usize] = ldexp(
                (*_table.offset((qi - 1i32) as isize))[pli as usize]
                  [qti as usize][(si - 1i32) as usize]
                  .rmse as libc::c_double,
                -5i32
              );
              rb[n as usize] = ldexp(
                (*_table.offset((qi - 1i32) as isize))[pli as usize]
                  [qti as usize][si as usize]
                  .rate as libc::c_double,
                -6i32
              );
              db[n as usize] = ldexp(
                (*_table.offset((qi - 1i32) as isize))[pli as usize]
                  [qti as usize][si as usize]
                  .rmse as libc::c_double,
                -5i32
              );
              rc[n as usize] = ldexp(
                (*_table.offset(qi as isize))[pli as usize][qti as usize]
                  [(si - 1i32) as usize]
                  .rate as libc::c_double,
                -6i32
              );
              dc[n as usize] = ldexp(
                (*_table.offset(qi as isize))[pli as usize][qti as usize]
                  [(si - 1i32) as usize]
                  .rmse as libc::c_double,
                -5i32
              );
              let fresh0 = n;
              n = n + 1;
              *m.as_mut_ptr().offset(fresh0 as isize) = *(*_metrics
                .offset((qi - 1i32) as isize))[pli as usize][qti as usize]
                .as_mut_ptr()
                .offset(si as isize)
                .offset(-1isize)
            }
            if qi > 0i32 {
              ds = if si + 1i32 < 24i32 { 1i32 } else { -1i32 };
              q0[n as usize] = OC_MODE_LOGQ[(qi - 1i32) as usize][pli as usize][qti as usize] as libc::c_int;
              q1[n as usize] = OC_MODE_LOGQ[qi as usize][pli as usize][qti as usize] as libc::c_int;
              s0[n as usize] = si + ds << _shift;
              s1[n as usize] = si << _shift;
              ra[n as usize] = ldexp(
                (*_table.offset((qi - 1i32) as isize))[pli as usize]
                  [qti as usize][(si + ds) as usize]
                  .rate as libc::c_double,
                -6i32
              );
              da[n as usize] = ldexp(
                (*_table.offset((qi - 1i32) as isize))[pli as usize]
                  [qti as usize][(si + ds) as usize]
                  .rmse as libc::c_double,
                -5i32
              );
              rb[n as usize] = ldexp(
                (*_table.offset((qi - 1i32) as isize))[pli as usize]
                  [qti as usize][si as usize]
                  .rate as libc::c_double,
                -6i32
              );
              db[n as usize] = ldexp(
                (*_table.offset((qi - 1i32) as isize))[pli as usize]
                  [qti as usize][si as usize]
                  .rmse as libc::c_double,
                -5i32
              );
              rc[n as usize] = ldexp(
                (*_table.offset(qi as isize))[pli as usize][qti as usize]
                  [(si + ds) as usize]
                  .rate as libc::c_double,
                -6i32
              );
              dc[n as usize] = ldexp(
                (*_table.offset(qi as isize))[pli as usize][qti as usize]
                  [(si + ds) as usize]
                  .rmse as libc::c_double,
                -5i32
              );
              let fresh1 = n;
              n = n + 1;
              *m.as_mut_ptr().offset(fresh1 as isize) = *(*_metrics
                .offset((qi - 1i32) as isize))[pli as usize][qti as usize]
                .as_mut_ptr()
                .offset(si as isize)
            }
            if qi + 1i32 < 8i32 && si > 0i32 {
              q0[n as usize] = OC_MODE_LOGQ[(qi + 1i32) as usize][pli as usize][qti as usize] as libc::c_int;
              q1[n as usize] = OC_MODE_LOGQ[qi as usize][pli as usize][qti as usize] as libc::c_int;
              s0[n as usize] = si - 1i32 << _shift;
              s1[n as usize] = si << _shift;
              ra[n as usize] = ldexp(
                (*_table.offset((qi + 1i32) as isize))[pli as usize]
                  [qti as usize][(si - 1i32) as usize]
                  .rate as libc::c_double,
                -6i32
              );
              da[n as usize] = ldexp(
                (*_table.offset((qi + 1i32) as isize))[pli as usize]
                  [qti as usize][(si - 1i32) as usize]
                  .rmse as libc::c_double,
                -5i32
              );
              rb[n as usize] = ldexp(
                (*_table.offset((qi + 1i32) as isize))[pli as usize]
                  [qti as usize][si as usize]
                  .rate as libc::c_double,
                -6i32
              );
              db[n as usize] = ldexp(
                (*_table.offset((qi + 1i32) as isize))[pli as usize]
                  [qti as usize][si as usize]
                  .rmse as libc::c_double,
                -5i32
              );
              rc[n as usize] = ldexp(
                (*_table.offset(qi as isize))[pli as usize][qti as usize]
                  [(si - 1i32) as usize]
                  .rate as libc::c_double,
                -6i32
              );
              dc[n as usize] = ldexp(
                (*_table.offset(qi as isize))[pli as usize][qti as usize]
                  [(si - 1i32) as usize]
                  .rmse as libc::c_double,
                -5i32
              );
              let fresh2 = n;
              n = n + 1;
              *m.as_mut_ptr().offset(fresh2 as isize) = *(*_metrics
                .offset(qi as isize))[pli as usize][qti as usize]
                .as_mut_ptr()
                .offset(si as isize)
                .offset(-1isize)
            }
            if qi + 1i32 < 8i32 {
              ds = if si + 1i32 < 24i32 { 1i32 } else { -1i32 };
              q0[n as usize] = OC_MODE_LOGQ[(qi + 1i32) as usize][pli as usize][qti as usize] as libc::c_int;
              q1[n as usize] = OC_MODE_LOGQ[qi as usize][pli as usize][qti as usize] as libc::c_int;
              s0[n as usize] = si + ds << _shift;
              s1[n as usize] = si << _shift;
              ra[n as usize] = ldexp(
                (*_table.offset((qi + 1i32) as isize))[pli as usize]
                  [qti as usize][(si + ds) as usize]
                  .rate as libc::c_double,
                -6i32
              );
              da[n as usize] = ldexp(
                (*_table.offset((qi + 1i32) as isize))[pli as usize]
                  [qti as usize][(si + ds) as usize]
                  .rmse as libc::c_double,
                -5i32
              );
              rb[n as usize] = ldexp(
                (*_table.offset((qi + 1i32) as isize))[pli as usize]
                  [qti as usize][si as usize]
                  .rate as libc::c_double,
                -6i32
              );
              db[n as usize] = ldexp(
                (*_table.offset((qi + 1i32) as isize))[pli as usize]
                  [qti as usize][si as usize]
                  .rmse as libc::c_double,
                -5i32
              );
              rc[n as usize] = ldexp(
                (*_table.offset(qi as isize))[pli as usize][qti as usize]
                  [(si + ds) as usize]
                  .rate as libc::c_double,
                -6i32
              );
              dc[n as usize] = ldexp(
                (*_table.offset(qi as isize))[pli as usize][qti as usize]
                  [(si + ds) as usize]
                  .rmse as libc::c_double,
                -5i32
              );
              let fresh3 = n;
              n = n + 1;
              *m.as_mut_ptr().offset(fresh3 as isize) = *(*_metrics
                .offset(qi as isize))[pli as usize][qti as usize]
                .as_mut_ptr()
                .offset(si as isize)
            }
            if 0 == OC_HAS_MODE_METRICS && niters == 0i32 {
              let mut w: libc::c_double = 0.;
              d = 0i32 as libc::c_double;
              r = d;
              w = r;
              loop {
                let fresh4 = n;
                n = n - 1;
                if !(fresh4 > 0i32) {
                  break;
                }
                w += m[n as usize].w;
                r += m[n as usize].r;
                d += m[n as usize].d
              }
              r = if w > 1E-3f64 { r / w } else { 0i32 as libc::c_double };
              d = if w > 1E-3f64 { d / w } else { 0i32 as libc::c_double };
              (*_weight.offset(qi as isize))[pli as usize][qti as usize]
                [si as usize] = w
            } else {
              (*_weight.offset(qi as isize))[pli as usize][qti as usize]
                [si as usize] = oc_mode_metrics_solve(
                &mut r,
                &mut d,
                m.as_mut_ptr(),
                s0.as_mut_ptr(),
                s1.as_mut_ptr(),
                q0.as_mut_ptr(),
                q1.as_mut_ptr(),
                ra.as_mut_ptr(),
                rb.as_mut_ptr(),
                rc.as_mut_ptr(),
                da.as_mut_ptr(),
                db.as_mut_ptr(),
                dc.as_mut_ptr(),
                n
              )
            }
            /*if r < 0.0 {
              dbg!(m, s0, s1, q0, q1, ra, rb, rc, da, db, dc, n);
              dbg!(pli, qti, qi, si);
              dbg!(r, d);
            }*/
            // assert!(r >= 0.0);
            if r < 0.0 {
              dbg!("Negative r: {}", r);
              r = 0.0; // ¯\_(ツ)_/¯
            }
            if d < 0.0 {
              dbg!("Negative d: {}", d);
              d = 0.0; // ¯\_(ツ)_/¯
            }
            rate = -32768i32
              - (-32768i32
                - ((ldexp(r, 6i32) + 0.5f64) as libc::c_int
                  + (32767i32 - (ldexp(r, 6i32) + 0.5f64) as libc::c_int
                    & -((32767i32 < (ldexp(r, 6i32) + 0.5f64) as libc::c_int)
                      as libc::c_int)))
                & -(((ldexp(r, 6i32) + 0.5f64) as libc::c_int
                  + (32767i32 - (ldexp(r, 6i32) + 0.5f64) as libc::c_int
                    & -((32767i32 < (ldexp(r, 6i32) + 0.5f64) as libc::c_int)
                      as libc::c_int))
                  > -32768i32) as libc::c_int));
            rmse = -32768i32
              - (-32768i32
                - ((ldexp(d, 5i32) + 0.5f64) as libc::c_int
                  + (32767i32 - (ldexp(d, 5i32) + 0.5f64) as libc::c_int
                    & -((32767i32 < (ldexp(d, 5i32) + 0.5f64) as libc::c_int)
                      as libc::c_int)))
                & -(((ldexp(d, 5i32) + 0.5f64) as libc::c_int
                  + (32767i32 - (ldexp(d, 5i32) + 0.5f64) as libc::c_int
                    & -((32767i32 < (ldexp(d, 5i32) + 0.5f64) as libc::c_int)
                      as libc::c_int))
                  > -32768i32) as libc::c_int));
            assert!(rate >= 0);
            assert!(rmse >= 0);
            dr += abs(
              rate
                - (*_table.offset(qi as isize))[pli as usize][qti as usize]
                  [si as usize]
                  .rate as libc::c_int
            );
            dd += abs(
              rmse
                - (*_table.offset(qi as isize))[pli as usize][qti as usize]
                  [si as usize]
                  .rmse as libc::c_int
            );
            (*_table.offset(qi as isize))[pli as usize][qti as usize]
              [si as usize]
              .rate = rate as ogg_int16_t;
            (*_table.offset(qi as isize))[pli as usize][qti as usize]
              [si as usize]
              .rmse = rmse as ogg_int16_t;
            si += 1
          }
          qi += 1
        }
        qti += 1
      }
      pli += 1
    }
    if !((dr > 0i32 || dd > 0i32) && {
      let fresh5 = niters;
      niters = niters + 1;
      fresh5 < _niters_min || dr < prevdr && dd < prevdd
    }) {
      break;
    }
  }
  if 0 != _reweight {
    pli = 0i32;
    while pli < 3i32 {
      qti = 0i32;
      while qti < 2i32 {
        qi = 0i32;
        while qi < 8i32 {
          si = 0i32;
          while si < 24i32 {
            let mut wt: libc::c_double = 0.;
            wt = (*_weight.offset(qi as isize))[pli as usize][qti as usize]
              [si as usize];
            wt /= 0.25f64 + wt;
            (*_table.offset(qi as isize))[pli as usize][qti as usize]
              [si as usize]
              .rate = ((*_table.offset(qi as isize))[pli as usize]
              [qti as usize][si as usize]
              .rate as libc::c_int as libc::c_double
              * wt
              + 0.5f64) as ogg_int16_t;
            (*_table.offset(qi as isize))[pli as usize][qti as usize]
              [si as usize]
              .rmse = ((*_table.offset(qi as isize))[pli as usize]
              [qti as usize][si as usize]
              .rmse as libc::c_int as libc::c_double
              * wt
              + 0.5f64) as ogg_int16_t;
            si += 1
          }
          qi += 1
        }
        qti += 1
      }
      pli += 1
    }
  };
}

/*Dump the in memory mode metrics to a file.
Note this data format isn't portable between different platforms.*/
#[no_mangle]
pub unsafe extern fn oc_mode_metrics_dump(mode_metrics_satd: *const [[[[oc_mode_metrics; OC_COMP_BINS]; 2]; 3]; OC_LOGQ_BINS-1]) {
  let mut fmetrics: *mut FILE = 0 as *mut FILE;
  let filename = libc::getenv(b"OD_MODE_METRICS_FILENAME\x00" as *const u8 as *const libc::c_char);
  fmetrics = fopen(
    filename,
    b"wb\x00" as *const u8 as *const libc::c_char
  );
  if !fmetrics.is_null() {
    fwrite(
      OC_MODE_LOGQ.as_ptr() as *const libc::c_void,
      ::std::mem::size_of::<[[[ogg_int16_t; 2]; 3]; 8]>() as libc::c_ulong,
      1i32 as libc::c_ulong,
      fmetrics
    );
    fwrite(
      mode_metrics_satd as *const libc::c_void,
      ::std::mem::size_of::<[[[[oc_mode_metrics; 24]; 2]; 3]; 7]>()
        as libc::c_ulong,
      1i32 as libc::c_ulong,
      fmetrics
    );
    fclose(fmetrics);
  };
}
