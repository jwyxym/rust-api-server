use serde::Deserialize;
#[derive(Clone, Debug, Deserialize)]
pub struct Config {
	pics: Vec<Item>,
	text: Vec<Item>,
	file: Vec<FileItem>,
	string: Vec<StringItem>,
	web: Vec<WebItem>,
	user: Vec<User>,
	upload: String
}

impl Config {
	pub fn pics(&self) -> Vec<Item> {
		self.pics.clone()
	}

	pub fn text(&self) -> Vec<Item> {
		self.text.clone()
	}

	pub fn file(&self) -> Vec<FileItem> {
		self.file.clone()
	}

	pub fn string(&self) -> Vec<StringItem> {
		self.string.clone()
	}

	pub fn web(&self) -> Vec<WebItem> {
		self.web.clone()
	}

	pub fn user(&self) -> Vec<User> {
		self.user.clone()
	}

	pub fn upload(&self) -> String {
		self.upload.clone()
	}
}

#[derive(Clone, Debug, Deserialize)]
pub struct Item {
	key: String,
	path: String
}
impl Item {
	pub fn chk(&self, key: String) -> bool {
		self.key.to_lowercase() == key.to_lowercase()
	}

	pub fn path(&self) -> String {
		self.path.clone()
	}
}

#[derive(Clone, Debug, Deserialize)]
pub struct WebItem {
	key: String,
	path: String
}

impl WebItem {
	pub fn chk(&self, key: String) -> bool {
		self.key.to_lowercase() == key.to_lowercase()
	}

	pub fn path(&self) -> String {
		self.path.clone()
	}
}

#[derive(Clone, Debug, Deserialize)]
pub struct StringItem {
	key: String,
	text: String
}

impl StringItem {
	pub fn chk(&self, key: String) -> bool {
		self.key.to_lowercase() == key.to_lowercase()
	}

	pub fn text(&self) -> String {
		self.text.clone()
	}
}

#[derive(Clone, Debug, Deserialize)]
pub struct FileItem {
	key: String,
	path: String,
	name: String
}

impl FileItem {
	pub fn chk(&self, key: String) -> bool {
		self.key.to_lowercase() == key.to_lowercase()
	}

	pub fn name(&self) -> String {
		self.name.clone()
	}

	pub fn path(&self) -> String {
		self.path.clone()
	}
}

#[derive(Clone, Debug, Deserialize)]
pub struct User {
	name: String,
	password: String
}

impl User {
	pub fn chk(&self, name: String, password: String) -> bool {
		self.name.to_lowercase() == name.to_lowercase() && self.password.to_lowercase() == password.to_lowercase()
	}

	pub fn name(&self) -> String {
		self.name.clone()
	}

	pub fn password(&self) -> String {
		self.password.clone()
	}
}