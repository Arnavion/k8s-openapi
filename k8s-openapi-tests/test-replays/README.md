To generate these test files:

1. Download the appropriate version of minikube

	```bash
	V='1.2.0'
	curl -Lo ~/bin/minikube-$V "https://storage.googleapis.com/minikube/releases/v$V/minikube-linux-amd64"
	chmod +x ~/bin/minikube-$V
	```

1. Create the cluster

	```bash
	minikube-$V --logtostderr -v9999 start --vm-driver kvm2 --profile minikube --bootstrapper kubeadm --kubernetes-version v1.15.1
	```

1. Run the tests in record mode

	```bash
	K8S_RECORD=1 cargo test --features 'test_v1_15'
	```

1. Delete the VM and its network

	```bash
	virsh destroy minikube
	virsh undefine minikube
	virsh net-undefine minikube-net
	virsh net-destroy minikube-net
	rm -rf ~/.kube ~/.minikube
	```

---

<table>
	<thead>
		<tr><th>k8s version</th><th>minikube version</th></tr>
	</thead>
	<tbody>
		<tr><td>1.8.15</td><td>0.28.0</td></tr>
		<tr><td>1.9.11</td><td>0.28.0</td></tr>
		<tr><td>1.10.13</td><td>0.28.2</td></tr>
		<tr><td>1.11.10</td><td>0.28.2</td></tr>
		<tr><td>1.12.10</td><td>0.30.0</td></tr>
		<tr><td>1.13.11</td><td>1.3.1</td></tr>
		<tr><td>1.14.7</td><td>1.3.1</td></tr>
		<tr><td>1.15.4</td><td>1.3.1</td></tr>
		<tr><td>1.16.1</td><td>1.3.1</td></tr>
	</tbody>
</table>

---

# Misc

- If `minikube start` fails with:

	>`Unable to start VM: create: Error creating machine: Error in driver during machine creation: ensuring active networks: starting network default: virError(Code=38, Domain=0, Message='error creating bridge interface virbr0: File exists')`

	Destroy the VM and its network as above, then reset the `default` libvirt network with:

	```bash
	virsh net-undefine default
	virsh net-destroy default
	virsh net-define /usr/share/libvirt/networks/default.xml
	virsh net-start default
	```
