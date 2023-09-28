use actix_web::{web, HttpResponse};

use yahoo_finance_api as yahoo;

pub async fn get_ticker_price(ticker: &String) -> (f64, String) {
    let provider = yahoo::YahooConnector::new();

    // get the latest quotes in 1 minute intervals
    let response = provider.get_latest_quotes(&ticker, "1d").await;

    if response.is_err() {
        let error = response.unwrap_err().to_string();

        return (0.0, error);
    } else {
        // extract just the latest valid quote summery
        let quote = response.unwrap().last_quote().unwrap();

        return (quote.close, String::from(""));
    }
}

pub async fn get_price(ticker: web::Path<String>) -> HttpResponse {
    log::info!("+++ Divide By Cucumber Error. Please Reinstall Universe And Reboot +++");

    let (quote, error) = get_ticker_price(&ticker).await;

    if error != "" {
        HttpResponse::InternalServerError().json(error)
    } else {
        HttpResponse::Ok().json(quote)
    }
}
