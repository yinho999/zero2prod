use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// Adding a new subscription
#[tracing::instrument(name = "Adding a new subscription", skip(form, pool), fields(request_id=%Uuid::new_v4(), subscriber_email = %form.email, subscriber_name = %form.name))]

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    // // Lets Generate a random unique ID for this subscription
    // let request_id = Uuid::new_v4();

    // // Spans, like logs, have an associated level
    // // `info_span` creates a span at the info-level
    // // We are using the same interpolation syntax of "println "/"print" here
    // let request_span = tracing::info_span!(
    //     "Adding a new subscriber",
    //     %request_id,
    //     subscriber_email = %form.email,
    //     subscriber_name = %form.name
    // );
    // let _request_span_guard = request_span.enter();

    // We do not call `.enter` on query_span!
    // `.instrument` takes care of it at the right moments
    // in the query future lifetime
    match insert_subscriber(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!(
                "Error saving new subscribe details in the database: {:?}",
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Save the new subscriber in the database
#[tracing::instrument(
    name = "Saving new subscribe details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name) VALUES ($1, $2, $3)"#,
        Uuid::new_v4(),
        form.email,
        form.name,
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!(
            "Error saving new subscribe details in the database: {:?}",
            e
        );
        e
    })?;
    Ok(())
}
