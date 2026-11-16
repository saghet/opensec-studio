FROM alpine:3.21.3
RUN apk add --no-cache tshark python3 && adduser -D -u 10002 opensec-pcap
USER opensec-pcap
WORKDIR /opt/opensec
COPY services/opensec-pcap-analyzer/app.py /opt/opensec/app.py
EXPOSE 8090
ENTRYPOINT ["python3", "/opt/opensec/app.py"]
