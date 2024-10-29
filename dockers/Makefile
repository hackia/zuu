COLOR_RESET=\033[0m
COLOR_GREEN=\033[32m
COLOR_BLUE=\033[34m
COLOR_RED=\033[31m
COLOR_BOLD=\033[1m

USERNAME ?= otechdo
REPO ?= zuu

LANGUAGES := $(shell ls -d ./*/ | xargs -n1 basename)

.PHONY: check-tools
check-tools:
	@which docker > /dev/null || (echo -e "$(COLOR_RED)Docker is not installed. Please install Docker.$(COLOR_RESET)"; exit 1)
	@docker buildx version > /dev/null || (echo -e "$(COLOR_RED)Docker Buildx is not installed. Please install Docker Buildx.$(COLOR_RESET)"; exit 1)
	@which git > /dev/null || (echo -e "$(COLOR_RED)Git is not installed. Please install Git.$(COLOR_RESET)"; exit 1)
	@echo -e "$(COLOR_GREEN)All required tools are installed.$(COLOR_RESET)"

.PHONY: push
push: check-tools $(addprefix push-, $(LANGUAGES))

push-%: build-%
	@echo -e "$(COLOR_GREEN)    Pushing$(COLOR_RESET) $(COLOR_BOLD)$(USERNAME)/$(REPO):$*$(COLOR_RESET) to Docker Hub..."
	@docker push $(USERNAME)/$(REPO):$*
	@echo -e "\r$(COLOR_GREEN)    Finished pushing$(COLOR_RESET) $(COLOR_BOLD)$(USERNAME)/$(REPO):$*$(COLOR_RESET)"

.PHONY: build
build: check-tools $(addprefix build-, $(LANGUAGES))

build-%:
	@echo -e "$(COLOR_BLUE)    Building$(COLOR_RESET) $(COLOR_BOLD)$(USERNAME)/$(REPO):$*$(COLOR_RESET) image..."
	@docker buildx build -t $(USERNAME)/$(REPO):$* ./$*
	@echo -e "\r$(COLOR_GREEN)    Finished building$(COLOR_RESET) $(COLOR_BOLD)$(USERNAME)/$(REPO):$*$(COLOR_RESET)"

.PHONY: clean
clean:
	@echo -e "$(COLOR_BLUE)    Cleaning$(COLOR_RESET) Docker images..."
	@docker rmi $(addprefix $(USERNAME)/$(REPO):, $(LANGUAGES)) > /dev/null 2>&1
	@echo -e "$(COLOR_GREEN)    Finished cleaning$(COLOR_RESET)"
.PHONY: login
login:
	@echo -e "$(COLOR_GREEN)    Logging into Docker Hub as $(COLOR_BOLD)$(USERNAME)$(COLOR_RESET)..."
	@docker login --username $(USERNAME)

.PHONY: logout
logout:
	@echo -e "$(COLOR_GREEN)    Logging out from Docker Hub...$(COLOR_RESET)"
	@docker logout

.PHONY: pull
pull: check-tools $(addprefix pull-, $(LANGUAGES))

pull-%:
	@echo -e "$(COLOR_GREEN)    Pulling$(COLOR_RESET) $(COLOR_BOLD)$(USERNAME)/$(REPO):$*$(COLOR_RESET) from Docker Hub..."
	@docker pull $(USERNAME)/$(REPO):$*
	@echo -e "\r$(COLOR_GREEN)    Finished pulling$(COLOR_RESET) $(COLOR_BOLD)$(USERNAME)/$(REPO):$*$(COLOR_RESET)"

.DEFAULT_GOAL := push
