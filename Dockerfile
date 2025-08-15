FROM --platform=linux/amd64 ubuntu:24.04
RUN apt-get update && apt-get -y install grub2 xorriso mtools
