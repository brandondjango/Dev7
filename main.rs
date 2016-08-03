//Brandon Lockridge
//Rust RockPaperScissors

use std::io;

//player input
enum Choice {
	Rock,
	Paper,
	Scissors,
	Quit,
	Invalid
}

//outcome of a game
enum Outcome {
	Win,
	Loss,
	Tie,
	Invalid
}

//keep track of things
#[derive(Copy, Clone)]
struct Stats {
	numRocks: u32,
	numPapers: u32,
	numScissors: u32,
	Wins: u32,
	Losses: u32,
	Ties: u32
}

// return the correct choice based on user input
fn getPlayerInput() -> Choice {
	println!("Rock(r), Paper(p), Scissors(s), or Quit(q)");
	let mut choice = String::new();
	io::stdin().read_line(&mut choice);
	
	match choice.trim() {
		"r" => Choice::Rock,
		"p" => Choice::Paper,
		"s" => Choice::Scissors,
		"q" => Choice::Quit,
		_ => Choice::Invalid
	}
}


fn getComputerInput(stats: Stats) -> Choice {
	
	if(stats.numRocks > stats.numScissors && stats.numRocks > stats.numPapers){Choice::Paper }
	else if(stats.numScissors > stats.numRocks && stats.numScissors > stats.numPapers){Choice::Rock }
	else if(stats.numPapers > stats.numRocks && stats.numPapers > stats.numScissors){Choice::Scissors}
	else {println!("Computer chooses Paper"); Choice::Paper }
		
	}
	
	// compare the player and cpu choices and return the outcome
fn getOutcome(player: Choice, computer: Choice) -> Outcome{
	match player {
		Choice::Rock => match computer {
			Choice::Scissors => Outcome::Win,
			Choice::Rock => Outcome::Tie,
			Choice::Paper => Outcome::Loss,			
			_ => Outcome::Invalid
		},
		Choice::Paper => match computer {
			Choice::Rock => Outcome::Win,
			Choice::Paper => Outcome::Tie,
			Choice::Scissors => Outcome::Loss,
			_ => Outcome::Invalid
		},
		Choice::Scissors => match computer {
			Choice::Paper => Outcome::Win,
			Choice::Scissors => Outcome::Tie,
			Choice::Rock => Outcome::Loss,
			_ => Outcome::Invalid
		},
		_ => Outcome::Invalid	
	}
	
	
} 



fn main() {
	//initialize all stats to zero for new game
	let mut stats: Stats = Stats { numRocks: 0, numPapers: 0, numScissors: 0,  Wins: 0, Losses: 0, Ties: 0};
	
	println!("This is a Rock Paper Scissors game played against a CPU");
	println!("To enter your choice of Rock, Paper, or Scissors, enter either r, p, or s. To quit and see your game statistics, enter q.");
	
	
	//loop for the game, get the user's choice, loops back to the begining in case invalid input, quits the loop if a quit command is recieved 
	loop {
		let input = getPlayerInput();
		
		match input {
			Choice::Invalid => {
				println!("Only r, p, s, and q are considered valid inputs");
				continue;										
			}, 	
			Choice::Quit => break,				
			_ => {}												
		}
		
		//End of Illegal input handling, quit handling, and stats for rock, papers, and scissors
		
		match input{
			Choice::Rock => {stats.numRocks = stats.numRocks +1},
			Choice::Paper => {stats.numPapers = stats.numPapers +1},
			Choice::Scissors => {stats.numScissors = stats.numScissors +1},
			_ => {}		
		}
		
		//Actual game
		// get cpu choice
		let mut computer = getComputerInput(stats);
		
		match computer{
			Choice::Rock => {println!("Computer chose Rock")},
			Choice::Paper => {println!("Computer chose Paper")},
			Choice::Scissors => {println!("Computer chose Scissors")},
			Choice::Quit =>{println!("quit")},
			Choice::Invalid => {println!("invalid")}		
		}
		
		//get the winner
		let mut outcome = getOutcome(input, computer);
		
		//stats on wins and losses
		match outcome {
			Outcome::Win => {println!("Player wins!"); stats.Wins = stats.Wins + 1},
			Outcome::Loss => {println!("Player loses!"); stats.Losses = stats.Losses + 1},
			Outcome::Tie => {println!("Player ties!"); stats.Ties = stats.Ties + 1},
			Outcome::Invalid => {}		
		}
		
		
		
		
		}//end loop
		
	println!("Player Stats:");
	let mut numGames: u32 = stats.Wins + stats. Losses + stats.Ties;
	
	println!("Player chose Rock {} times ", stats.numRocks);
	println!("Player chose Paper {} times ", stats.numPapers);
	println!("Player chose Scissors {} times ", stats.numScissors);
	println!("Player won {} times, and {:.2}% of the time  ", stats.Wins, (100 * stats.Wins / numGames));
	println!("Player loss {} times, and {:.2}% of the time ", stats.Losses, (100 * stats.Losses / numGames));
	println!("Player tied {} times, and {:.2}% of the time ", stats.Ties, (100 * stats.Ties / numGames));
	
	
	
	
	
	
	
	
	
	}//end main