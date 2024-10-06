use leptos::*;

use crate::models::colors_model::{ColorForCreate, ColorForSelect};

#[server(InsertColor)]
pub async fn insert_color(color_name: String, color_type: String) -> Result<(), ServerFnError> {
    use super::ssr::db;
    use leptos_axum::ResponseOptions;
    use uuid::Uuid;
    use http::{header::SET_COOKIE, HeaderValue, StatusCode};
    
    let mut conn = db().await?;

    let uuid = Uuid::now_v7();
    let b32u = data_encoding::BASE64_NOPAD.encode(uuid.as_bytes());

    match sqlx::query("INSERT INTO color_table (color_id, color_name, color_type) VALUES (?,?,?)")
        .bind(b32u.clone())
        .bind(color_name)
        .bind(color_type)
        .execute(&mut conn)
        .await
    {
        Ok(_) => {
            let resp = expect_context::<ResponseOptions>();
            resp.set_status(StatusCode::CREATED);
            resp.insert_header(SET_COOKIE, HeaderValue::from_str(format!("color={}", b32u.clone()).to_string().as_ref()).unwrap());
            Ok(())
        },
        Err(e) => Err(ServerFnError::ServerError(e.to_string()))
    }
}

#[server(GetColors)]
pub async fn get_colors() -> Result<Vec<ColorForSelect>, ServerFnError> {
    use super::ssr::db;
    use leptos_axum::ResponseOptions;
    use uuid::Uuid;
    use http::{header::SET_COOKIE, HeaderValue, StatusCode};

    // use futures::TryStreamExt;

    let mut conn = db().await?;
    let uuid = Uuid::now_v7();
    let b32u = data_encoding::BASE64_NOPAD.encode(uuid.as_bytes());

    let query = r#"SELECT
        id,
        color_id,
        color_name,
        color_type,
        DATE_FORMAT(ctime, '%M %d %Y') as ctime, 
        DATE_FORMAT(mtime, '%M %d %Y') as mtime
        FROM color_table"#;
    let colors: Vec<ColorForSelect> = sqlx::query_as(query).fetch_all(&mut conn).await?;

    let resp = expect_context::<ResponseOptions>();
    resp.set_status(StatusCode::IM_A_TEAPOT);
    resp.insert_header(SET_COOKIE, HeaderValue::from_str(format!("color={}", b32u.clone()).to_string().as_ref()).unwrap());


    Ok(colors)
}
