use tonic::{transport::Server, Request, Response, Status};

use payments::bitcoin_server::{Bitcoin,BitcoinServer};
use payments::{BtcPaymentResponse, BtcPaymentRequest};

//mod is defined and payments is included to include proto macro through tonic based on payments.proto file. This way above mentioned types are brought to scope.
pub mod payments {
    tonic::include_proto!("payments");
}

//create new struct for bitcoin service
#[derive(Debug, Default)]
pub struct BitcoinService {}

//implement bitcoin trait that we defined above for bitcoin service
#[tonic::async_trait]
impl Bitcoin for BitcoinService{
    async fn send_payment(
        &self,
        request: Request<BtcPaymentRequest>,
    ) -> Result<Response<BtcPaymentResponse>,Status> {
        println!("Got a response: {:?}", request);

        let req = request.into_inner();

        let reply = BtcPaymentResponse {
            successful: true,
            message: format!("Sent {}BTC to {}.", req.amount, req.to_addr).into(),
        };
        Ok(Response::new(reply))
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let btc_service = BitcoinService::default();

    Server::builder()
        .add_service(BitcoinServer::new(btc_service))
        .serve(addr)
        .await?;
    Ok(())
}
