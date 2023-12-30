.PHONY: run-registry
run-registry:
	docker run -d -p 5000:5000 --restart always --name registry registry:2

.PHONY: ls-registry
ls-registry:
	reg ls -f localhost:5000

.PHONY: check-manifest-registry
check-manifest-registry:
	curl -H 'Accept: application/vnd.oci.image.manifest.v1+json' http://localhost:5000/v2/models/llama2/manifests/sha256:${MANIFEST} | jq

.PHONY: push_raw
push_raw:
	cargo run --release -- push localhost:5000/models/llama2 ~/.models/llama-2-7b-chat.ggmlv3.q2_K.bin ~/.models/llama-2-7b-chat.ggmlv3.q2_K.tokenizer

.PHONY: run_inference
run_inference:
	cargo run --release -- run-raw /home/ereslibre/.models/llama-2-7b-chat.ggmlv3.q2_K.bin \
		/home/ereslibre/.models/llama-2-7b-chat.ggmlv3.q2_K.tokenizer

.PHONY: inspect-registry
inspect-registry:
	echo "Catalog:"
	curl http://localhost:5000/v2/_catalog | jq
	echo "Blob list:"
	curl -H 'Accept: application/vnd.oci.image.manifest.v1+json' http://localhost:5000/v2/models/llama2/manifests/latest | jq

.PHONY: clean
clean:
	docker rm -f registry

.PHONY: restart
restart: clean run-registry
