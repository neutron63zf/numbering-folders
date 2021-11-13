FROM xd009642/tarpaulin

WORKDIR /volume

COPY Cargo.toml Cargo.lock ./

RUN mkdir ./src && touch ./src/main.rs && echo "fn main(){}" > ./src/main.rs && mkdir /export

RUN cargo build && cargo build --release

# 最後だけ実行される
COPY src ./src

VOLUME [ "/export" ]
