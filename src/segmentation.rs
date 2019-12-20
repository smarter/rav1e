// Copyright (c) 2018-2019, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::context::*;
use crate::header::PRIMARY_REF_NONE;
use crate::util::Pixel;
use crate::quantize::*;
use crate::FrameInvariants;
use crate::FrameState;

pub fn segmentation_optimize<T: Pixel>(
  fi: &FrameInvariants<T>, fs: &mut FrameState<T>,
) {
  assert!(fi.enable_segmentation);
  fs.segmentation.enabled = true;

  if fs.segmentation.enabled {
    fs.segmentation.update_map = true;

    // We don't change the values between frames.
    fs.segmentation.update_data = fi.primary_ref_frame == PRIMARY_REF_NONE;

    let mut deltas = vec!();

    for y in 0..fi.h_in_imp_b {
      for x in 0..fi.w_in_imp_b {
        let intra_cost = fi.lookahead_intra_costs[y * fi.w_in_imp_b + x] as f64;
        let propagate_cost = fi.block_importances[y * fi.w_in_imp_b + x] as f64;
        let q_scale =
          if intra_cost == 0. {
            1.
          } else {
            let strength = 1.0;
            let frac = (intra_cost + propagate_cost) / intra_cost;
            frac.powf(-strength / 6.0)
          };
        let base_q = ac_q(fi.base_q_idx, 0, fi.config.bit_depth);
        let scaled_q = ((base_q as f64)*q_scale).round() as i64;
        let scaled_q_idx = select_ac_qi(scaled_q, fi.config.bit_depth);
        deltas.push(scaled_q_idx as i16 - fi.base_q_idx as i16);
      }
    }

    if !fs.segmentation.update_data {
      return;
    }

    // A series of AWCY runs with deltas 13, 15, 17, 18, 19, 20, 21, 22, 23
    // showed this to be the optimal one.
    const TEMPORAL_RDO_QI_DELTA: i16 = 21;

    // Avoid going into lossless mode by never bringing qidx below 1.
    // Because base_q_idx changes more frequently than the segmentation
    // data, it is still possible for a segment to enter lossless, so
    // enforcement elsewhere is needed.
    let offset_lower_limit = 1 - fi.base_q_idx as i16;

    // Fill in 3 slots with 0, delta, -delta. The slot IDs are also used in
    // luma_chroma_mode_rdo() so if you change things here make sure to check
    // that place too.
    for i in 0..3 {
      fs.segmentation.features[i][SegLvl::SEG_LVL_ALT_Q as usize] = true;
      fs.segmentation.data[i][SegLvl::SEG_LVL_ALT_Q as usize] = match i {
        0 => 0,
        1 => TEMPORAL_RDO_QI_DELTA,
        2 => (-TEMPORAL_RDO_QI_DELTA).max(offset_lower_limit),
        _ => unreachable!(),
      };
    }

    /* Figure out parameters */
    fs.segmentation.preskip = false;
    fs.segmentation.last_active_segid = 0;
    if fs.segmentation.enabled {
      for i in 0..8 {
        for j in 0..SegLvl::SEG_LVL_MAX as usize {
          if fs.segmentation.features[i][j] {
            fs.segmentation.last_active_segid = i as u8;
            if j >= SegLvl::SEG_LVL_REF_FRAME as usize {
              fs.segmentation.preskip = true;
            }
          }
        }
      }
    }
  }
}
