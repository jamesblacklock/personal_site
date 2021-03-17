frontend:
	(cd src/frontend && npm run build)
watch_frontend:
	(cd src/frontend && npm run watch)
build:
	cargo build
run:
	$(frontend)
	cargo run