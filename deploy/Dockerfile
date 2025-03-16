FROM ubuntu:latest

WORKDIR /code

RUN apt-get update

RUN apt-get install -y vim screen curl sudo unzip fuse3 net-tools

COPY ./* .

ENV PORT=80
ENV IP=0.0.0.0