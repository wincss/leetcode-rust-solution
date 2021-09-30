use crate::*;

impl Solution {
    pub fn compute_area(
        ax1: i32,
        ay1: i32,
        ax2: i32,
        ay2: i32,
        bx1: i32,
        by1: i32,
        bx2: i32,
        by2: i32,
    ) -> i32 {
        // +---+
        // | +-+-+
        // | | | |
        // +-+-+ |
        //   +---+

        // +---+
        // | +-+-+
        // | | | |
        // | +-+-|
        // +---|

        // +-----+
        // | +-+ |
        // | | | |
        // + +-+ |
        // +-----+

        //   +-+
        // +-+-+-+
        // | | | |
        // +-+-+-|
        //   +-+

        let area1 = (ax2 - ax1) * (ay2 - ay1);
        let area2 = (bx2 - bx1) * (by2 - by1);
        let overlay_x = 0.max(ax2.min(bx2) - ax1.max(bx1));
        let overlay_y = 0.max(ay2.min(by2) - ay1.max(by1));
        area1 + area2 - overlay_x * overlay_y
    }
}
