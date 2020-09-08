mod types;

use reqwest::blocking;

/* urls:
 *
 * search: https://api.boerse-frankfurt.de/global_search/limitedsearch/de?searchTerms={}
 *
 * equity: https://api.boerse-frankfurt.de/data/equity_master_data?isin={}
 * derivatives: https://api.boerse-frankfurt.de/data/derivatives_master_data?isin={}
 */

fn main() -> reqwest::Result<()> {
    for arg in std::env::args().skip(1) {
        if !(arg.len() == 6 || arg.len() == 12) {
            // no wkn or isin
            println!("Skipping invalid wkn/isin: {}", arg);
            continue;
        }

        // lookup isin and type based on wkn/isin
        let url = format!(
            "https://api.boerse-frankfurt.de/global_search/limitedsearch/de?searchTerms={}",
            arg
        );
        let results: Vec<Vec<types::SearchResult>> = blocking::get(&url)?.json()?;
        if results.len() == 0 {
            // boerse frankfurt does not know this wkn/isin
            println!("Skipping invalid wkn/isin: {}", arg);
            continue;
        }
        let result = &results[0][0];

        if result.r#type == "DERIVATIVE" {
            // isin is derivative
            let url = format!(
                "https://api.boerse-frankfurt.de/data/derivatives_master_data?isin={}",
                result.isin
            );
            let data: types::DerivativeMasterData = blocking::get(&url)?.json()?;
            print_derivative(data);
        } else if result.r#type == "EQUITY" {
            // isin is equity
            let url = format!(
                "https://api.boerse-frankfurt.de/data/equity_master_data?isin={}",
                result.isin
            );
            let data: types::EquityMasterData = blocking::get(&url)?.json()?;
            print_equity(data);
        }
    }

    return Ok(());
}

fn print_core() {
    //
}

fn print_derivative(data: types::DerivativeMasterData) {
    print_core();
    dbg!(data);
}

fn print_equity(data: types::EquityMasterData) {
    print_core();
    dbg!(data);
}
