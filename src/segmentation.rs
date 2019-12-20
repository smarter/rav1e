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

use criterion_stats::univariate::*;
use kde::*;

pub fn segmentation_optimize<T: Pixel>(
  fi: &FrameInvariants<T>, fs: &mut FrameState<T>,
) {
  assert!(fi.enable_segmentation);
  fs.segmentation.enabled = true;

  if fs.segmentation.enabled {
    fs.segmentation.update_map = true;

    // We don't change the values between frames.
    fs.segmentation.update_data = fi.primary_ref_frame == PRIMARY_REF_NONE;

    let mut qs = vec!();

    let base_q = ac_q(fi.base_q_idx, 0, fi.config.bit_depth);
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
        let scaled_q = (base_q as f64)*q_scale;
        // let scaled_q_idx = select_ac_qi(scaled_q asi 64, fi.config.bit_depth);
        // deltas.push(scaled_q_idx as i16 - fi.base_q_idx as i16);
        qs.push(scaled_q);
      }
    }

    // Clustering technique inspired by https://stackoverflow.com/a/35151947/12430076
    
    let sample = Sample::new(&qs);
    // dbg!(sample.min(), sample.mean(), sample.max());
    let kfun = Kde::new(sample, kernel::Gaussian, Bandwidth::Silverman);
    let min_q = sample.min().floor() as i64;
    let max_q = sample.max().ceil() as i64;
    let mut qi_deltas = vec!();
    for q in min_q..=max_q {
      let qf = q as f64;
      let density = kfun.estimate(qf);
      if density > kfun.estimate(qf - 1.) && density > kfun.estimate(qf + 1.) {
        let qi = select_ac_qi(q, fi.config.bit_depth) as i16;
        qi_deltas.push(qi - fi.base_q_idx as i16);
      }
    }

    // // dbg!(kfun.estimate(10.), kfun.estimate(50.), kfun.estimate(60.), kfun.estimate(100.));
    // let ivs: Vec<i64> = (0..=200).collect();
    // let vs: Vec<f64> = ivs.iter().map(|&x| 18. + (x as f64)/6.0).collect();
    // println!("{:?}", kfun.map(&vs));

    if !fs.segmentation.update_data {
      return;
    }

    // Avoid going into lossless mode by never bringing qidx below 1.
    // Because base_q_idx changes more frequently than the segmentation
    // data, it is still possible for a segment to enter lossless, so
    // enforcement elsewhere is needed.
    let offset_lower_limit = 1 - fi.base_q_idx as i16;

    for (i, &delta) in qi_deltas.iter().take(8).enumerate() {
      fs.segmentation.features[i][SegLvl::SEG_LVL_ALT_Q as usize] = true;
      fs.segmentation.data[i][SegLvl::SEG_LVL_ALT_Q as usize] = delta;
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
