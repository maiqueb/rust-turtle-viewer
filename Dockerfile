FROM docker.io/library/rust:1.74 as builder
WORKDIR /usr/src/rust-turtle-viewer
COPY . .
RUN cargo install --path .

FROM registry.access.redhat.com/ubi9/ubi-minimal
RUN microdnf install -y libpq-devel && microdnf clean all
COPY --from=builder /usr/local/cargo/bin/rust-turtle-manager /usr/local/bin/rust-turtle-viewer
ENTRYPOINT ["rust-turtle-viewer"]