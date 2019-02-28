To generate these test files:

1. Download the appropriate version of minikube

	```powershell
	iwr -OutFile ./minikube.0.30.0.exe 'https://storage.googleapis.com/minikube/releases/v0.30.0/minikube-windows-amd64'
	```

	```bash
	curl -Lo ./minikube.0.30.0 'https://storage.googleapis.com/minikube/releases/v0.30.0/minikube-linux-amd64'
	chmod +x ./minikube.0.30.0
	```

1. Create the cluster.

	```bash
	./minikube.0.30.0 --logtostderr -v9999 start --vm-driver virtualbox --profile minikube --bootstrapper kubeadm --kubernetes-version v1.12.3
	```

1. Run the tests in record mode

	```bash
	K8S_RECORD=1 cargo test --features 'test_v1_12'
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
		<tr><td>1.11.7</td><td>0.28.2</td></tr>
		<tr><td>1.12.6</td><td>0.30.0</td></tr>
		<tr><td>1.13.3</td><td>0.31.0</td></tr>
	</tbody>
</table>
