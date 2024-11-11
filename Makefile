# Colors for better visibility in output
GREEN := \033[0;32m
BOLD := \033[1m
RESET := \033[0m

.PHONY: all
all:
	@echo "Phantasm Makefile for Developers"
	@echo ""
	@echo "$(BOLD)Usage$(RESET): make <target>"
	@echo ""
	@echo "$(BOLD)Available Target$(RESET)"
	@echo "  - generate_grpc_stubs: Generate gRPC stubs for clients"

.PHONY: generate_grpc_stubs
generate_grpc_stubs:
	@echo "Generating gRPC stubs..."
	@python -m grpc_tools.protoc -I./protos \
	  --python_out=./clients/python/stubs \
		--grpc_python_out=./clients/python/stubs \
		./protos/*.proto

	@echo "$(GREEN)gRPC stubs generated successfully!$(RESET)"
