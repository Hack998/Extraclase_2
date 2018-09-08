trait Play {
	fn play(&self);
}

struct IPlayer ();

impl Play for IPlayer{
	fn play(&self){
	}
}

trait OldPlay {
    fn oldPlay(&self);
}

struct OldPlayer{
	song : String,
	volume : i32
}

impl OldPlayer {
	fn new(song: &str, volume: &i32) -> OldPlayer {
		OldPlayer {
			song: song.to_string(),
			volume: *volume
		}
	}
}

impl OldPlay for OldPlayer {
    fn oldPlay(&self){
        println!("Playing song: {0} at volume: {1}", self.song, self.volume);
    }
}

struct Adapter {
	player: OldPlayer
}

impl Adapter {
	fn new(song: &str, volume: &i32) -> Adapter {
		Adapter {
			player : OldPlayer::new(song, volume)
		}
	}
}

impl Play for Adapter {
	fn play(&self){
		self.player.oldPlay();
	}
}

fn main() {
	let player = Adapter::new("Almost Love", &50);
	player.play();
}
