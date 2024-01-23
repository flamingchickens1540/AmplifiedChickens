use crate::model;
use axum::{
    body::Body,
    extract::{Multipart, Path, Query, State},
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde::{de, Deserialize, Deserializer};
use std::{
    fmt,
    fs::File,
    io::{BufWriter, Cursor},
    str::FromStr,
};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ResizeOptions {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    width: Option<u32>,
    height: Option<u32>,
    scale: Option<f32>,
}

pub async fn upload(
    State(state): State<model::AppState>,
    Query(keys): Query<model::ScoutEventTeam>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut tasks = Vec::new();
    let image_dir: String = std::env::var("IMAGE_DIR").expect("IMAGE_DIR is not set");

    while let Some(field) = multipart.next_field().await.unwrap() {
        let data = field.bytes().await.unwrap();

        let new_keys = keys.clone();
        let new_db = state.db.pool.clone();

        let task = tokio::spawn(async move {
            let name = Uuid::new_v4().to_string();
            let img = image::load_from_memory(&data).unwrap();

            let path = format!("https://localhost:3007/image/{name}.png");
            let file = File::create(&path).unwrap();
            let mut writer = BufWriter::new(file);
            img.write_to(&mut writer, image::ImageOutputFormat::Png)
                .unwrap();
            let _ = sqlx::query("INSERT INTO images (name, event_key, team_key, url, scout_id) VALUES ($1, $2, $3, $4, $5) IF EXISTS THEN DO NOTHING")
                .bind(name)
                .bind(new_keys.event_key)
                .bind(new_keys.team_key)
                .bind(path) // FIXME: Make this the correct url
                .bind(new_keys.scout_id)
                .execute(&new_db)
                .await;
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

pub async fn image(
    State(state): State<model::AppState>,
    Path(image): Path<String>,
    options: Query<ResizeOptions>,
) -> impl IntoResponse {
    let image_dir: String = std::env::var("IMAGE_DIR").expect("IMAGE_DIR is not set");

    let image = image::open(format!("{image_dir}/{image}")).unwrap();

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
    match Option::<String>::deserialize(de)?.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}
