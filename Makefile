NAMESPACE=gwihlidal
NAME=cloudstore
TAG=latest
CONTAINER=$(NAMESPACE)/$(NAME):$(TAG)

.PHONY: build
build:
	docker build -t $(CONTAINER) .

.PHONY: push
push: build test
	docker push $(CONTAINER)