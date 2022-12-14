// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 9.  
//! Ben and Bart Massey 2022

use std::collections::HashSet;

use aoc::{*, reparse::*, geom::{*, Dirn::*}};

fn char_to_dirn(ch: char) -> Dirn {
    match ch {
        'L' => Left,
        'R' => Right,
        'U' => Up,
        'D' => Down,
        _ => panic!("unknown dirn {ch}"),
    }
}

fn clamp((tx, ty): &mut (i64, i64), (cx, cy): (i64, i64)) {
    let (ntx, nty) = if (*tx - cx).abs() <= 1 && (*ty - cy).abs() <= 1 {
        (*tx, *ty)
    } else if *tx != cx && *ty == cy {
        ((*tx).clamp(cx - 1, cx + 1), *ty)
    } else if *tx == cx && *ty != cy {
        (*tx, (*ty).clamp(cy - 1, cy + 1))
    } else if (*tx - cx).abs() > 1 {
        ((*tx).clamp(cx - 1, cx + 1), cy)
    } else if (*ty - cy).abs() > 1 {
        (cx, (*ty).clamp(cy - 1, cy + 1))
    } else {
        panic!("lost tail: t=({}, {}) c=({}, {})", *tx, *ty, cx, cy);
    };
    *tx = ntx;
    *ty = nty;
}

fn main() {
    let step_re = Reparse::new(r"(^[LRUD]) ([0-9]+)$");
    let path = input_lines()
        .map(|l| {
            let matches = step_re.parse(&l).unwrap();
            let dirn: char = matches.get(1);
            let dist: i64 = matches.get(2);
            (char_to_dirn(dirn), dist)
        });
    match get_part() {
        Part1 => {
            let mut visited = HashSet::new();
            let mut h_posn = (0, 0);
            let mut t_posn = (0, 0);
            for (dirn, dist) in path {
                let disp: (i64, i64) = dirn.disp();
                for _ in 0..dist {
                    h_posn.0 += disp.0;
                    h_posn.1 += disp.1;
                    clamp(&mut t_posn, h_posn);
                    visited.insert(t_posn);
                }
            }
            println!("{}", visited.len());
        }
        Part2 => todo!(),
    }
}
