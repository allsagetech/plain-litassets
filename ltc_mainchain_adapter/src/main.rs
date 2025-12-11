mod litecoin_client;

use crate::litecoin_client::LitecoinClient;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};
use bitcoincore_rpc::bitcoin::Network as BtcNetwork;

// Generated CUSF protos live under these modules.
pub mod pb {
    pub mod cusf {
        pub mod common {
            pub mod v1 {
                tonic::include_proto!("cusf.common.v1");
            }
        }

        pub mod mainchain {
            pub mod v1 {
                tonic::include_proto!("cusf.mainchain.v1");
            }
        }
    }
}

use pb::cusf::mainchain::v1::{
    validator_service_server::{ValidatorService, ValidatorServiceServer},
    wallet_service_server::{WalletService, WalletServiceServer},
    GetChainInfoRequest,
    GetChainInfoResponse,
    GetChainTipRequest,
    GetChainTipResponse,
    Network,
};

type UnimplementedStream<T> = ReceiverStream<Result<T, Status>>;

struct LitecoinValidator {
    ltc: LitecoinClient,
}

#[tonic::async_trait]
impl ValidatorService for LitecoinValidator {
        async fn get_chain_info(
        &self,
        _req: Request<GetChainInfoRequest>,
    ) -> Result<Response<GetChainInfoResponse>, Status> {
        let info = self
            .ltc
            .get_chain_info()
            .map_err(|e| Status::internal(e.to_string()))?;

        let network = match info.chain {
            BtcNetwork::Bitcoin => Network::Mainnet,
            BtcNetwork::Testnet => Network::Testnet,
            BtcNetwork::Regtest => Network::Regtest,
            _ => Network::Unspecified,
        } as i32;

        Ok(Response::new(GetChainInfoResponse { network }))
    }

        async fn get_chain_tip(
        &self,
        _req: Request<GetChainTipRequest>,
    ) -> Result<Response<GetChainTipResponse>, Status> {
        let (hash, height) = self
            .ltc
            .best_block()
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(GetChainTipResponse {
            block_header_info: Some(pb::cusf::mainchain::v1::BlockHeaderInfo {
                height: height as u32,
                block_hash: Some(pb::cusf::common::v1::ReverseHex {
                    hex: Some(hash.to_string()),
                }),
                ..Default::default()
            }),
        }))
    }

    // ===== Stub the rest of ValidatorService =====

    async fn get_block_header_info(
        &self,
        _req: Request<pb::cusf::mainchain::v1::GetBlockHeaderInfoRequest>,
    ) -> Result<Response<pb::cusf::mainchain::v1::GetBlockHeaderInfoResponse>, Status> {
        Err(Status::unimplemented("get_block_header_info not implemented"))
    }

    async fn get_block_info(
        &self,
        _req: Request<pb::cusf::mainchain::v1::GetBlockInfoRequest>,
    ) -> Result<Response<pb::cusf::mainchain::v1::GetBlockInfoResponse>, Status> {
        Err(Status::unimplemented("get_block_info not implemented"))
    }

    async fn get_bmm_h_star_commitment(
        &self,
        _req: Request<pb::cusf::mainchain::v1::GetBmmHStarCommitmentRequest>,
    ) -> Result<Response<pb::cusf::mainchain::v1::GetBmmHStarCommitmentResponse>, Status> {
        Err(Status::unimplemented("get_bmm_h_star_commitment not implemented"))
    }

    async fn get_coinbase_psbt(
        &self,
        _req: Request<pb::cusf::mainchain::v1::GetCoinbasePsbtRequest>,
    ) -> Result<Response<pb::cusf::mainchain::v1::GetCoinbasePsbtResponse>, Status> {
        Err(Status::unimplemented("get_coinbase_psbt not implemented"))
    }

    async fn get_ctip(
        &self,
        _req: Request<pb::cusf::mainchain::v1::GetCtipRequest>,
    ) -> Result<Response<pb::cusf::mainchain::v1::GetCtipResponse>, Status> {
        Err(Status::unimplemented("get_ctip not implemented"))
    }

    async fn get_sidechain_proposals(
        &self,
        _req: Request<pb::cusf::mainchain::v1::GetSidechainProposalsRequest>,
    ) -> Result<Response<pb::cusf::mainchain::v1::GetSidechainProposalsResponse>, Status> {
        Err(Status::unimplemented("get_sidechain_proposals not implemented"))
    }

    async fn get_sidechains(
        &self,
        _req: Request<pb::cusf::mainchain::v1::GetSidechainsRequest>,
    ) -> Result<Response<pb::cusf::mainchain::v1::GetSidechainsResponse>, Status> {
        Err(Status::unimplemented("get_sidechains not implemented"))
    }

    async fn get_two_way_peg_data(
        &self,
        _req: Request<pb::cusf::mainchain::v1::GetTwoWayPegDataRequest>,
    ) -> Result<Response<pb::cusf::mainchain::v1::GetTwoWayPegDataResponse>, Status> {
        Err(Status::unimplemented("get_two_way_peg_data not implemented"))
    }

    type SubscribeEventsStream =
        UnimplementedStream<pb::cusf::mainchain::v1::SubscribeEventsResponse>;

    async fn subscribe_events(
        &self,
        _req: Request<pb::cusf::mainchain::v1::SubscribeEventsRequest>,
    ) -> Result<Response<Self::SubscribeEventsStream>, Status> {
        Err(Status::unimplemented("subscribe_events not implemented"))
    }

    type SubscribeHeaderSyncProgressStream =
        UnimplementedStream<pb::cusf::mainchain::v1::SubscribeHeaderSyncProgressResponse>;

    async fn subscribe_header_sync_progress(
        &self,
        _req: Request<pb::cusf::mainchain::v1::SubscribeHeaderSyncProgressRequest>,
    ) -> Result<Response<Self::SubscribeHeaderSyncProgressStream>, Status> {
        Err(Status::unimplemented(
            "subscribe_header_sync_progress not implemented",
        ))
    }

    async fn stop(
        &self,
        _req: Request<pb::cusf::mainchain::v1::StopRequest>,
    ) -> Result<Response<pb::cusf::mainchain::v1::StopResponse>, Status> {
        Err(Status::unimplemented("stop not implemented"))
    }
}

