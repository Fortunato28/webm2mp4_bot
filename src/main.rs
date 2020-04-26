use webm2mp4_bot as w2m;

#[tokio::main]
async fn main() {
    let link = "https://img-9gag-fun.9cache.com/photo/a1WpNrP_460svvp9.webm";
    let temp_dir = "tmp/";
    let video = w2m::Video::new(link).await;
    video.save_to_fs(temp_dir);

    w2m::to_mp4(&temp_dir, &video.filename, "result.mp4");
}
