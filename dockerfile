FROM rust:1.91.0-alpine

# workディレクトリの作成
WORKDIR /work
 
CMD ["/bin/sh"]

# sudo docker build -t rust_study:1.68 .
# sudo docker run --rm -it  -v "$(pwd)/src:/work" rust_study:1.68
# sudo docker container exec -it <コンテナ名> bash
# sudo docker container ls