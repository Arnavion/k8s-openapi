To run all tests:

```sh
# Create all node images
KIND_IMAGES="$(realpath ~/kind-images)"
mkdir -p "$KIND_IMAGES"
./test.sh all create-node-image "$KIND_IMAGES"

# Create all clusters
./test.sh all create-cluster "$KIND_IMAGES"

# Run all tests in record mode
K8S_RECORD=1 ./test.sh all run-tests

# Delete all clusters
./test.sh all delete-cluster

# Run all tests in replay mode
./test.sh all run-tests
```
