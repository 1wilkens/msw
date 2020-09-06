mod types;

use reqwest::blocking;

/* urls:
 *
 * equity: https://api.boerse-frankfurt.de/data/equity_master_data?isin={}",
 * derivatives: https://api.boerse-frankfurt.de/data/derivatives_master_data?isin={}",
 */

fn main() -> reqwest::Result<()> {
    let client = blocking::Client::new();
    let wkn = "TT3EEH";

    let url = format!(
        "https://api.boerse-frankfurt.de/global_search/count/{}",
        wkn
    );

    let isin = "DE000TT3EEH8";
    let url = format!(
        "https://api.boerse-frankfurt.de/data/derivatives_master_data?isin={}",
        isin
    );

    let data: types::DerivativeMasterData = blocking::get(&url)?.json()?;
    println!("WKN: {} ISIN: {}", data.wkn, data.isin);

    return Ok(());
}
