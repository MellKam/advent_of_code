#[derive(Debug, PartialEq, Copy, Clone)]
enum MoveType {
	Rock = 1,
	Paper = 2,
	Scissors = 3,
}

impl From<i32> for MoveType {
	fn from(num: i32) -> Self {
		match num {
			1 => MoveType::Rock,
			2 => MoveType::Paper,
			3 => MoveType::Scissors,
			_ => panic!("Cannot convert '{num}' to MoveType"),
		}
	}
}

const MOVE_COUNTS: i32 = 3;

#[derive(Debug, PartialEq, Copy, Clone)]
enum GameResult {
	Draw,
	Lose,
	Win,
}

impl GameResult {
	fn get_score(&self) -> i32 {
		return match *self {
			GameResult::Win => 6,
			GameResult::Draw => 3,
			GameResult::Lose => 0,
		};
	}
}

fn parse_move(string_move: &str) -> MoveType {
	match string_move {
		"A" | "X" => MoveType::Rock,
		"B" | "Y" => MoveType::Paper,
		"C" | "Z" => MoveType::Scissors,
		_ => panic!("Cannot parse move type"),
	}
}

fn parse_game_result(string_game_result: &str) -> GameResult {
	match string_game_result {
		"X" => GameResult::Lose,
		"Y" => GameResult::Draw,
		"Z" => GameResult::Win,
		_ => panic!("Cannot parse game result"),
	}
}

fn get_player_game_score(opponent_move: MoveType, player_move: MoveType) -> i32 {
	let mut score: i32 = 0;
	let player_move_score = player_move as i32;

	if opponent_move == player_move {
		// Draw
		score += GameResult::Draw.get_score() + player_move_score;
		return score;
	}

	if ((opponent_move as i32) - player_move_score).rem_euclid(MOVE_COUNTS) == 1 {
		// Lose
		score += GameResult::Lose.get_score() + player_move_score;
	} else {
		// Win
		score += GameResult::Win.get_score() + player_move_score;
	}

	return score;
}

fn get_move_by_game_result(opponent_move: MoveType, expected_game_result: GameResult) -> MoveType {
	if expected_game_result == GameResult::Draw {
		return opponent_move;
	};

	let inverted_result =
		(expected_game_result as i32 - opponent_move as i32).rem_euclid(MOVE_COUNTS) * -1;

	if inverted_result == 0 {
		return MoveType::from(MOVE_COUNTS);
	}

	return MoveType::from(inverted_result.rem_euclid(MOVE_COUNTS));
}

fn main() {
	let sum = include_str!("../../input.txt")
		.lines()
		.map(|line| {
			let (opponent, game_result) = match line.split_once(" ") {
				Some((o, r)) => (o, r),
				None => panic!("Error while splitting line"),
			};

			let opponent_move = parse_move(opponent);
			let player_move = get_move_by_game_result(opponent_move, parse_game_result(game_result));

			return get_player_game_score(opponent_move, player_move);
		})
		.sum::<i32>();

	println!("{sum}");
}
