NAME = Gomoku

GAME_RS_PKG = game-rs/pkg
GAME_RS_PKG_DEPS = $(shell find game-rs/ -type f ! -path 'game-rs/pkg/*' ! -path 'game-rs/target/*')

FRONTEND = build
FRONTEND_DEPS = $(GAME_RS_PKG) $(shell find src/ static/ package* *config* -type f)


# Inspired from: https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
help:
	@grep -E '^([a-zA-Z_-]+:.*)?## .*' $(MAKEFILE_LIST) | \
	awk 'BEGIN {FS = ":.*## |^## "}; {\
		if ($$1 == "") {\
			printf "\n\x1b[35;1m%s\x1b[0m\n", $$2\
		} else {\
			printf "    \x1b[36;1m%-30s\x1b[0m %s\n", $$1, $$2\
		}\
	}'
	@echo
.PHONY: help



## Development

dev: ## Run the development server
	npm i
	cd game-rs && cargo watch -i .gitignore -i "pkg/*" -s "cd .. && make $(GAME_RS_PKG)" & \
	npm run dev -- --clearScreen=false & \
	wait
.PHONY: dev


all: $(NAME) ## Build the project
.PHONY: all

$(GAME_RS_PKG): $(GAME_RS_PKG_DEPS)
	cd game-rs && wasm-pack build --target web
	mkdir -p ./src/lib/ai/game-rs/
	cp -r ./game-rs/pkg/* ./src/lib/ai/game-rs/

$(FRONTEND): $(FRONTEND_DEPS)
	npm run build
	echo gomoku.mathias.ninja > ./$(FRONTEND)/CNAME

$(NAME): $(GAME_RS_PKG) $(FRONTEND)
	echo -e "#!/usr/bin/env bash\nnpm run preview" > $(NAME)
	chmod +x $(NAME)


clean: ## Remove the build artifacts
	rm -rf $(GAME_RS_PKG) .svelte-kit
	cd game-rs && cargo clean

fclean: clean ## Remove the build artifacts and binaries
	rm -rf $(FRONTEND) $(NAME)

re: fclean all ## Rebuild the project

.PHONY: clean fclean re



## Testing

test: ## Run unit tests
	# cd game-rs && wasm-pack test --node
	# cd game-rs && wasm-pack test --headless --firefox
	cd game-rs && cargo test

# make bench BENCH_FLAGS=--save-baseline=main
# make bench BENCH_FLAGS=--baseline=main
bench: ## Run the benchmarks
	cd game-rs && cargo bench --bench profiling -- --callgrind-args="--cache-sim=no" $(BENCH_FLAGS)

clippy: ## Run the linter
	cd game-rs && cargo clippy --all-targets --all-features

.PHONY: test bench clippy



## Training

# train: $(GAME_RS_PKG) ## Train the model
# 	node --experimental-strip-types ./training/main.ts

# plot: $(GAME_RS_PKG) ## Plot the training
# 	node --experimental-strip-types ./training/plot.ts

winrate: $(GAME_RS_PKG) ## Compute winrate
	node --experimental-strip-types ./training/winrate.ts

.PHONY: train plot winrate



## Debug

threat_debug: ## Show threat lookup table
	cd game-rs && cargo run --bin threat_debug | less -r

.PHONY: threat_debug


# profiling-clean:
# 	cd game-rs && rm -rf callgrind.out.* dhat.out.*

# profiling-build:
# 	cd game-rs && cargo build --bin profiling

# # https://nnethercote.github.io/perf-book/profiling.html
# # https://www.codeofview.com/fix-rs/2017/01/24/how-to-optimize-rust-programs-on-linux/
# profiling-callgrind: profiling-clean profiling-build
# 	cd game-rs && valgrind --tool=callgrind --dump-instr=yes --collect-jumps=yes --simulate-cache=yes ./target/debug/profiling
# 	cd game-rs && kcachegrind callgrind.out.*

# # https://valgrind.org/docs/manual/dh-manual.html#dh-manual.running-DHAT
# profiling-dhat: profiling-clean profiling-build
# 	cd game-rs && valgrind --tool=dhat ./target/debug/profiling

# .PHONY: profiling-clean profiling-build profiling-callgrind profiling-dhat
