// Importation du module `data_analysis`
mod data_analysis;

use data_analysis::{fetch_klines, calculate_moving_average}; // Importation des fonctions
use tokio; // Pour l'exécution asynchrone

#[tokio::main] // Annotation pour utiliser `tokio`
async fn main() {
    let symbol = "BTCUSDT"; // Symbole de la paire de trading
    let interval = "1m"; // Intervalle de temps pour chaque bougie
    let limit = 1000; // Nombre de bougies à récupérer

    // Récupération des données de bougies
    match fetch_klines(symbol, interval, limit).await {
        Ok(klines) => {
            // Extraction des prix de clôture et conversion en `f64`
            let closes: Vec<f64> = klines
                .iter()
                .map(|k| k.close.parse::<f64>().unwrap())
                .collect();

            // Calcul des moyennes mobiles pour différentes fenêtres
            let ma200 = calculate_moving_average(&closes, 200);
            let ma99 = calculate_moving_average(&closes, 99);
            let ma25 = calculate_moving_average(&closes, 25);
            let ma7 = calculate_moving_average(&closes, 7);

            // Affichage des résultats
            println!("Moyenne Mobile 200: {:?}", ma200);
            println!("Moyenne Mobile 99: {:?}", ma99);
            println!("Moyenne Mobile 25: {:?}", ma25);
            println!("Moyenne Mobile 7: {:?}", ma7);
        }
        Err(e) => eprintln!("Erreur lors de la récupération des données: {}", e), // Gestion des erreurs
    }
}
