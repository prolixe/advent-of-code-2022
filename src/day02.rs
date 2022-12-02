#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
enum Outcome {
    Lost,
    Draw,
    Win,
}

#[derive(Debug)]
struct Game {
    opponent: Shape,
    player: Shape,
}

#[derive(Debug)]
struct Gamev2 {
    opponent: Shape,
    outcome: Outcome,
}

impl Game {
    fn outcome(&self) -> Outcome {
        if self.opponent == self.player {
            return Outcome::Draw;
        }
        if self.player == Shape::Scissor && self.opponent == Shape::Rock {
            return Outcome::Lost;
        }
        if self.player == Shape::Rock && self.opponent == Shape::Scissor {
            return Outcome::Win;
        }
        if self.player > self.opponent {
            return Outcome::Win;
        }
        Outcome::Lost
    }
}

impl Gamev2 {
    fn played_shape(&self) -> Shape {
        if self.outcome == Outcome::Draw {
            return self.opponent;
        }
        if self.outcome == Outcome::Win {
            return match self.opponent {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissor,
                Shape::Scissor => Shape::Rock,
            };
        }
        if self.outcome == Outcome::Lost {
            return match self.opponent {
                Shape::Rock => Shape::Scissor,
                Shape::Paper => Shape::Rock,
                Shape::Scissor => Shape::Paper,
            };
        }
        panic!("should not reach here!");
    }
}

pub fn day02() {
    let contents = include_str!("../resources/day02.txt");
    let games = parse(contents);
    let total_score: i32 = games.iter().map(score).sum();
    println!("Day02: Part 1: Total score: {}", total_score);
    let games_v2 = parse_v2(contents);
    let total_score_v2: i32 = games_v2.iter().map(score_v2).sum();
    println!("Day02: Part 2: Total score: {}", total_score_v2);
}

fn parse(contents: &str) -> Vec<Game> {
    contents.trim().split('\n').map(parse_row).collect()
}

fn parse_row(row: &str) -> Game {
    let letters: Vec<char> = row.trim().chars().collect();
    let opponent_letter = letters[0];
    let player_letter = letters[2];
    let opponent = match_letter(opponent_letter);
    let player = match_letter(player_letter);
    Game { opponent, player }
}

fn parse_v2(contents: &str) -> Vec<Gamev2> {
    contents.trim().split('\n').map(parse_row_v2).collect()
}

fn parse_row_v2(row: &str) -> Gamev2 {
    let letters: Vec<char> = row.trim().chars().collect();
    let opponent_letter = letters[0];
    let outcome_letter = letters[2];
    let opponent = match_letter(opponent_letter);
    let outcome = match_outcome(outcome_letter);
    Gamev2 { opponent, outcome }
}

fn match_letter(letter: char) -> Shape {
    match letter {
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        'C' => Shape::Scissor,
        'X' => Shape::Rock,
        'Y' => Shape::Paper,
        'Z' => Shape::Scissor,
        _ => panic!("Not a valid letter! '{:?}'", letter),
    }
}

fn match_outcome(letter: char) -> Outcome {
    match letter {
        'X' => Outcome::Lost,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!("Not a valid letter! '{:?}'", letter),
    }
}

fn score(game: &Game) -> i32 {
    let shape_score = match game.player {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissor => 3,
    };
    let round_score: i32 = match game.outcome() {
        Outcome::Lost => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    };
    shape_score + round_score
}

fn score_v2(game: &Gamev2) -> i32 {
    let shape_score = match game.played_shape() {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissor => 3,
    };
    let round_score: i32 = match game.outcome {
        Outcome::Lost => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    };
    shape_score + round_score
}
