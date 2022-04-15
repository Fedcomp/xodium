FROM rust:1.60

RUN apt-get update

ARG UID
ARG GID
RUN useradd vscode -u $UID -g $GID -m
USER vscode
WORKDIR /home/vscode/workspace

# Must be installed under user
RUN cargo install cargo-edit

CMD ["sleep", "infinity"]