struct LitecoinWallet {
    ltc: LitecoinClient,
}

#[tonic::async_trait]
#[tonic::async_trait]
impl WalletService for LitecoinWallet {
    type CreateSidechainProposalStream =
        UnimplementedStream<pb::cusf::mainchain::v1::CreateSidechainProposalResponse>;
    type GenerateBlocksStream =
        UnimplementedStream<pb::cusf::mainchain::v1::GenerateBlocksResponse>;

    async fn create_bmm_critical_data_transaction(
        &self,
        _req: Request<pb::cusf::mainchain::v1::CreateBmmCriticalDataTransactionRequest>,
    ) -> Result<Response<pb::cusf::mainchain::v1::CreateBmmCriticalDataTransactionResponse>, Status>
    {
        Err(Status::unimplemented(
            "create_bmm_critical_data_transaction not implemented",
        ))
    }

    async fn create_deposit_transaction(
        &self,
        _req: Request<pb::cusf::mainchain::v1::CreateDepositTransactionRequest>,
    ) -> Result<Response<pb::cusf::mainchain::v1::CreateDepositTransactionResponse>, Status> {
        Err(Status::unimplemented(
            "create_deposit_transaction not implemented",
        ))
    }

    async fn create_new_address(
        &self,
        _req: Request<pb::cusf::mainchain::v1::CreateNewAddressRequest>,
    ) -> Result<Response<pb::cusf::mainchain::v1::CreateNewAddressResponse>, Status> {
        Err(Status::unimplemented("create_new_address not implemented"))
    }

    async fn create_sidechain_proposal(
        &self,
        _req: Request<pb::cusf::mainchain::v1::CreateSidechainProposalRequest>,
    ) -> Result<Response<Self::CreateSidechainProposalStream>, Status> {
        Err(Status::unimplemented(
            "create_sidechain_proposal not implemented",
        ))
    }

