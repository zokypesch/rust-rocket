FROM liuchong/rustup

ENV ROCKET_ADDRESS=0.0.0.0

ENV ROCKET_PORT=8080

ENV DATABASE_URL=mysql://user:pass@127.0.0.1/heroes

# ADD ./app/src /app
RUN mkdir -p /app
WORKDIR /app

# COPY ./src /app

# COPY ./Cargo.toml /app/Cargo.toml
# COPY ./diesel.toml /app/diesel.toml
# COPY ./app/Cargo.toml /app

# RUN ls
COPY ./app /app
# COPY . /app

# RUN cat Cargo.toml

RUN rustup default nightly

RUN cargo build

CMD ["cargo", "run"]