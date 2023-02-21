use clap::Parser;
use image_rs::image::ImageClient;
use std::path::Path;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    auth: bool,

    #[arg(short, long)]
    signature: bool,

    #[arg(long, default_value_t = String::from("cc_kbc::http://127.0.0.1:8080"))]
    aa_kbc_params: String,

    #[arg(required = true, long)]
    image_url: String,

    #[arg(long, default_value_t = String::from("./"))]
    target_dir: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let target_dir_path = Path::new(&cli.target_dir);
    std::env::set_var("CC_IMAGE_WORK_DIR", &target_dir_path);

    let mut image_client = ImageClient::default();

    if cli.auth {
        image_client.config.auth = true;
    }

    if cli.signature {
        image_client.config.security_validate = true;
    }

    let aa_parameter = format!("provider:attestation-agent:{}", &cli.aa_kbc_params);

    image_client.pull_image(
        &cli.image_url,
        &target_dir_path,
        &None,
        &Some(&aa_parameter),
    )
    .await.unwrap();
}