    async fn create_wallet(
        &self,
        _req: Request<pb::cusf::mainchain::v1::CreateWalletRequest>,
    ) -> Result<Response<pb::cusf::mainchain::v1::CreateWalletResponse>, Status> {
        Err(Status::unimplemented("create_wallet not implemented"))
    }

    async fn get_balance(
        &self,
        _req: Request<pb::cusf::mainchain::v1::GetBalanceRequest>,
    ) -> Result<Response<pb::cusf::mainchain::v1::GetBalanceResponse>, Status> {
        Err(Status::unimplemented("get_balance not implemented"))
    }

    async fn list_sidechain_deposit_transactions(
        &self,
        _req: Request<pb::cusf::mainchain::v1::ListSidechainDepositTransactionsRequest>,
    ) -> Result<Response<pb::cusf::mainchain::v1::ListSidechainDepositTransactionsResponse>, Status>
    {
        Err(Status::unimplemented(
            "list_sidechain_deposit_transactions not implemented",
        ))
    }

    async fn list_transactions(
        &self,
        _req: Request<pb::cusf::mainchain::v1::ListTransactionsRequest>,
    ) -> Result<Response<pb::cusf::mainchain::v1::ListTransactionsResponse>, Status> {
        Err(Status::unimplemented("list_transactions not implemented"))
    }

    async fn list_unspent_outputs(
        &self,
        _req: Request<pb::cusf::mainchain::v1::ListUnspentOutputsRequest>,
    ) -> Result<Response<pb::cusf::mainchain::v1::ListUnspentOutputsResponse>, Status> {
        Err(Status::unimplemented(
            "list_unspent_outputs not implemented",
        ))
    }

    async fn get_info(
        &self,
        _req: Request<pb::cusf::mainchain::v1::GetInfoRequest>,
    ) -> Result<Response<pb::cusf::mainchain::v1::GetInfoResponse>, Status> {
        Err(Status::unimplemented("get_info not implemented"))
    }

    async fn send_transaction(
        &self,
        _req: Request<pb::cusf::mainchain::v1::SendTransactionRequest>,
    ) -> Result<Response<pb::cusf::mainchain::v1::SendTransactionResponse>, Status> {
        Err(Status::unimplemented("send_transaction not implemented"))
    }

    async fn unlock_wallet(
        &self,
        _req: Request<pb::cusf::mainchain::v1::UnlockWalletRequest>,
    ) -> Result<Response<pb::cusf::mainchain::v1::UnlockWalletResponse>, Status> {
        Err(Status::unimplemented("unlock_wallet not implemented"))
    }

    async fn broadcast_withdrawal_bundle(
        &self,
        _req: Request<pb::cusf::mainchain::v1::BroadcastWithdrawalBundleRequest>,
    ) -> Result<Response<pb::cusf::mainchain::v1::BroadcastWithdrawalBundleResponse>, Status> {
        Err(Status::unimplemented(
            "broadcast_withdrawal_bundle not implemented",
        ))
    }

    async fn generate_blocks(
        &self,
        _req: Request<pb::cusf::mainchain::v1::GenerateBlocksRequest>,
    ) -> Result<Response<Self::GenerateBlocksStream>, Status> {
        Err(Status::unimplemented("generate_blocks not implemented"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ⚠️ These are just defaults; you probably want to expose them as CLI flags/env.
    let rpc_url  = "http://127.0.0.1:18034";
    let rpc_user = "drive";
    let rpc_pass = "chain";

    let ltc_for_validator = LitecoinClient::new(rpc_url, rpc_user, rpc_pass)?;
    let ltc_for_wallet    = LitecoinClient::new(rpc_url, rpc_user, rpc_pass)?;

    let validator_svc = ValidatorServiceServer::new(LitecoinValidator {
        ltc: ltc_for_validator,
    });
    let wallet_svc = WalletServiceServer::new(LitecoinWallet {
        ltc: ltc_for_wallet,
    });

    let addr = "127.0.0.1:50051".parse()?; // CUSF-style default

    println!("Litecoin mainchain server listening on {}", addr);

    Server::builder()
        .add_service(validator_svc)
        .add_service(wallet_svc)
        .serve(addr)
        .await?;

    Ok(())
}
