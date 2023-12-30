package main

import (
	demo "github.com/saschagrunert/demo"
	cli "github.com/urfave/cli/v2"
)

func main() {
	d := demo.New()

	d.Setup(setup)
	d.Cleanup(cleanup)

	d.Add(ociRegistryAsModelRegistry(), "oci-registry-as-model-registry", "Example showing the oci-registry-as-model-registry project")

	d.Run()
}

func setup(ctx *cli.Context) error {
	// Ensure can be used for easy sequential command execution
	return demo.Ensure(
		"echo 'Doing first setup…'",
		"echo 'Doing second setup…'",
		"echo 'Doing third setup…'",
	)
}

func cleanup(ctx *cli.Context) error {
	demo.Ensure("rm -rf ~/.local/share/oci-registry-as-model-registry")
	demo.Ensure("docker rm -f registry")

	return nil
}

func ociRegistryAsModelRegistry() *demo.Run {
	r := demo.NewRun(
		"oci-registry-as-model-registry",
	)

	r.Step(demo.S(
		"List containers",
	), demo.S(
		"docker ps -a",
	))

	r.Step(demo.S(
		"Start a registry",
	), demo.S(
		"docker run -d -p 5000:5000 --restart always --name registry registry:2",
	))

	r.Step(demo.S(
		"Push a container image to the registry",
	), demo.S(
		`docker pull library/ubuntu:latest && \
       docker tag docker.io/library/ubuntu:latest localhost:5000/library/ubuntu:latest && \
       docker push localhost:5000/library/ubuntu:latest`,
	))

	r.Step(demo.S(
		"List container image and artifacts in the registry",
	), demo.S(
		"reg ls -f http://localhost:5000 2> /dev/null",
	))

	r.Step(demo.S(
		"List models on the local store",
	), demo.S(
		"oci-registry-as-model-registry list",
	))

	r.Step(demo.S(
		"Push model without any LoRA adapter",
	), demo.S(
		`oci-registry-as-model-registry push localhost:5000/models/llama-2:1.0.0-base \
       base-model \
       1.0.0 \
       Llama-2 \
       ~/.models/llama-2-q4_0-with-adapter/model.bin \
       ~/.models/llama-2-q4_0-with-adapter/tokenizer.json`,
	))

	r.Step(demo.S(
		"List container image and artifacts in the registry",
	), demo.S(
		"reg ls -f http://localhost:5000 2> /dev/null",
	))

	r.Step(demo.S(
		"List models on the local store",
	), demo.S(
		"oci-registry-as-model-registry list",
	))

	r.Step(demo.S(
		"Pull model",
	), demo.S(
		"oci-registry-as-model-registry pull localhost:5000/models/llama-2:1.0.0-base",
	))

	r.Step(demo.S(
		"List models on the local store",
	), demo.S(
		"oci-registry-as-model-registry list",
	))

	r.Step(demo.S(
		"Run inference on base model",
	), demo.S(
		`cat <<EOF | oci-registry-as-model-registry run localhost:5000/models/llama-2:1.0.0-base
Be concise with your answer. What is the capital of France?
EOF`,
	))

	r.Step(demo.S(
		"Push model with one LoRA adapter -- more are also possible!",
	), demo.S(
		`oci-registry-as-model-registry push localhost:5000/models/llama-2:1.0.0-vigogne \
       vigogne-model \
       1.0.0 \
       Llama-2 \
       ~/.models/llama-2-q4_0-with-adapter/model.bin \
       ~/.models/llama-2-q4_0-with-adapter/tokenizer.json \
       ~/.models/llama-2-q4_0-with-adapter/adapter.bin`,
	))

	r.Step(demo.S(
		"Run inference on fine-tuned model",
	), demo.S(
		`cat <<EOF | oci-registry-as-model-registry run localhost:5000/models/llama-2:1.0.0-vigogne
Be concise with your answer. What is the capital of France?
EOF`,
	))

	r.Step(demo.S(
		"List models on the local store",
	), demo.S(
		"oci-registry-as-model-registry list",
	))

	r.Step(demo.S(
		"Describe model at remote registry",
	), demo.S(
		"oci-registry-as-model-registry describe localhost:5000/models/llama-2:1.0.0-base",
	))

	r.Step(demo.S(
		"Model local cache",
	), demo.S(
		"tree ~/.local/share/oci-registry-as-model-registry",
	))

	return r
}
