use std::env;
use std::thread;

use pier_client_polkadot::proto::plugin::*;
use pier_client_polkadot::proto::plugin_grpc::*;
use pier_client_polkadot::proto::ibtp::IBTP;

use grpc::{ServerHandlerContext, ServerResponseSink};
use grpc::ServerRequestSingle;
use grpc::ServerResponseUnarySink;
use tls_api::TlsAcceptorBuilder;

struct AppchainPluginImpl;

impl AppchainPlugin for AppchainPluginImpl {
    fn initialize(&self, o: ServerHandlerContext, req: ServerRequestSingle<InitializeRequest>, resp: ServerResponseUnarySink<Empty>) -> grpc::Result<()> {
        todo!()
    }

    fn start(&self, o: ServerHandlerContext, req: ServerRequestSingle<Empty>, resp: ServerResponseUnarySink<Empty>) -> grpc::Result<()> {
        todo!()
    }

    fn stop(&self, o: ServerHandlerContext, req: ServerRequestSingle<Empty>, resp: ServerResponseUnarySink<Empty>) -> grpc::Result<()> {
        todo!()
    }

    fn get_ibtp(&self, o: ServerHandlerContext, req: ServerRequestSingle<Empty>, resp: ServerResponseSink<IBTP>) -> grpc::Result<()> {
        todo!()
    }

    fn submit_ibtp(&self, o: ServerHandlerContext, req: ServerRequestSingle<IBTP>, resp: ServerResponseUnarySink<SubmitIBTPResponse>) -> grpc::Result<()> {
        todo!()
    }

    fn rollback_ibtp(&self, o: ServerHandlerContext, req: ServerRequestSingle<RollbackIBTPRequest>, resp: ServerResponseUnarySink<RollbackIBTPResponse>) -> grpc::Result<()> {
        todo!()
    }

    fn increase_in_meta(&self, o: ServerHandlerContext, req: ServerRequestSingle<IBTP>, resp: ServerResponseUnarySink<IBTP>) -> grpc::Result<()> {
        todo!()
    }

    fn get_out_message(&self, o: ServerHandlerContext, req: ServerRequestSingle<GetOutMessageRequest>, resp: ServerResponseUnarySink<IBTP>) -> grpc::Result<()> {
        todo!()
    }

    fn get_in_message(&self, o: ServerHandlerContext, req: ServerRequestSingle<GetInMessageRequest>, resp: ServerResponseUnarySink<GetInMessageResponse>) -> grpc::Result<()> {
        todo!()
    }

    fn get_in_meta(&self, o: ServerHandlerContext, req: ServerRequestSingle<Empty>, resp: ServerResponseUnarySink<GetMetaResponse>) -> grpc::Result<()> {
        todo!()
    }

    fn get_out_meta(&self, o: ServerHandlerContext, req: ServerRequestSingle<Empty>, resp: ServerResponseUnarySink<GetMetaResponse>) -> grpc::Result<()> {
        todo!()
    }

    fn get_callback_meta(&self, o: ServerHandlerContext, req: ServerRequestSingle<Empty>, resp: ServerResponseUnarySink<GetMetaResponse>) -> grpc::Result<()> {
        todo!()
    }

    fn commit_callback(&self, o: ServerHandlerContext, req: ServerRequestSingle<IBTP>, resp: ServerResponseUnarySink<Empty>) -> grpc::Result<()> {
        todo!()
    }

    fn get_receipt(&self, o: ServerHandlerContext, req: ServerRequestSingle<IBTP>, resp: ServerResponseUnarySink<IBTP>) -> grpc::Result<()> {
        todo!()
    }

    fn get_lock_event(&self, o: ServerHandlerContext, req: ServerRequestSingle<Empty>, resp: ServerResponseSink<LockEvent>) -> grpc::Result<()> {
        todo!()
    }

    fn get_update_meta(&self, o: ServerHandlerContext, req: ServerRequestSingle<Empty>, resp: ServerResponseSink<UpdateMeta>) -> grpc::Result<()> {
        todo!()
    }

    fn un_escrow(&self, o: ServerHandlerContext, req: ServerRequestSingle<UnLock>, resp: ServerResponseUnarySink<Empty>) -> grpc::Result<()> {
        todo!()
    }

    fn name(&self, o: ServerHandlerContext, req: ServerRequestSingle<Empty>, resp: ServerResponseUnarySink<NameResponse>) -> grpc::Result<()> {
        todo!()
    }

    fn types(
        &self,
        _: ServerHandlerContext,
        req: ServerRequestSingle<Empty>,
        resp: ServerResponseUnarySink<TypeResponse>,
    ) -> grpc::Result<()> {
        let mut r = TypeResponse::new();
        let name = "world";
        println!("greeting request from {}", name);
        r.set_types(format!("Hello {}", name));
        resp.finish(r)
    }

    fn query_filter_lock_start(&self, o: ServerHandlerContext, req: ServerRequestSingle<QueryFilterLockStartRequest>, resp: ServerResponseUnarySink<QueryFilterLockStartResponse>) -> grpc::Result<()> {
        todo!()
    }

    fn query_lock_event_by_index(&self, o: ServerHandlerContext, req: ServerRequestSingle<QueryLockEventByIndexRequest>, resp: ServerResponseUnarySink<LockEvent>) -> grpc::Result<()> {
        todo!()
    }

    fn query_appchain_index(&self, o: ServerHandlerContext, req: ServerRequestSingle<Empty>, resp: ServerResponseUnarySink<QueryAppchainIndexResponse>) -> grpc::Result<()> {
        todo!()
    }

    fn query_relay_index(&self, o: ServerHandlerContext, req: ServerRequestSingle<Empty>, resp: ServerResponseUnarySink<QueryRelayIndexResponse>) -> grpc::Result<()> {
        todo!()
    }
}

fn test_tls_acceptor() -> tls_api_native_tls::TlsAcceptor {
    let pkcs12 = include_bytes!("../foobar.com.p12");
    let builder = tls_api_native_tls::TlsAcceptorBuilder::from_pkcs12(pkcs12, "mypass").unwrap();
    builder.build().unwrap()
}

fn is_tls() -> bool {
    env::args().any(|a| a == "--tls")
}

fn main() {
    let tls = is_tls();

    let port = if !tls { 50051 } else { 50052 };

    let mut server = grpc::ServerBuilder::new();
    server.http.set_port(port);
    server.add_service(AppchainPluginServer::new_service_def(AppchainPluginImpl));
    //server.http.set_cpu_pool_threads(4);
    if tls {
        server.http.set_tls(test_tls_acceptor());
    }
    let _server = server.build().expect("server");

    println!(
        "AppchainPlugin server started on port {} {}",
        port,
        if tls { "with tls" } else { "without tls" }
    );

    loop {
        thread::park();
    }
}
