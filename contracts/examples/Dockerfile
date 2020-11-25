#Déployer le testnet elrond sur serveur linuex
#directement inspiré de https://github.com/erdDEVcode/erdnet/blob/master/Dockerfile
FROM ubuntu:groovy
#fabrication de l'image pour X86: docker build -t f80hub/elrond-dev . & docker push f80hub/elrond-dev:latest
#déploiement : docker rm -f elrond-dev && docker pull f80hub/elrond-dev:latest
#ligne de commande pour le portable : docker rm -f elrond-dev && docker run --name elrond-dev -v c:/Users/hhoar/PycharmProjects/elrond-wasm-rs:/home/erd/dev/ -ti f80hub/elrond-dev bash
#ligne de commande pour le fixe : docker rm -f elrond-dev && docker run --name elrond-dev -v c:/Users/hhoareau/ClionProjects/elrond-wasm-rs:/home/root/dev/ -ti f80hub/elrond-dev bash

RUN apt update
RUN apt install -y wget python3 python3-venv sudo build-essential nano net-tools python3-pip
RUN adduser --home /home/erd --shell /bin/bash --disabled-password erd

RUN echo "erd ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers
RUN export PATH="$HOME/.local/bin:$PATH"

RUN pip3 install wheel
RUN pip3 install --upgrade --no-cache-dir erdpy

WORKDIR /home/root

RUN echo "cd /home/root/dev/contracts/examples/factorial\nerdpy contract build\ncd .." >> /home/root/work.sh

CMD ["bash"]