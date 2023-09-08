mod observation;

use async_std::task::block_on;
use rand::seq::IteratorRandom;
use reqwest::Client;

use crate::observation::Observation;

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let params = [("maxResults", "100"), ("sppLocale", "es")];

    let observations = client
        .get("https://api.ebird.org/v2/data/obs/AR/recent/notable")
        .header("x-ebirdapitoken", "n1olhv1hlb8u")
        .query(&params)
        .send()
        .await?
        .json::<Vec<Observation>>()
        .await?;

    // Debido a que la API no permite obtener datos aleatorios, se mezclan manualmente.
    let mut rng = rand::thread_rng();
    let chosen = observations.into_iter().choose_multiple(&mut rng, 10);

    chosen.iter().enumerate().for_each(|(i, obs)| {
        println!("Observación {}", i + 1);
        println!("\tCantidad: {}", obs.how_many);
        println!("\tNombre Comun: {}", obs.com_name);
        println!("\tNombre Cientifico: {}", obs.sci_name);
        println!("\tUbicación: {}", obs.loc_name);
    });

    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        eprintln!("Error: {:?}", err);
    }
}
