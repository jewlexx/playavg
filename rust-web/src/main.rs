use actix_web::{
    get,
    web::{self, ServiceConfig},
    Responder,
};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_secrets::SecretStore;

mod api;

#[derive(Clone)]
struct Secrets {
    client_id: String,
    client_secret: String,
    token: String,
}

#[get("/{pid}")]
async fn get_playlist(pid: web::Path<String>, secrets: web::Data<Secrets>) -> impl Responder {
    let url = format!("https://api.spotify.com/v1/playlists/{pid}");

    let resp = reqwest::Client::new()
        .get(dbg!(url))
        .bearer_auth(&secrets.token)
        .send()
        .await
        .unwrap();

    let resp: api::Playlist = resp.json().await.unwrap();

    let mut avg_sum = 0.0;
    let total = resp.tracks.items.len() as f64;

    for item in resp.tracks.items {
        avg_sum += item.track.duration_ms as f64 / total;
    }

    avg_sum.to_string()
}

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_secrets::Secrets] secrets: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let Some(client_id) = secrets.get("client_id") else { todo!() };
    let Some(client_secret) = secrets.get("client_secret") else { todo!() };
    let Some(token) = secrets.get("access_token") else { todo!() };

    let secrets = Secrets {
        client_id,
        client_secret,
        token,
    };

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(get_playlist).app_data(web::Data::new(secrets));
    };

    Ok(config.into())
}
