use heuristic::{Heuristic, index_positions};
use board::Board;

pub struct OutOfRaw {
    positions: Box<[(isize, isize)]>,
}

impl Heuristic for OutOfRaw {
    fn new(expected: &Board) -> Self {
        Self{ positions: index_positions(expected) }
    }

    fn distance(&self, current: &Board) -> usize {
        let line_size = current.line_size as isize;
        let mut cost = 0;
        for (i, &tile) in current.data.iter().enumerate() {
            let i = i as isize;
            let (exp_x, exp_y) = self.positions[tile as usize];
            let (cur_x, cur_y) = (i % line_size, i / line_size);
            if exp_x != cur_x {
                cost += 1;
            }
            if exp_y != cur_y {
                cost += 1;
            }
        }
        cost
    }
}