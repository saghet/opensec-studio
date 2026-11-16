FROM debian:bookworm-20250224-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates curl gpg iw wireless-tools \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get update && apt-get install -y --no-install-recommends kismet || true \
    && rm -rf /var/lib/apt/lists/*
EXPOSE 2501
CMD ["kismet", "--no-ncurses"]
