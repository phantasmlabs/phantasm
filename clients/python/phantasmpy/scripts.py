import os


def generate_grpc_stubs():
    proto_dir = os.path.abspath((os.path.join("..", "..", "protos")))
    proto_file = os.path.join(proto_dir, "receiver.proto")

    generated_dir = os.path.abspath(os.path.join("gen"))
    os.makedirs(generated_dir, exist_ok=True)

    os.system(
        " ".join(
            [
                "python -m grpc_tools.protoc",
                f"-I{proto_dir}",
                f"--python_out={generated_dir}",
                f"--grpc_python_out={generated_dir}",
                proto_file,
            ]
        )
    )

    print(f"Generated gRPC stubs in {generated_dir}")
