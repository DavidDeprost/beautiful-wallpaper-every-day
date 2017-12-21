extern crate reqwest;
#[macro_use]
extern crate hyper;
use hyper::header::{Headers, Authorization};

extern crate serde;
extern crate serde_json;

extern crate rand;
use rand::Rng;

#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::Write;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Urls {
    raw: String,
    full: String,
    regular: String,
    small: String,
    thumb: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Category {
    id: String,
    title: String,
    photo_count: u32,
    links: Links,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Exif {
    make: Option<String>,
    model: Option<String>,
    exposure_time: Option<String>,
    aperture: Option<String>,
    focal_length: Option<String>,
    iso: Option<u16>,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Links {
    #[serde(rename = "self")]
    _self: String,
    html: String,
    photos: Option<String>,
    likes: Option<String>,
    portfolio: Option<String>,
    download: Option<String>,
    download_location: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct ProfileImage {
    small: String,
    medium: String,
    large: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct User {
    id: String,
    username: String,
    name: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    updated_at: Option<String>,
    twitter_username: Option<String>,
    portfolio_url: Option<String>,
    bio: Option<String>,
    location: Option<String>,
    total_likes: u32,
    total_photos: u32,
    total_collections: u32,
    profile_image: ProfileImage,
    links: Links,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Position {
    latitude: f64,
    longitude: f64,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Location {
    city: String,
    country: String,
    position: Position,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Collection {
    id: u32,
    title: String,
    published_at: String,
    updated_at: String,
    curated: bool,
    cover_photo: CoverPhoto,
    user: User,
    links: Links,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct CoverPhoto {
    id: String,
    width: u16,
    height: u16,
    color: String,
    likes: u16,
    liked_by_user: bool,
    description: Option<String>,
    user: User,
    urls: Urls,
    categories: Vec<Category>,
    links: Links,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct UnsplashFoto {
    id: String,
    created_at: String,
    updated_at: String,
    width: u16,
    height: u16,
    color: String,
    downloads: u16,
    likes: u16,
    liked_by_user: bool,
    description: Option<String>,
    exif: Exif,
    location: Option<Location>,
    current_user_collections: Vec<Collection>,
    urls: Urls,
    categories: Vec<Category>,
    links: Links,
    user: User,
    slug: Option<String>,
}

header! { (AcceptVersion, "Accept-Version") => [String] }

fn main() {
    let mut headers = Headers::new();
    let client = reqwest::Client::new();

    headers.set(Authorization(
        "Client-ID ee88235a89c58088c3ebf8025e90214c4574909913e0b7442165f4f87452384e"
            .to_owned(),
    ));
    headers.set(AcceptVersion("v1".to_owned()));

    let resp = client
        .get("https://api.unsplash.com/photos/random")
        .headers(headers)
        .send();

    let json: UnsplashFoto = match resp {
        Ok(mut response) => {
            match response.json() {
                Ok(result) => result,
                Err(e) => {
                    // let's keep the jsons responses that error
                    // to build tests later
                    let name_rnd: String = rand::thread_rng().gen_ascii_chars().take(10).collect();
                    let full_name = &["jsons", &name_rnd[..]].join("/");
                    let path = Path::new(full_name);
                    let mut buf: Vec<u8> = vec![];
                    response.copy_to(&mut buf).unwrap();

                    let mut file = match File::create(&path) {
                        Err(why) => {
                            panic!(
                                "could not create {:?} {:?}",
                                path.display(),
                                why.description()
                            )
                        }
                        Ok(file) => file,
                    };

                    match file.write_all(buf.as_slice()) {
                        Err(why) => {
                            panic!(
                                "could not write to {:?}: {:?}",
                                path.display(),
                                why.description()
                            )
                        }
                        Ok(_) => println!("wrote faulty json to {:?}", path.display()),
                    };

                    panic!(
                        "JSON PARSE ERROR: {:?} \n {:?}",
                        e,
                        response.text().unwrap()
                    )
                }
            }
        }
        Err(e) => panic!("NETWORK ERROR: {:?}", e),
    };

    println!("{:?}", json.urls.full);

    //let test: UnsplashFoto = serde_json::from_str(data).unwrap();

    //assert!(resp.status().is_success());

    //let mut content = String::new();
    //resp.read_to_string(&mut content);
}