
pub struct Game {
	response_history: Vec<String>,
	command_history: Vec<String>
}

impl Default for Game {
	fn default() -> Self {
		Self {
			response_history: vec![],
			command_history: vec![],
		}
	}
}

impl Game {
	pub fn get_history(&self) -> &Vec<String> {
		&self.response_history
	}
	
	pub fn get_user_inputs(&self) -> &Vec<String> {
		&self.command_history
	}
	
	pub fn send_command(&mut self, command: String) {
		self.command_history.push(command);
		// TODO: Kick off processing.
		self.response_history.push("Did a thing!".to_string());
	}
}