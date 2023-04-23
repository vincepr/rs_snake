use std::collections::{HashSet, VecDeque};

/*
   Logic of the snake game itself (excluding timers/user-inputs)
*/

/// Points on the Game-Grid.
/// - (0,0) is not valid
/// - width a game field of width:3 height:2 all valid points:
///     - (1,1) (2,1) (3,1)
///     - (1,2) (2,1) (3,2)
pub type Point = (usize, usize);

#[derive(PartialEq, Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug)]
pub struct Game {
    gameover: bool,
    highscore: usize,
    pub width: usize,
    pub height: usize,
    pub snake: VecDeque<Point>, // Queue of Points. Head is s[0], End is s[s.len-1]:
    direction: Direction,       // direction on this frame:
    next_direction: Direction,  // direction on the next frame (so we can only change 90° per tick)
    pub food: HashSet<Point>,   // Hash Set of all Points that contain Food
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        let h: usize = height / 2; // starting height of snake
        let mut food = HashSet::new();
        food.insert((7, h));
        food.insert((2, 2));

        Self {
            gameover: false,
            highscore: 0,
            width,
            height,
            snake: VecDeque::from([(3, h), (2, h), (1, h)]),
            direction: Direction::Right,
            next_direction: Direction::Right,
            food: food,
        }
    }

    /// player input to change direction (on the next tick)
    pub fn direction_change(&mut self, dir: Direction) {
        match (&self.direction, dir) {
            // illegal moves:
            //(old, new) if *old==new => {},
            (Direction::Up, Direction::Down) => {}
            (Direction::Down, Direction::Up) => {}
            (Direction::Right, Direction::Left) => {}
            (Direction::Left, Direction::Right) => {}
            // legal move:
            (_, dir) => self.next_direction = dir,
        }
    }

    pub fn get_typ(&self, p: &Point) -> &'static str {
        if self.food.contains(p) {
            "🍓"
        } else if self.snake[0] == *p {
            "🐸"
        } else if self.snake.contains(p){
            "🟢"
        } else {
            " "
        }
    }

    pub fn get_score(&self) ->String{
        if !self.gameover{
            return format!("Points: {}", self.highscore)
        }
        format!("Game is Over! Points: {}", self.highscore)
    }

    /// Game Loop
    pub fn tick(&mut self) {
        if self.gameover {
            return;
        }
        // movement according to direction value:
        let mut new_head = self.snake[0];
        match &self.next_direction {
            Direction::Up => new_head.1 = (new_head.1 - 1).max(0), // to avoid underflowing uint we .min(0)
            Direction::Down => new_head.1 += 1,
            Direction::Right => new_head.0 += 1,
            Direction::Left => new_head.0 = (new_head.0 - 1).max(0),
        }
        self.direction = self.next_direction.clone();
        // check for collision with wall or collision with snake itself
        if self.is_out_of_bounds(new_head) || self.snake.contains(&new_head) {
            self.gameover = true;
            return;
        }
        self.snake.push_front(new_head);

        // check if we are eating food (if yes then we dont pop_back -> get bigger)
        if self.food.contains(&new_head) {
            let bonus = self.highscore / 100;
            self.highscore += 10 + bonus;
            // delete the "eaten" food and generate a new one:
            self.food.remove(&new_head);

            self.food.insert(self.random_new_food());
            // generate new food
        } else {
            self.snake.pop_back();
        }
    }

    // helper functions:
    // check if point out of bounds (for crashing into a wall etc.)
    fn is_out_of_bounds(&self, (x, y): Point) -> bool {
        x == 0 || y == 0 || x > self.width || y > self.height
    }

    // get a randomized valid location for the next food item.
    // this will crash when the whole board is a giant snake+food but whatever
    fn random_new_food(&self) -> Point {
        use crate::random::random_range as rng;
        // first we just guess a few times (fast performance)
        for _ in 0..30 {
            let r: Point = (rng(1, self.width), rng(1, self.height));
            if !self.food.contains(&r) && !self.snake.contains(&r) {
                return r;
            }
        }
        // once the board fills up with snake-body we do it the slow & proper-way (more mem allocation),
        // by getting all free positions and only rng-ing over those:
        let free_positions = (1..self.height)
            .flat_map(|y| {
                (1..self.width)
                    .map(move |x| (x, y))
                    .filter(|p| !self.snake.contains(&p) && !self.food.contains(&p))
            })
            .collect::<Vec<Point>>();
        if free_positions.is_empty() {
            // TODO: make set game over in this case? (we just remove the food by )
            // panic!("END OF GAME REACHED- every position is full with snake (or food), not handling this properly for now")
            // this will never be reached anyway, so we just remove the food by returning unreachable 0,0 (not rendered)
            return (0,0)
        }

        free_positions[rng(0, free_positions.len())]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut g = Game::new(15, 10);
        dbg!(&g);
        g.direction_change(Direction::Up);
        g.tick();
        dbg!(&g);
        dbg!(g.snake[0]);
    }
}
