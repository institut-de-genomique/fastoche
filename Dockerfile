FROM rust:1.66

# copy the git repo to the container
COPY . /gitlab/repo

# change directory for the remaining run commands
WORKDIR /gitlab/repo

# install the project module
RUN cargo build --tests
