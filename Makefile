NAMESPACE=gwihlidal
NAME=cloudstore
TAG=latest
CONTAINER=$(NAMESPACE)/$(NAME):$(TAG)

.PHONY: build
build:
	docker build -t $(CONTAINER) .

.PHONY: push
push: build
	docker push $(CONTAINER)

.PHONY: run
run:
	docker run --rm -p 8080:8080 -it $(CONTAINER)