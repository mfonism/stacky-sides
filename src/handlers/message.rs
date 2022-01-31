#[derive(Debug)]
pub enum GameMessage {
    // -- OUTGOING MESSAGES
    // state_str is in the form -- state [[...], [...], ..., [...]]
    Board { state_str: String },
    // ending_str is in the form -- end x
    // where x is either 1 or 2, representing which player won
    // x is 0 in the case of a draw
    End { ending_str: String },
    // -- INCOMING MESSAGES
    Selection { row: u8, col: u8 },
}

impl GameMessage {
    pub fn read(text: String) -> Result<Self, &'static str> {
        let text = text.to_lowercase();

        if text.starts_with("board") {
            return Ok(Self::Board { state_str: text });
        }

        if text.starts_with("end") {
            return Ok(Self::End { ending_str: text });
        }

        // validate and process incoming messages
        let parts = text.split(" ").collect::<Vec<&str>>();
        if parts.len() == 0 {
            return Err("empty message!");
        }

        if parts[0] == "selection" && parts.len() == 3 {
            if let (Ok(row), Ok(col)) = (parts[1].parse(), parts[2].parse()) {
                return Ok(GameMessage::Selection { row, col });
            }
            return Err("could not parse selection message");
        }

        Err("could not parse message")
    }
}
