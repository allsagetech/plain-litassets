use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let proto_root = manifest_dir.join("../proto/proto");

    let common_proto    = proto_root.join("cusf/common/v1/common.proto");
    let validator_proto = proto_root.join("cusf/mainchain/v1/validator.proto");
    let wallet_proto    = proto_root.join("cusf/mainchain/v1/wallet.proto");

    tonic_build::configure()
        .compile_protos(
            &[
                common_proto.to_str().unwrap(),
                validator_proto.to_str().unwrap(),
                wallet_proto.to_str().unwrap(),
            ],
            &[proto_root.to_str().unwrap()],
        )
        .unwrap();
}
