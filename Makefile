.PHONY: all buildmodule runhost

all: buildmodule runhost

buildmodule:
	rm -rf ./build/examples/hello-world/go-hello-world.wasm
	GOOS=wasip1 GOARCH=wasm go build -o ./build/examples/hello-world/go-hello-world.wasm ./examples/hello-world/main.go

runhost:
	RUST_BACKTRACE=full cargo run
