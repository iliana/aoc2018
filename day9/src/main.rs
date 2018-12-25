use std::fmt;

#[derive(Debug, Clone)]
struct Game {
    marbles: Vec<u32>,
    cursor: usize,
    next_marble: u32,
    players: Vec<u32>,
    next_player: usize,
}

impl Game {
    fn new(players: usize, last: u32) -> Game {
        Game {
            marbles: {
                let mut v = Vec::with_capacity(last as usize);
                v.push(0);
                v
            },
            cursor: 0,
            next_marble: 1,
            players: vec![0; players],
            next_player: 0,
        }
    }

    fn next(&mut self) {
        if self.next_marble % 23 == 0 {
            self.cursor = (self.marbles.len() + self.cursor - 7) % self.marbles.len();
            self.players[self.next_player] += self.next_marble + self.marbles.remove(self.cursor);
        } else {
            self.cursor = if self.marbles.len() == 1 {
                1
            } else {
                let cursor = (self.cursor + 2) % self.marbles.len();
                if cursor == 0 {
                    self.marbles.len()
                } else {
                    cursor
                }
            };
            self.marbles.insert(self.cursor, self.next_marble);
        }
        self.next_marble += 1;
        self.next_player = (self.next_player + 1) % self.players.len();
        //println!("{}", self);
    }

    fn high_score(&mut self) -> u32 {
        *self.players.iter().max().unwrap()
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}]",
            (self.players.len() + self.next_player - 1) % self.players.len() + 1
        )?;
        for (i, marble) in self.marbles.iter().enumerate() {
            if i == self.cursor {
                write!(f, "({})", marble)?;
            } else {
                write!(f, " {} ", marble)?;
            }
        }
        Ok(())
    }
}

fn play_game(players: usize, last: u32) -> u32 {
    let mut game = Game::new(players, last);
    for _ in 0..last {
        game.next();
    }
    game.high_score()
}

fn main() {
    println!("Part 1: {}", play_game(405, 71700));
    println!("Part 2: {}", play_game(405, 71700 * 100));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn check() {
        assert_eq!(play_game(9, 25), 32);
        assert_eq!(play_game(10, 1618), 8317);
        assert_eq!(play_game(13, 7999), 146373);
        assert_eq!(play_game(17, 1104), 2764);
        assert_eq!(play_game(21, 6111), 54718);
        assert_eq!(play_game(30, 5807), 37305);
    }
}
