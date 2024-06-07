// Importation des modules nécessaires
use reqwest::Error; // Pour gérer les erreurs de requêtes HTTP
use serde::Deserialize; // Pour désérialiser les réponses JSON

// Définition de la structure `Kline` pour représenter les données des bougies
#[derive(Deserialize, Debug)]
pub struct Kline {
    #[serde(rename = "t")]
    pub open_time: u64, // Temps d'ouverture
    #[serde(rename = "o")]
    pub open: String, // Prix d'ouverture
    #[serde(rename = "h")]
    pub high: String, // Prix le plus élevé
    #[serde(rename = "l")]
    pub low: String, // Prix le plus bas
    #[serde(rename = "c")]
    pub close: String, // Prix de clôture
    #[serde(rename = "v")]
    pub volume: String, // Volume
    #[serde(rename = "T")]
    pub close_time: u64, // Temps de clôture
    #[serde(rename = "q")]
    pub quote_asset_volume: String, // Volume en devise cotée
    #[serde(rename = "n")]
    pub number_of_trades: u64, // Nombre de transactions
    #[serde(rename = "V")]
    pub taker_buy_base_asset_volume: String, // Volume d'achat des acheteurs
    #[serde(rename = "Q")]
    pub taker_buy_quote_asset_volume: String, // Volume de devise cotée des acheteurs
    #[serde(rename = "B")]
    pub ignore: String, // Champ ignoré
}

// Fonction asynchrone pour récupérer les données de bougies depuis l'API Binance
pub async fn fetch_klines(symbol: &str, interval: &str, limit: u16) -> Result<Vec<Kline>, Error> {
    // URL de l'API avec les paramètres fournis
    let url = format!(
        "https://api.binance.com/api/v3/klines?symbol={}&interval={}&limit={}",
        symbol, interval, limit
    );
    
    // Envoi de la requête GET asynchrone à l'URL
    let response = reqwest::get(&url).await?;
    
    // Désérialisation de la réponse JSON en un vecteur de vecteurs de valeurs JSON
    let klines: Vec<Vec<serde_json::Value>> = response.json().await?;
    
    // Transformation des vecteurs JSON en instances de `Kline`
    let klines: Vec<Kline> = klines
        .into_iter()
        .map(|k| Kline {
            open_time: k[0].as_u64().unwrap(),
            open: k[1].as_str().unwrap().to_string(),
            high: k[2].as_str().unwrap().to_string(),
            low: k[3].as_str().unwrap().to_string(),
            close: k[4].as_str().unwrap().to_string(),
            volume: k[5].as_str().unwrap().to_string(),
            close_time: k[6].as_u64().unwrap(),
            quote_asset_volume: k[7].as_str().unwrap().to_string(),
            number_of_trades: k[8].as_u64().unwrap(),
            taker_buy_base_asset_volume: k[9].as_str().unwrap().to_string(),
            taker_buy_quote_asset_volume: k[10].as_str().unwrap().to_string(),
            ignore: k[11].as_str().unwrap().to_string(),
        })
        .collect();
    
    // Retourne les données de bougies ou une erreur
    Ok(klines)
}


// Fonction pour calculer les moyennes mobiles
pub fn calculate_moving_average(data: &[f64], window: usize) -> Vec<f64> {
    // Utilisation de la méthode `windows` pour créer des tranches de données de taille `window`
    data.windows(window)
        .map(|w| w.iter().sum::<f64>() / w.len() as f64) // Calcul de la moyenne de chaque tranche
        .collect() // Conversion en vecteur
}
