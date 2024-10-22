# If you encounter this error:
# File ".../receiver_pb2_grpc.py", line 7, in <module>
# import receiver_pb2 as receiver__pb2
# ModuleNotFoundError: No module named 'receiver_pb2'
#
# You need to change the import statement in receiver_pb2_grpc.py to:
# from . import receiver_pb2 as receiver__pb2
