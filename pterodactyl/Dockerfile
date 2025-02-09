FROM ghcr.io/quozul/picolimbo:master

RUN adduser -h /home/container -D container

RUN chown -R container: /usr/src/app/data

USER container
ENV USER=container HOME=/home/container
WORKDIR /home/container

COPY ./entrypoint.sh /entrypoint.sh
CMD [ "/bin/sh", "/entrypoint.sh" ]
