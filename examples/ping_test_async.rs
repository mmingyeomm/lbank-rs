use std::sync::Arc;

use lbank_rs::common::{self, AsyncCommon, Common};
use lbank_rs::api::{AsyncLBank, LBank};
use tungstenite::http::response;

#[tokio::main]
async fn main() {
    
    let common : AsyncCommon = AsyncLBank::new(None, None); 

    let arcgen = Arc::new(common);  

    let mut handles = Vec::new(); 


    for i in 1..30 {

        let x = arcgen.clone(); 
        let handle = tokio::spawn(async move {

            match x.ping().await  {

                Ok(response) => {
                    println!("✅ Thread {} Success: {}", i, response);
                }
                Err(e) => {
                    println!("❌ Thread {} Error: {:?}", i, e);
                }
        
            }

        });

        handles.push(handle);

    }
    
    let results = futures::future::join_all(handles).await;


}
