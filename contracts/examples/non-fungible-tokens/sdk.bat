docker rm -f elrond-dev
docker run --name elrond-dev -v c:/Users/hhoareau/CLionProjects/elrond-wasm-rs/:/home/root/dev/ -ti f80hub/elrond-dev bash