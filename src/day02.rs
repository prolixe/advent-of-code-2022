#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug)]
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

pub fn day02() {
    let contents = include_str!("../resources/day02.txt");
    let games = parse(contents);
    for game in games.iter() {
        println!("Game: {:?}, Outcome: {:?}", game, game.outcome())
    }
    let total_score: i32 = games.iter().map(score).sum();
    println!("Day02: Part 1: Total score: {}", total_score);
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
