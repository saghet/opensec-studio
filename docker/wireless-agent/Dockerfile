FROM debian:bookworm-20250224-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    iw iproute2 pciutils usbutils ethtool python3 ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /opt/opensec
COPY services/opensec-wireless-agent/hw_manager.py /opt/opensec/hw_manager.py
CMD ["python3", "/opt/opensec/hw_manager.py"]
