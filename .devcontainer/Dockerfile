FROM ubuntu:24.04

# Install basic dependencies
RUN apt-get update \
    && apt-get install -y \
    curl \
    build-essential \
    git \
    unzip \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install dprint
RUN curl -fsSL https://dprint.dev/install.sh | sh
ENV PATH="/root/.dprint/bin:${PATH}"

# Set bash prompt
RUN echo 'PS1="\[\033[01;31m\]\u@\h\[\033[00m\]:\[\033[01;34m\]\w\[\033[00m\]\$ "' >> /root/.bashrc

# Set working directory
WORKDIR /workspace