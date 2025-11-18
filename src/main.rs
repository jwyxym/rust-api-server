mod send;
mod ureq;

use chrono::Local;
use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::{
	HttpServer,
	HttpRequest,
	Responder,
	Error,
	App,
	get,
	dev,
	web,
	error:: {
		ErrorNotFound
	}
};
use std::{
	fs::{
		read_to_string
	},
	io::{
		Error as std_Error
	}
};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
struct Config {
	pics: Vec<Item>,
	text: Vec<Item>,
	file: Vec<FileItem>,
	string: Vec<StringItem>,
	web: Vec<WebItem>
}

#[derive(Clone, Debug, Deserialize)]
struct Item {
	key: String,
	path: String
}

#[derive(Clone, Debug, Deserialize)]
struct WebItem {
	key: String,
	path: String
}

#[derive(Clone, Debug, Deserialize)]
struct StringItem {
	key: String,
	text: String
}

#[derive(Clone, Debug, Deserialize)]
struct FileItem {
	key: String,
	path: String,
	name: String
}

#[get("/string/{key}")]
async fn string(req: HttpRequest, config: web::Data<Config>) -> Result<impl Responder, Error> {
	let req: &dev::Path<dev::Url>= req.match_info();
	let key: String = req.query("key").to_string();
    println!("[{}]: /string/{}", Local::now(), key);
	match config.string.iter().find(|i| i.key.to_lowercase() == key) {
		Some(item) => send::string(item.text.clone()),
		None => Err(ErrorNotFound("String not found".to_string()))
	}
}

#[get("/text/{key}")]
async fn text(req: HttpRequest, config: web::Data<Config>) -> Result<impl Responder, Error> {
	let req: &dev::Path<dev::Url>= req.match_info();
	let key: String = req.query("key").to_string();
    println!("[{}]: /text/{}", Local::now(), key);
	match config.text.iter().find(|i| i.key.to_lowercase() == key) {
		Some(item) => send::text(item.path.clone()),
		None => Err(ErrorNotFound("File not found".to_string()))
	}
}

#[get("/file/{key}")]
async fn file(req: HttpRequest, config: web::Data<Config>) -> Result<NamedFile, Error> {
	let req: &dev::Path<dev::Url>= req.match_info();
	let key: String = req.query("key").to_string();
    println!("[{}]: /file/{}", Local::now(), key);
	match config.file.iter().find(|i| i.key.to_lowercase() == key) {
		Some(item) => send::file(item.path.clone(), item.name.clone()),
		None => Err(ErrorNotFound("File not found".to_string()))
	}
}

#[get("/pics/{key}/{name}")]
async fn pics(req: HttpRequest, config: web::Data<Config>) -> Result<NamedFile, Error> {
	let req: &dev::Path<dev::Url>= req.match_info();
	let key: String = req.query("key").to_string();
	let name: String = req.query("name").to_string();
    println!("[{}]: /pics/{}/{}", Local::now(), key, name);
	let path = match config.pics.iter().find(|i| i.key.to_lowercase() == key) {
		Some(item) => item.path.clone(),
		None => "File not found".to_string()
	};
	send::pics(path, name)
}

#[get("/web/{key}")]
async fn http(req: HttpRequest, config: web::Data<Config>) -> Result<impl Responder, Error> {
	let req: &dev::Path<dev::Url>= req.match_info();
	let key: String = req.query("key").to_string();
    println!("[{}]: /web/{}", Local::now(), key);
	match config.web.iter().find(|i| i.key.to_lowercase() == key) {
		Some(item) => {
			let t = ureq::get(item.path.clone()).await;
			send::string(t)
		}
		None => Err(ErrorNotFound("Web Error".to_string()))
	}
}

#[actix_web::main]
async fn main() -> Result<(), std_Error> {
	match read_to_string("config.json") {
		Ok(content) => {
			let config: Config = serde_json::from_str(&content)?;
			HttpServer::new(move || {
				App::new()
					.app_data(web::Data::new(config.clone()))
					.wrap(Cors::permissive())
					.service(string)
					.service(text)
					.service(file)
					.service(pics)
					.service(http)
			})
			.bind("127.0.0.1:8082")?
			.run()
			.await
		},
		Err(e) => Err(e)
	}
}
