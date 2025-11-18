pub async fn get(path: String) -> String {
	match ureq::get(&path).call() {
		Ok(res) => {
			match res.into_body().read_to_string() {
				Ok(body) => {
					body
				}
				Err(e) => e.to_string()
			}
		}
		Err(e) => e.to_string()
	}
}
