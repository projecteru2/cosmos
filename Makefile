grpc:
	curl https://raw.githubusercontent.com/projecteru2/core/master/rpc/gen/core.proto -O
	protoc --rust_out=src/orchestrator/eru/ --rust-grpc_out=src/orchestrator/eru/ core.proto
	gsed -i -e 's/\basync\b/r#async/g' src/orchestrator/eru/core.rs
