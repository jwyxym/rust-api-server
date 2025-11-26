mod send;
mod ureq;
mod write;
mod structs;

use chrono::Local;
use actix_multipart::Multipart;
use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::{
	HttpServer,
	HttpRequest,
	HttpResponse,
	Responder,
	Error,
	App,
	get,
	post,
	dev,
	web,
	error::{
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

#[get("/string/{key}")]
async fn string(req: HttpRequest, config: web::Data<structs::Config>) -> Result<impl Responder, Error> {
	let req: &dev::Path<dev::Url>= req.match_info();
	let key: String = req.query("key").to_string();
    println!("[{}]: /string/{}", Local::now(), key);
	match config.string().iter().find(|i| i.chk(key.clone())) {
		Some(item) => send::string(item.text()),
		None => Err(ErrorNotFound("String not found".to_string()))
	}
}

#[get("/text/{key}")]
async fn text(req: HttpRequest, config: web::Data<structs::Config>) -> Result<impl Responder, Error> {
	let req: &dev::Path<dev::Url>= req.match_info();
	let key: String = req.query("key").to_string();
    println!("[{}]: /text/{}", Local::now(), key);
	match config.text().iter().find(|i| i.chk(key.clone())) {
		Some(item) => send::text(item.path()),
		None => Err(ErrorNotFound("File not found".to_string()))
	}
}

#[get("/file/{key}")]
async fn file(req: HttpRequest, config: web::Data<structs::Config>) -> Result<NamedFile, Error> {
	let req: &dev::Path<dev::Url>= req.match_info();
	let key: String = req.query("key").to_string();
    println!("[{}]: /file/{}", Local::now(), key);
	match config.file().iter().find(|i| i.chk(key.clone())) {
		Some(item) => send::file(item.path(), item.name()),
		None => Err(ErrorNotFound("File not found".to_string()))
	}
}

#[get("/pics/{key}/{name}")]
async fn pics(req: HttpRequest, config: web::Data<structs::Config>) -> Result<NamedFile, Error> {
	let req: &dev::Path<dev::Url>= req.match_info();
	let key: String = req.query("key").to_string();
	let name: String = req.query("name").to_string();
    println!("[{}]: /pics/{}/{}", Local::now(), key, name);
	let path = match config.pics().iter().find(|i| i.chk(key.clone())) {
		Some(item) => item.path(),
		None => "File not found".to_string()
	};
	send::pics(path, name)
}

#[get("/web/{key}")]
async fn http(req: HttpRequest, config: web::Data<structs::Config>) -> Result<impl Responder, Error> {
	let req: &dev::Path<dev::Url>= req.match_info();
	let key: String = req.query("key").to_string();
    println!("[{}]: /web/{}", Local::now(), key);
	match config.web().iter().find(|i| i.chk(key.clone())) {
		Some(item) => {
			let t = ureq::get(item.path()).await;
			send::string(t)
		}
		None => Err(ErrorNotFound("Web Error".to_string()))
	}
}

#[post("/upload")]
async fn upload(query: web::Query<structs::User>, payload: Multipart, config: web::Data<structs::Config>) -> Result<HttpResponse, Error> {
    let name: String = query.name();
	let password: String = query.password();
	println!("[{}]: /upload?user={}&password={}", Local::now(), name, password);
	match config.user().iter().find(|i| i.chk(name.clone(), password.clone())) {
		Some(_) => write::file(payload, config.upload()).await,
		None => Err(ErrorNotFound("User Error".to_string()))
	}
}

#[actix_web::main]
async fn main() -> Result<(), std_Error> {
	match read_to_string("config.json") {
		Ok(content) => {
			let config: structs::Config = serde_json::from_str(&content)?;
			HttpServer::new(move || {
				App::new()
					.app_data(web::Data::new(config.clone()))
					.wrap(Cors::permissive())
					.service(string)
					.service(text)
					.service(file)
					.service(pics)
					.service(http)
					.service(upload)
			})
			.bind("127.0.0.1:8082")?
			.run()
			.await
		},
		Err(e) => Err(e)
	}
}
