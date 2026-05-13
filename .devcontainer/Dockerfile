FROM rust:1.95.0 AS builder

# --------------------------
# Install system dependencies
# --------------------------

RUN apt-get update \
	&& apt-get install -y --no-install-recommends zsh \
  git \
  sudo \
	&& rm -rf /var/lib/apt/lists/*

# -------------------------
# Finalize user setup
# -------------------------

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN groupadd --gid $USER_GID $USERNAME \
	&& useradd --uid $USER_UID --gid $USER_GID -m -s /usr/bin/zsh $USERNAME

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["game-of-life"]