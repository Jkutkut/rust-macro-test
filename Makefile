# ****** Operating System ******
OS = $(shell uname -s)
ifeq ($(OS),Linux)
	DIR = $(shell pwd)
endif
ifeq ($(OS),Darwin)
	DIR = ${PWD}
endif
REPO = $(shell echo ${DIR} | sed 's/.*\///' | tr '[:upper:]' '[:lower:]')

# ****** Rust Constants ******
CARGO = /root/.cargo/bin/cargo
CODE_VOLUME = -v ${DIR}:/${REPO}
CARGO_REGISTRY = -v cargo_registy:/root/.cargo/registry

# ****** Docker Constants ******
DOCKER_RUN = docker run --rm
DOCKER_RUN_IT = ${DOCKER_RUN} -it
DOCKER_RUN_D = ${DOCKER_RUN} -d

RUN_ATTRS = ${CODE_VOLUME} ${CARGO_REGISTRY} -w /${REPO}

# ****** Project ******
NAME = $(shell grep -m 1 name Cargo.toml | cut -d '"' -f 2)
VERSION = "v$(shell grep -m 1 version Cargo.toml | cut -d '"' -f 2)"

DEV_CONTAINER_NAME = ${NAME}_dev
DEV_IMAGE_NAME = jkutkut/docker4rust

# ****** Docker Containers ******
stop:
	docker stop ${DEV_CONTAINER_NAME}

DEV_ATTRS = --name ${DEV_CONTAINER_NAME} \
			${RUN_ATTRS}

test:
	${DOCKER_RUN_IT} ${DEV_ATTRS} --entrypoint ${CARGO} ${DEV_IMAGE_NAME} test

test_watch:
	${DOCKER_RUN_IT} ${DEV_ATTRS} --entrypoint ${CARGO} ${DEV_IMAGE_NAME} watch --clear test

connect_dev:
	docker exec -it ${DEV_CONTAINER_NAME} sh

terminal_dev:
	${DOCKER_RUN_IT} ${DEV_ATTRS} ${DEV_IMAGE_NAME}

build:
	${DOCKER_RUN_IT} ${RUN_ATTRS} --entrypoint ${CARGO} ${DEV_IMAGE_NAME} build

build_release:
	${DOCKER_RUN_IT} ${RUN_ATTRS} --entrypoint ${CARGO} ${DEV_IMAGE_NAME} build --release
