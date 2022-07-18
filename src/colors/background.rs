use checkers::utils;

struct IBackgroundColors {
    pub BgBlack: String,
    pub BgRed: String,
	pub BgGreen: String,
	pub BgYellow: String,
	pub BgBlue: String,
	pub BgMagenta: String,
	pub BgCyan: String,
	pub BgWhite: String
}

pub fn background_colors(string: &str) -> IBackgroundColors {
    let Colors = IBackgroundColors {
        BgBlack: basics::initiate(40, 49, "", string),
		BgRed: basics::initiate(41, 49, "", string),
		BgGreen: basics::initiate(42, 49, "", string),
		BgYellow: basics::initiate(43, 49, "", string),
		BgBlue: basics::initiate(44, 49, "", string),
		BgMagenta: basics::initiate(45, 49, "", string),
		BgCyan: basics::initiate(46, 49, "", string),
		BgWhite: basics::initiate(47, 49, "", string),
    };

	return Colors;
}