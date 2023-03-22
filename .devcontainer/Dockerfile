FROM ubuntu:22.04

WORKDIR /home/

COPY . .

RUN apt-get update
RUN apt-get install -y \
	curl \
	git \
	gnupg2 \
	jq \
	sudo \
	zsh \
	vim \
	build-essential \
	openssl


RUN useradd -m -s /bin/zsh github

# add to sudoers
RUN echo "github ALL=(ALL) NOPASSWD:ALL" | EDITOR="tee -a" visudo

USER github


RUN bash ./setup.sh