use api_rs::preprocessor::{
    preprocessor_server::{Preprocessor as ApiPreprocessor, PreprocessorServer as ApiServer}, PreprocessorRequest as ApiRequest,
    PreprocessorTextResponse as ApiResponse,
};
use log::{error, info, warn};
use preprocessor::{Preprocessor, ProcessingOptions};
use tonic::{transport::Server, Request, Response, Status};

const NO_PROCESSING_OPTIONS_MESSAGE: &'static str =
    "The preprocessing request contained no preprocessing options.";

#[derive(Default)]
pub struct PreprocessorService {}

#[tonic::async_trait]
impl ApiPreprocessor for PreprocessorService {
    async fn preprocess_text(
        &self,
        request: Request<ApiRequest>,
    ) -> Result<Response<ApiResponse>, Status> {
        info!("Recieved preprocssing request");

        let ApiRequest {
            raw_text,
            processing_options,
        } = request.into_inner();
        let processing_options: ProcessingOptions = processing_options
            .ok_or({
                error!("{}", NO_PROCESSING_OPTIONS_MESSAGE);
                Status::invalid_argument(NO_PROCESSING_OPTIONS_MESSAGE)
            })
            .and_then(|opt| {
                opt.try_into().map_err(|e: String| {
                    error!("{e:?}");
                    Status::invalid_argument(e.to_string())
                })
            })?;

        info!("Starting Preprocessing");
        let preprocessor = Preprocessor::new(processing_options);
        let preprocessed_text = preprocessor.process(raw_text);
        info!("Finished Preprocessing");

        info!("Sending Response");
        let reply = ApiResponse { preprocessed_text };
        Ok(Response::new(reply))
    }
}

/// Basic Scheduler Service for the system periodically calls a specific system
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    info!("Finished initalizing logging");

    info!("Creating the Preprocessor Service");
    let port = std::env::var("PORT").unwrap_or({
        warn!("Failed to read PORT, using 50051");
        "50051".to_string()
    });
    let address = format!("[::1]:{port}").parse().map_err(|e: std::net::AddrParseError| {
        error!("Failed to parse address: {:?}", e.to_string());
        e
    })?;
    let preprocessor_service = PreprocessorService::default();

    info!("Preprocessor Service listening on {:?}", address);
    Server::builder()
        .add_service(ApiServer::new(preprocessor_service))
        .serve(address)
        .await?;

    Ok(())
}
