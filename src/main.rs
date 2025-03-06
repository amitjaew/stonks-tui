mod utils;
//use utils::api;
//use utils::scheme::CriptoUsdHistory;
use utils::ui;

#[tokio::main]
async fn main() {
    
    //let chilean_indicators = api::get_chilean_indicators().await.unwrap();
    //let cripto_indicators = api::get_crypto_indicators().await.unwrap();
    //let clp_compare = api::get_clp_usd_comp().await.unwrap();
    //let btc_compare = api::get_btc_usd_comp().await.unwrap();
    //let xmr_compare: CriptoUsdHistory = api::get_xmr_usd_comp().await.unwrap();
    //println!("{:?}", clp_compare.get_points());
    
    println!("Sire, please wait until stonks are loaded");
    let _ = ui::gui_startup().await;
}
