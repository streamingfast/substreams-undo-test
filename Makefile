ENDPOINT ?= mainnet.eth.streamingfast.io:443
START_BLOCK ?= 0
STOP_BLOCK ?= +100

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: stream
stream: build
	substreams gui -e $(ENDPOINT) substreams.yaml graph_out -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: package
package: build
	substreams package substreams.yaml
