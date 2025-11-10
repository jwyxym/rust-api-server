use actix_files::NamedFile;
use actix_web::{
	Error,
	Responder,
	HttpResponse,
	http::header::ContentDisposition,
	error:: {
		ErrorBadRequest,
		ErrorNotFound
	}
};
use std::{
	path::{
		Path,
		PathBuf
	},
	fs::{
		read_to_string
	},
	ffi::OsStr
};

pub fn file(path: String, name: String) -> Result<NamedFile, Error> {
	let mut file: NamedFile = NamedFile::open(path)?;
	
	file = file.set_content_disposition(
		ContentDisposition::attachment(name)
	);
	
	Ok(file)
}

pub fn pics(path: String, name: String) -> Result<NamedFile, Error> {
	let path: PathBuf = Path::new(&path).join(name);
	
	let ext: String = path.extension()
		.and_then(|ext: &OsStr| ext.to_str())
		.map(|ext: &str| ext.to_lowercase())
		.unwrap_or_default();

	if ext != "jpg" && ext != "jpeg" && ext != "png" {
		return Err(ErrorBadRequest("Only .jpg, .jpeg, or .png files are allowed"));
	}
	if !path.exists() {
		return Err(ErrorNotFound("File not found"));
	}
	
	Ok(NamedFile::open(path)?)
}

pub fn text(path: String) -> Result<impl Responder, Error> {
	match read_to_string(path) {
		Ok(content) => {
			Ok(HttpResponse::Ok().body(content))
		},
		Err(e) => {
			Err(ErrorBadRequest(e.to_string()))
		}
	}
}

pub fn string(str: String) -> Result<impl Responder, Error> {
	Ok(HttpResponse::Ok().body(str))
}