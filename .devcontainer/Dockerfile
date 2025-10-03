FROM ubuntu:24.04

ARG USERNAME=ubuntu
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Create the user
RUN apt-get update \
    && apt-get install -y sudo \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

# Install basic dependencies (as root)
RUN apt-get update \
    && apt-get install -y \
    curl \
    build-essential \
    git \
    unzip \
    && rm -rf /var/lib/apt/lists/*

# Switch to the user
USER $USERNAME

# # Install Rust 
# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# ENV PATH="/home/${USERNAME}/.cargo/bin:${PATH}"

# Install dprint
RUN curl -fsSL https://dprint.dev/install.sh | sh
ENV PATH="/home/${USERNAME}/.dprint/bin:${PATH}"

# Configure colored prompt
RUN echo 'export PS1="\[\e[32m\]\u@\h\[\e[m\]:\[\e[34m\]\w\[\e[m\]\$ "' >> /home/${USERNAME}/.bashrc

# Set working directory
WORKDIR /workspaces
USER root
RUN chown -R $USERNAME:$USERNAME /workspaces
USER $USERNAME