To run all tests:

```sh
# Create all clusters

for v in {11..17}; do
    ./test.sh create-cluster "1.$v" || break
done


# Run all tests
for v in {11..17}; do
    ./test.sh run-tests "1.$v" || break
done


# Delete all clusters
for v in {11..17}; do
    ./test.sh delete-cluster "1.$v" || break
done
```
