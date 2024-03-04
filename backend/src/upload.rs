use axum::{
    body::Body,
    extract::{DefaultBodyLimit, Multipart, Path, Query},
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::get,
    routing::post,
    Router,
};
use serde::{de, Deserialize, Deserializer};
use std::{
    fmt,
    fs::File,
    io::{BufWriter, Cursor},
    str::FromStr,
};
use uuid::Uuid;

const MAX_IMAGE_SIZE: usize = std::env::var("MAX_IMAGE_SIZE").parse().unwrap_or(50) * 1024 * 1024;
const IMAGE_DIR: String = std::env::var("IMAGE_DIR").expect("IMAGE_DIR is not set");

#[derive(Deserialize)]
struct ResizeOptions {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    width: Option<u32>,
    height: Option<u32>,
    scale: Option<f32>,
}

pub fn build_router() -> Router {
    Router::new()
        .route("/image/:image", get(image))
        .route("/upload", post(upload))
        .layer(DefaultBodyLimit::max(MAX_IMAGE_SIZE))
}

async fn upload(mut multipart: Multipart) -> impl IntoResponse {
    let mut tasks = Vec::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let data = field.bytes().await.unwrap();

        let task = tokio::spawn(async move {
            let name = Uuid::new_v4().to_string();
            let img = image::load_from_memory(&data).unwrap();

            let path = format!("{IMAGE_DIR}/{name}.png");
            let file = File::create(&path).unwrap();
            let mut writer = BufWriter::new(file);
            img.write_to(&mut writer, image::ImageOutputFormat::Png)
                .unwrap();
        });

        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }

    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::empty())
        .unwrap()
}

async fn image(Path(image): Path<String>, options: Query<ResizeOptions>) -> impl IntoResponse {
    let image = image::open(format!("{IMAGE_DIR}/{image}")).unwrap();

    let image = match (options.width, options.height, options.scale) {
        (Some(width), Some(height), _) => {
            image::imageops::resize(&image, width, height, image::imageops::FilterType::Nearest)
        }
        (None, None, Some(scale)) => {
            let scale = scale / 100.0;
            image::imageops::resize(
                &image,
                (scale * image.width() as f32) as u32,
                (scale * image.height() as f32) as u32,
                image::imageops::FilterType::Nearest,
            )
        }
        _ => image.to_rgba8(),
    };

    let mut buffer = Cursor::new(Vec::new());
    image
        .write_to(&mut buffer, image::ImageOutputFormat::Png)
        .unwrap();

    let body = Body::from(buffer.into_inner());

    Response::builder()
        .header("Content-Type", "image/png")
        .body(body)
        .unwrap()
}

// Serde deserialization decorator to map empty Strings to None
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}
