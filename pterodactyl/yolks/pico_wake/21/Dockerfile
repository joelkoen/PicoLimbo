FROM ghcr.io/quozul/picolimbo:master

RUN apk add --no-cache openjdk21-jre lsof curl ca-certificates openssl git tar sqlite fontconfig freetype tzdata iproute2 libstdc++ && \
    adduser -h /home/container -D container

USER container
ENV USER=container HOME=/home/container
WORKDIR /home/container

COPY ./entrypoint.sh /entrypoint.sh
CMD [ "/bin/sh", "/entrypoint.sh" ]
