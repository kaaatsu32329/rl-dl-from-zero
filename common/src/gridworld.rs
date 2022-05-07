use std::f64::INFINITY;

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub struct GridWorld<'a> {
    action_space: [u8; 4],
    action_meaning: [&'a str; 4],
    reward_map: [[f64; 4]; 3],
    goal_state: (i32, i32),
    wall_state: (i32, i32),
    start_state: (i32, i32),
    agent_state: (i32, i32),
}

#[allow(unused)]
impl GridWorld<'_> {
    pub fn new() -> Self {
        let ninf = -1. * std::f64::INFINITY;
        Self {
            action_space: [0, 1, 2, 3],
            action_meaning: ["UP", "DOWN", "LEFT", "RIGHT"],
            reward_map: [[0., 0., 0., 1.], [0., ninf, 0., -1.], [0., 0., 0., 0.]],
            goal_state: (0, 3),
            wall_state: (1, 1),
            start_state: (2, 0),
            agent_state: (2, 0),
        }
    }

    pub fn height(self) -> usize {
        self.reward_map.len()
    }

    pub fn width(self) -> usize {
        self.reward_map[0].len()
    }

    fn shape(self) -> (usize, usize) {
        (self.reward_map.len(), self.reward_map[0].len())
    }

    fn action(self) -> [u8; 4] {
        self.action_space
    }

    pub fn states(&self) -> Vec<(usize, usize)> {
        let mut v = vec![];
        for h in 0..self.height() {
            for w in 0..self.width() {
                v.push((h, w));
            }
        }
        v
    }

    fn next_state(&self, state: (i32, i32), action: usize) -> (i32, i32) {
        let action_move_map = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let agent_move = action_move_map[action];
        let mut next_state = (state.0 + agent_move.0, state.1 + agent_move.1);
        let (ny, nx) = next_state;

        if nx < 0 || nx >= self.width() as i32 || ny < 0 || ny >= self.height() as i32 {
            next_state = state;
        } else if next_state == self.wall_state {
            next_state = state
        }
        next_state
    }

    fn reward(self, next_state: (i32, i32)) -> f64 {
        self.reward_map[next_state.0 as usize][next_state.1 as usize]
    }

    pub fn render_v(self, v: f64, policy: f64, print_value: bool) -> () {}
}
