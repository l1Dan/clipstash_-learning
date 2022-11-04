use super::{model, DataError, DatabasePool};

type Result<T> = std::result::Result<T, DataError>;

pub async fn get_clip<M: Into<model::GetClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::Clip> {
    let model = model.into();
    let short_code = model.short_code.as_str();
    Ok(sqlx::query_as!(
        model::Clip,
        "SELECT * FROM clips WHERE short_code = ?",
        short_code,
    )
    .fetch_one(pool)
    .await?)
}

pub async fn new_clip<M: Into<model::NewClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::Clip> {
    let model = model.into();
    let _ = sqlx::query!(
        r#"INSERT INTO clips (
            clip_id,
            short_code,
            content,
            title,
            posted,
            expires,
            password,
            hits)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
        model.clip_id,
        model.short_code,
        model.content,
        model.title,
        model.posted,
        model.expires,
        model.password,
        0,
    )
    .execute(pool)
    .await?;
    get_clip(model.short_code, pool).await
}

pub async fn update_clip<M: Into<model::UpdateClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::Clip> {
    let model = model.into();
    let _ = sqlx::query!(
        r#"UPDATE clips SET
        content = ?,
        expires = ?,
        password = ?,
        title = ?
        WHERE short_code = ?"#,
        model.content,
        model.expires,
        model.password,
        model.title,
        model.short_code
    )
    .execute(pool)
    .await?;
    get_clip(model.short_code, pool).await
}
