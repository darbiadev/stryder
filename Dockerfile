FROM debian:buster

COPY target/release/stryder stryder

EXPOSE 5000
ENTRYPOINT ./stryder

LABEL org.opencontainers.image.source = "https://github.com/darbiadev/stryder"
LABEL org.opencontainers.image.description = "A bot for Darbia's Discord Servers"
