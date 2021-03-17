#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::{
	path::Path,
	path::PathBuf,
};

use rocket::{
	response::NamedFile,
};

#[get("/")]
fn index() -> Option<NamedFile> {
	NamedFile::open("target/debug/frontend/index.html").ok()
}

#[get("/<_path..>", rank = 2)]
fn app(_path: PathBuf) -> Option<NamedFile> {
	NamedFile::open("target/debug/frontend/index.html").ok()
}

#[get("/js/<path..>")]
fn js(path: PathBuf) -> Option<NamedFile> {
	NamedFile::open(Path::new("target/debug/frontend/js/").join(path)).ok()
}

#[get("/css/<path..>")]
fn css(path: PathBuf) -> Option<NamedFile> {
	NamedFile::open(Path::new("target/debug/frontend/css/").join(path)).ok()
}

#[get("/img/<path..>")]
fn img(path: PathBuf) -> Option<NamedFile> {
	NamedFile::open(Path::new("target/debug/frontend/img/").join(path)).ok()
}

#[get("/favicon.ico")]
fn favicon() -> Option<NamedFile> {
	NamedFile::open(Path::new("target/debug/frontend/favicon.ico")).ok()
}

fn main() {
	let routes = routes![
		index, app, js, css, img, favicon,
	];
	rocket::ignite().mount("/", routes).launch();
}