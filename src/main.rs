use webm2mp4_bot as w2m;

#[tokio::main]
async fn main() {
    let link = "https://img-9gag-fun.9cache.com/photo/a1WpNrP_460svvp9.webm";
    w2m::download_resource(link).await;
    //w2m::run().await;
}
