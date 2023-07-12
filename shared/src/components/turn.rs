use std::collections::VecDeque;

use bevy_ecs::prelude::Component;
use log::info;

#[derive(Component, Default, Debug)]
pub struct Turn {
    pool: VecDeque<usize>,
    total_player: usize,
}

impl Turn {
    pub fn new(total_player: usize) -> Self {
        let mut queue = VecDeque::new();

        for i in 0..total_player {
            queue.push_back(i);
        }

        Self {
            pool: queue,
            total_player,
        }
    }

    pub fn new_player_join(&mut self) {
        if self.total_player > 3 {
            info!("This game only able for 4 players");
            return;
        }
        self.pool.push_back(self.pool.len());
        self.total_player += 1;
    }

    pub fn current_active_player(&mut self) -> Option<usize> {
        self.pool.front().copied()
    }

    pub fn make_move(&mut self) {
        let a = self.pool.pop_front().unwrap();
        self.pool.push_back(a);
    }

    pub fn next_turn(&mut self) -> Option<usize> {
        self.make_move();
        self.current_active_player()
    }

    pub fn skip_turn(&mut self) -> Option<usize> {
        self.pool.pop_front().unwrap();
        self.recalculate_turn();

        self.current_active_player()
    }

    pub fn calculate_turn(&mut self, first_player_pos: usize) {
        self.pool.clear();

        self.pool.push_back(first_player_pos);

        for _ in 0..self.total_player - 1 {
            let next_p = (first_player_pos + 1) % self.total_player;
            self.pool.push_back(next_p);
        }
    }

    pub fn recalculate_turn(&mut self) {
        if self.pool.len() == 1 {
            for _ in 0..self.total_player - 1 {
                let next_p = (*self.pool.back().unwrap() + 1) % self.total_player;
                self.pool.push_back(next_p);
            }
        }
    }
}
