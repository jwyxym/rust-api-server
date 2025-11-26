use fs2::FileExt;
use futures::StreamExt;
use actix_multipart::Multipart;
use actix_web::{
	Error,
	HttpResponse,
	error::{
		ErrorNotFound
	}
};
use std::{
	fs::{
		File,
		create_dir_all
	},
	io::{
		Write
	}
};

pub async fn file(mut payload: Multipart, path: String) -> Result<HttpResponse, Error> {
	let mut create_file: bool = false;
	while let Some(item) = payload.next().await {
		if let Ok(mut field) = item {
			if let Some(content_disposition) = field.content_disposition() {
				if let Some(filename) = content_disposition.get_filename() {
					create_dir_all(&path)?;
					let filepath: String = format!("./{}/{}", path, filename);
					let mut file: File = File::create(&filepath)?;
					file.lock_exclusive()?;
					while let Some(chunk) = field.next().await {
						if let Ok(data) = chunk {
							file.write_all(&data)?;
						}
					}
					create_file = true;
					file.unlock()?;
				}
			}
		}
	}
	if create_file {
		Ok(HttpResponse::Ok().body("Ok"))
	} else {
		Err(ErrorNotFound("File Error".to_string()))
	}
}