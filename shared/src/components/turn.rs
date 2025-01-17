use std::collections::{HashSet, VecDeque};

use bevy_ecs::prelude::Component;
use log::info;

#[derive(Component, Default, Debug)]
pub struct Turn {
    pool: VecDeque<usize>,
    /// Players already have no cards
    players_out: HashSet<usize>,
    total_player: usize,
    leader_turn: usize,
}

impl Turn {
    pub fn new(total_player: usize) -> Self {
        let mut pool = VecDeque::new();
        let players_out = HashSet::new();

        for i in 0..total_player {
            pool.push_back(i);
        }

        Self {
            pool,
            total_player,
            players_out,
            leader_turn: 0,
        }
    }

    pub fn next_score(&self) -> u32 {
        let player_left = self.total_player - self.players_out.len();

        let next_score = match player_left {
            4 => 3,
            3 => 2,
            2 => 1,
            1 => 0,
            _ => unreachable!(),
        };

        next_score as u32
    }

    pub fn debug(&self) {
        info!("----------- TURN ---------------");
        info!("{:?}", self);
        info!("----------- END_TURN ---------------");
    }

    pub fn new_match(&mut self) {
        self.pool.clear();
        self.players_out.clear();

        for i in 0..self.total_player {
            self.pool.push_back(i);
        }
    }

    pub fn only_one_player_left(&self) -> bool {
        (self.total_player - self.players_out.len()) == 1
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
        let player_pos = self.pool.pop_front().unwrap();
        self.leader_turn = player_pos;
        self.pool.push_back(player_pos);
    }

    pub fn next_turn(&mut self) -> Option<usize> {
        self.make_move();
        self.current_active_player()
    }

    pub fn player_out(&mut self) -> usize {
        let player_pos = self.pool.pop_front().unwrap();
        self.players_out.insert(player_pos);

        self.debug();

        self.current_active_player().unwrap()
    }

    pub fn skip_turn(&mut self) -> (bool, Option<usize>) {
        // FIXME: crazy hack here!!!
        let mut leader_turn = false;

        let player_left = self.total_player - self.players_out.len();

        if player_left == 1 {
            return (leader_turn, self.current_active_player());
        }

        self.pool.pop_front().unwrap();

        if self.pool.len() == 1 {
            leader_turn = true;

            let mut last = (*self.pool.back().unwrap() + 1) % self.total_player;

            while self.pool.len() != player_left {
                if !self.players_out.contains(&last) {
                    self.pool.push_back(last);
                }
                last = (last + 1) % self.total_player;
            }
        }

        (leader_turn, self.current_active_player())
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
